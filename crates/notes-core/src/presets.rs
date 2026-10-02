use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::{io_err, Error, Result};
use crate::vault::Vault;

/// Built-in structures for a newly created vault.
///
/// Presets only create normal folders, Markdown templates and MD Notes service
/// metadata. They never introduce a proprietary note format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VaultPreset {
    Empty,
    Para,
    Zettelkasten,
}

/// Creates a new vault in an existing empty directory.
///
/// This deliberately refuses to apply a preset over any existing entry. A
/// preset is a creation operation, not a migration or a merge operation.
pub fn create_vault_with_preset(
    root: impl AsRef<Path>,
    name: Option<String>,
    preset: VaultPreset,
) -> Result<Vault> {
    let root = root.as_ref();
    if !root.is_dir() {
        return Err(Error::NotFound(root.display().to_string()));
    }
    if fs::read_dir(root)
        .map_err(io_err(root))?
        .next()
        .transpose()
        .map_err(io_err(root))?
        .is_some()
    {
        return Err(Error::AlreadyExists(format!(
            "vault creation requires an empty folder: {}",
            root.display()
        )));
    }

    let vault = Vault::init(root, name)?;
    match preset {
        VaultPreset::Empty => {}
        VaultPreset::Para => {
            create_directories(root, &["Projects", "Areas", "Resources", "Archives"])?
        }
        VaultPreset::Zettelkasten => {
            create_directories(root, &["Notes", "Sources", "daily"])?;
            let template = root.join(".mdnotes/templates/zettel.md");
            fs::write(
                &template,
                "---\ntype: note\ntags: []\n---\n\n# {{title}}\n\n## Idea\n\n## Links\n\n",
            )
            .map_err(io_err(&template))?;
        }
    }
    Ok(vault)
}

fn create_directories(root: &Path, names: &[&str]) -> Result<()> {
    for name in names {
        let path = root.join(name);
        fs::create_dir(&path).map_err(io_err(&path))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_preset_only_initializes_service_metadata() {
        let dir = tempfile::tempdir().unwrap();
        let vault =
            create_vault_with_preset(dir.path(), Some("Empty".into()), VaultPreset::Empty).unwrap();
        assert_eq!(vault.display_name(), "Empty");
        assert!(dir.path().join(".mdnotes/config.json").is_file());
        assert!(vault.tree().unwrap().is_empty());
    }

    #[test]
    fn para_preset_creates_canonical_folders() {
        let dir = tempfile::tempdir().unwrap();
        create_vault_with_preset(dir.path(), None, VaultPreset::Para).unwrap();
        for name in ["Projects", "Areas", "Resources", "Archives"] {
            assert!(dir.path().join(name).is_dir(), "missing {name}");
        }
    }

    #[test]
    fn zettelkasten_preset_creates_open_structure_and_template() {
        let dir = tempfile::tempdir().unwrap();
        create_vault_with_preset(dir.path(), None, VaultPreset::Zettelkasten).unwrap();
        for name in ["Notes", "Sources", "daily"] {
            assert!(dir.path().join(name).is_dir(), "missing {name}");
        }
        let template = dir.path().join(".mdnotes/templates/zettel.md");
        let content = fs::read_to_string(template).unwrap();
        assert!(content.contains("# {{title}}"));
        assert!(content.contains("type: note"));
    }

    #[test]
    fn preset_refuses_non_empty_folder_without_modifying_it() {
        let dir = tempfile::tempdir().unwrap();
        let existing = dir.path().join("keep.md");
        fs::write(&existing, "keep me").unwrap();

        let result = create_vault_with_preset(dir.path(), None, VaultPreset::Para);
        assert!(matches!(result, Err(Error::AlreadyExists(_))));
        assert_eq!(fs::read_to_string(existing).unwrap(), "keep me");
        assert!(!dir.path().join(".mdnotes").exists());
        assert!(!dir.path().join("Projects").exists());
    }
}
