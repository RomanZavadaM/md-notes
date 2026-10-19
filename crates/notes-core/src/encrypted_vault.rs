//! Encrypted vault setup (ADR-0007): the open `.mdnotes/encryption.json`,
//! creation, opening and recipient management.
//!
//! This module never writes a private key anywhere. Callers receive the key
//! as a string once (to keep it in the OS secret store and to show a backup
//! copy to the user) and pass it back when opening the vault.

use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Arc;

use age::x25519::{Identity, Recipient};
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};

use crate::encrypted::{EncryptedProvider, NameLayout};
use crate::error::{io_err, Error, Result};
use crate::storage::LocalFsProvider;
use crate::vault::{write_atomic, Vault, SERVICE_DIR};

/// File name inside `.mdnotes/`.
pub const ENCRYPTION_FILE: &str = "encryption.json";
/// Only supported format.
pub const ENCRYPTION_FORMAT: &str = "age";
/// Only supported version of the config.
pub const ENCRYPTION_VERSION: u32 = 1;

/// Entries that may already exist in a folder turned into an encrypted vault.
const ALLOWED_EXISTING: &[&str] = &[".git", ".gitignore", ".gitattributes", SERVICE_DIR];

/// `.mdnotes/encryption.json`: open settings of an encrypted vault. It holds
/// only public keys.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EncryptionConfig {
    pub format: String,
    pub version: u32,
    /// Identifies the private key of this vault in the OS secret store.
    pub vault_id: String,
    /// Public keys (`age1...`) every file is encrypted for.
    pub recipients: Vec<String>,
    /// How files are named on disk. Vaults created before random names were
    /// introduced have no such field and keep `named`.
    #[serde(default)]
    pub names: NameLayout,
}

impl EncryptionConfig {
    fn path(root: &Path) -> PathBuf {
        root.join(SERVICE_DIR).join(ENCRYPTION_FILE)
    }

    /// Reads and validates the config; `None` when the vault is not encrypted.
    pub fn load(root: &Path) -> Result<Option<Self>> {
        let path = Self::path(root);
        if !path.is_file() {
            return Ok(None);
        }
        let raw = fs::read_to_string(&path).map_err(io_err(&path))?;
        let config: Self = serde_json::from_str(&raw)
            .map_err(|err| Error::Config(format!("{ENCRYPTION_FILE}: {err}")))?;
        config.validate()?;
        Ok(Some(config))
    }

    /// Checks the format, the version, the vault id and every public key.
    pub fn validate(&self) -> Result<()> {
        if self.format != ENCRYPTION_FORMAT || self.version != ENCRYPTION_VERSION {
            return Err(Error::Config(format!(
                "unsupported encryption format {} v{}",
                self.format, self.version
            )));
        }
        if self.vault_id.trim().is_empty() {
            return Err(Error::Config("encryption vaultId is empty".into()));
        }
        self.parse_recipients().map(|_| ())
    }

    /// Public keys as age recipients.
    pub fn parse_recipients(&self) -> Result<Vec<Recipient>> {
        if self.recipients.is_empty() {
            return Err(Error::Config("encryption has no recipients".into()));
        }
        self.recipients
            .iter()
            .map(|text| parse_recipient(text))
            .collect()
    }

    fn save(&self, root: &Path) -> Result<()> {
        let dir = root.join(SERVICE_DIR);
        fs::create_dir_all(&dir).map_err(io_err(&dir))?;
        let json = serde_json::to_string_pretty(self).expect("config serializes") + "\n";
        write_atomic(&Self::path(root), json.as_bytes())
    }
}

/// Result of [`create_encrypted_vault`].
pub struct CreatedVault {
    pub vault: Vault,
    pub vault_id: String,
    /// Private key (`AGE-SECRET-KEY-...`). Shown once as a backup, then kept
    /// only in the OS secret store.
    pub secret_key: String,
    /// Public key of the generated private key.
    pub public_key: String,
}

fn parse_recipient(text: &str) -> Result<Recipient> {
    Recipient::from_str(text.trim())
        .map_err(|err| Error::Config(format!("invalid recipient key: {err}")))
}

/// Parses a private key string.
pub fn parse_identity(secret_key: &str) -> Result<Identity> {
    Identity::from_str(secret_key.trim())
        .map_err(|err| Error::Crypto(format!("invalid private key: {err}")))
}

/// Generates a key pair: `(private key, public key)`.
pub fn generate_key_pair() -> (String, String) {
    let identity = Identity::generate();
    (
        identity.to_string().expose_secret().to_string(),
        identity.to_public().to_string(),
    )
}

/// Public key of a private key string.
pub fn public_key_of(secret_key: &str) -> Result<String> {
    Ok(parse_identity(secret_key)?.to_public().to_string())
}

/// Whether `secret_key` belongs to the vault described by `config`.
pub fn key_matches(config: &EncryptionConfig, secret_key: &str) -> Result<bool> {
    let public = public_key_of(secret_key)?;
    Ok(config.recipients.iter().any(|r| r.trim() == public))
}

fn provider_for(
    root: &Path,
    recipients: Vec<Recipient>,
    identity: Identity,
    layout: NameLayout,
) -> Result<EncryptedProvider> {
    Ok(EncryptedProvider::new(
        Arc::new(LocalFsProvider::new(root)?),
        recipients,
        vec![identity],
    )?
    .with_layout(layout))
}

fn open_with_identity(root: &Path, config: &EncryptionConfig, identity: Identity) -> Result<Vault> {
    let own = identity.to_public().to_string();
    if !config.recipients.iter().any(|r| r.trim() == own) {
        return Err(Error::Crypto(
            "the key does not belong to this vault".into(),
        ));
    }
    let provider = provider_for(root, config.parse_recipients()?, identity, config.names)?;
    Vault::open_with_provider(root, Arc::new(provider))
}

fn ensure_folder_is_free(root: &Path) -> Result<()> {
    if !root.is_dir() {
        return Err(Error::NotFound(root.display().to_string()));
    }
    for entry in fs::read_dir(root).map_err(io_err(root))? {
        let entry = entry.map_err(io_err(root))?;
        let name = entry.file_name().to_string_lossy().into_owned();
        if !ALLOWED_EXISTING.contains(&name.as_str()) {
            return Err(Error::AlreadyExists(format!(
                "{} (the folder for an encrypted vault must be empty)",
                root.join(name).display()
            )));
        }
    }
    Ok(())
}

/// Creates a new encrypted vault in an empty folder. A new key pair is
/// generated; `extra_recipients` (for example a recovery key) are added.
pub fn create_encrypted_vault(
    root: &Path,
    name: Option<String>,
    extra_recipients: &[String],
) -> Result<CreatedVault> {
    if EncryptionConfig::load(root)?.is_some() {
        return Err(Error::AlreadyExists(format!(
            "{} is already an encrypted vault",
            root.display()
        )));
    }
    ensure_folder_is_free(root)?;

    let identity = Identity::generate();
    let public_key = identity.to_public().to_string();
    let mut recipients = vec![public_key.clone()];
    for extra in extra_recipients {
        let extra = parse_recipient(extra)?.to_string();
        if !recipients.contains(&extra) {
            recipients.push(extra);
        }
    }

    Vault::init(root, name)?;
    let config = EncryptionConfig {
        format: ENCRYPTION_FORMAT.to_string(),
        version: ENCRYPTION_VERSION,
        vault_id: ulid::Ulid::new().to_string(),
        recipients,
        names: NameLayout::Random,
    };
    config.save(root)?;

    let secret_key = identity.to_string().expose_secret().to_string();
    let vault = open_with_identity(root, &config, identity)?;
    Ok(CreatedVault {
        vault,
        vault_id: config.vault_id,
        secret_key,
        public_key,
    })
}

/// Opens an existing encrypted vault with its private key.
pub fn open_encrypted_vault(root: &Path, secret_key: &str) -> Result<Vault> {
    let config = EncryptionConfig::load(root)?
        .ok_or_else(|| Error::NotFound(format!("{SERVICE_DIR}/{ENCRYPTION_FILE}")))?;
    open_with_identity(root, &config, parse_identity(secret_key)?)
}

/// Adds a recipient (for example a recovery key) and re-encrypts every stored
/// file for the new recipient list. Returns the number of files rewritten.
pub fn add_recipient(root: &Path, secret_key: &str, recipient: &str) -> Result<u32> {
    let mut config = EncryptionConfig::load(root)?
        .ok_or_else(|| Error::NotFound(format!("{SERVICE_DIR}/{ENCRYPTION_FILE}")))?;
    let identity = parse_identity(secret_key)?;
    if !key_matches(&config, secret_key)? {
        return Err(Error::Crypto(
            "the key does not belong to this vault".into(),
        ));
    }
    let new = parse_recipient(recipient)?.to_string();
    if config.recipients.iter().any(|r| r.trim() == new) {
        return Ok(0);
    }
    config.recipients.push(new);
    let provider = provider_for(root, config.parse_recipients()?, identity, config.names)?;
    let rewritten = provider.reencrypt_all()?;
    config.save(root)?;
    Ok(rewritten)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dir() -> tempfile::TempDir {
        tempfile::tempdir().unwrap()
    }

    #[test]
    fn create_then_reopen_round_trips_and_keeps_secrets_out_of_files() {
        let tmp = dir();
        let created = create_encrypted_vault(tmp.path(), Some("Private".into()), &[]).unwrap();
        created
            .vault
            .write_note("a.md", "# Hidden title\n")
            .unwrap();

        assert!(created.secret_key.starts_with("AGE-SECRET-KEY-"));
        assert!(created.public_key.starts_with("age1"));
        // Random file names: neither the note name nor its title is on disk.
        assert!(!tmp.path().join("a.md.age").exists());
        assert!(!tmp.path().join("a.md").exists());
        let stored: Vec<_> = fs::read_dir(tmp.path())
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
            .filter(|n| n.ends_with(".age"))
            .collect();
        assert_eq!(stored.len(), 1);
        assert_eq!(stored[0].len(), 16 + 4);

        let config_text =
            fs::read_to_string(tmp.path().join(SERVICE_DIR).join(ENCRYPTION_FILE)).unwrap();
        assert!(config_text.contains(&created.public_key));
        assert!(!config_text.contains("AGE-SECRET-KEY-"));

        let reopened = open_encrypted_vault(tmp.path(), &created.secret_key).unwrap();
        assert_eq!(reopened.read_note("a.md").unwrap().title, "Hidden title");
        assert_eq!(reopened.note_paths().unwrap(), vec!["a.md"]);
    }

    #[test]
    fn wrong_or_malformed_key_is_rejected() {
        let tmp = dir();
        create_encrypted_vault(tmp.path(), None, &[]).unwrap();

        let (stranger, _) = generate_key_pair();
        assert!(matches!(
            open_encrypted_vault(tmp.path(), &stranger),
            Err(Error::Crypto(_))
        ));
        assert!(matches!(
            open_encrypted_vault(tmp.path(), "not a key"),
            Err(Error::Crypto(_))
        ));
        let plain = dir();
        assert!(matches!(
            open_encrypted_vault(plain.path(), &stranger),
            Err(Error::NotFound(_))
        ));
    }

    #[test]
    fn recovery_recipient_can_open_the_vault() {
        let tmp = dir();
        let (recovery_secret, recovery_public) = generate_key_pair();
        let created =
            create_encrypted_vault(tmp.path(), None, std::slice::from_ref(&recovery_public))
                .unwrap();
        created.vault.write_note("n.md", "# N\n").unwrap();

        let config = EncryptionConfig::load(tmp.path()).unwrap().unwrap();
        assert_eq!(config.recipients.len(), 2);
        assert!(key_matches(&config, &recovery_secret).unwrap());
        let vault = open_encrypted_vault(tmp.path(), &recovery_secret).unwrap();
        assert_eq!(vault.read_note("n.md").unwrap().title, "N");
    }

    #[test]
    fn refuses_non_empty_folders_and_bad_recipients() {
        let tmp = dir();
        fs::write(tmp.path().join("existing.md"), "plain").unwrap();
        assert!(matches!(
            create_encrypted_vault(tmp.path(), None, &[]),
            Err(Error::AlreadyExists(_))
        ));

        let empty = dir();
        assert!(matches!(
            create_encrypted_vault(empty.path(), None, &["nonsense".to_string()]),
            Err(Error::Config(_))
        ));
        assert!(EncryptionConfig::load(empty.path()).unwrap().is_none());

        create_encrypted_vault(empty.path(), None, &[]).unwrap();
        assert!(matches!(
            create_encrypted_vault(empty.path(), None, &[]),
            Err(Error::AlreadyExists(_))
        ));
    }

    #[test]
    fn config_validation_rejects_unsupported_files() {
        let (_, public) = generate_key_pair();
        let good = EncryptionConfig {
            format: ENCRYPTION_FORMAT.into(),
            version: ENCRYPTION_VERSION,
            vault_id: "01J".into(),
            recipients: vec![public],
            names: NameLayout::Random,
        };
        assert!(good.validate().is_ok());

        let mut bad = good.clone();
        bad.format = "other".into();
        assert!(bad.validate().is_err());
        let mut bad = good.clone();
        bad.version = 2;
        assert!(bad.validate().is_err());
        let mut bad = good.clone();
        bad.recipients.clear();
        assert!(bad.validate().is_err());
        let mut bad = good;
        bad.vault_id = " ".into();
        assert!(bad.validate().is_err());
    }

    #[test]
    fn adding_a_recipient_re_encrypts_existing_files() {
        let tmp = dir();
        let created = create_encrypted_vault(tmp.path(), None, &[]).unwrap();
        created.vault.write_note("a.md", "# A\n").unwrap();
        created.vault.write_note("sub/b.md", "# B\n").unwrap();
        created.vault.move_to_trash("a.md").unwrap();

        let (second_secret, second_public) = generate_key_pair();
        assert!(open_encrypted_vault(tmp.path(), &second_secret).is_err());

        let rewritten = add_recipient(tmp.path(), &created.secret_key, &second_public).unwrap();
        assert_eq!(rewritten, 2);
        assert_eq!(
            add_recipient(tmp.path(), &created.secret_key, &second_public).unwrap(),
            0
        );

        let vault = open_encrypted_vault(tmp.path(), &second_secret).unwrap();
        assert_eq!(vault.read_note("sub/b.md").unwrap().title, "B");
        let first = open_encrypted_vault(tmp.path(), &created.secret_key).unwrap();
        assert_eq!(first.read_note("sub/b.md").unwrap().title, "B");
    }
}
