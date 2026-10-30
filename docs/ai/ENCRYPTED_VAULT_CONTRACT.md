# MD Notes Encrypted Vault Contract for AI and automation

**Contract:** `md-notes-encrypted-vault/2`  
**Applies to:** current v1.0.0 behavior  
**Audience:** AI agents, automation, backup/migration tooling and code changes that touch encrypted vaults.

## 1. Security model

1. Encryption is optional and per-vault.
2. Ordinary vault behavior **MUST remain unchanged** when encryption is not enabled.
3. The format is **age**, currently X25519 recipients. Custom cryptography **MUST NOT** be introduced.
4. User-content plaintext **MUST be encrypted in memory before it is handed to the underlying persistent storage provider**.
5. A private age identity **MUST NOT** be stored inside the vault.

## 2. Current physical layouts

### New vaults
New v1.0.0 encrypted vaults use `random` name layout:
- physical files use random `*.age` names;
- logical note/folder paths live inside the encrypted MD Notes envelope;
- logical names **MUST NOT** be reconstructed by filename guessing.

### Legacy vaults
Older vaults may use `named` layout:
- logical path is represented physically with an added `.age` suffix;
- filename/folder metadata may therefore be visible.

Automation **MUST support the layout declared by encryption config**, not assume every vault is random.

## 3. Service directory

`.mdnotes/` is service metadata and is generally passed through unencrypted, **except `.mdnotes/trash/`**, which stores encrypted user files.

`.mdnotes/encryption.json` is intentionally open and may contain:
- format/version;
- `vaultId`;
- public `age1...` recipients;
- name layout.

It **MUST NOT** contain a private key.

## 4. Private keys

Private key form: `AGE-SECRET-KEY-...`.

A private key **MUST NOT** be written to:
- vault files;
- `.mdnotes/encryption.json`;
- `localStorage`;
- Git configuration or repository data;
- application logs;
- backup ZIP;
- telemetry/network services.

On supported desktop systems the working key is stored in the OS secret store.

An AI **MUST NOT** echo a real private key into documentation, issue comments, commit messages or logs.

## 5. Recovery recipients

Multiple public recipients MAY be configured. A recovery private key is user-controlled and SHOULD be stored separately/offline.

In the 1.0 stabilization, user-facing UI can add a **public** recovery recipient to an already-created encrypted vault. The operation re-encrypts vault files for all recipients while the working private key remains in the OS secret store. Automation **MUST NOT** ask for or store the recovery private key when adding a recipient.

## 6. Plaintext cache exception

`.mdnotes/cache/index.db` is a deliberate current MVP exception:
- it may contain derived plaintext;
- it is local;
- it is excluded from sync/Git by project rules;
- built-in backup excludes it.

Agents **MUST NOT** claim the cache is encrypted or that encrypted vault equals full-disk encryption.

Any future change to encrypt the cache requires an explicit project decision/contract update.

## 7. Attachments

The storage layer encrypts arbitrary attachment bytes. In the 1.0 stabilization, images and PDFs are read through the active provider, decrypted only in memory, and exposed to the WebView as in-memory Blob URLs.

Agents **MUST NOT** create plaintext temporary files merely to preview or externally open encrypted attachments. External “Open file” stays disabled for encrypted attachments unless a future design provides an equally strong no-plaintext-on-disk handoff. Other attachment types MAY remain without inline preview while their stored bytes stay encrypted.

## 8. Git/synchronization

Encrypted-vault Git push uses a plaintext guard. Automation **MUST NOT** intentionally stage/push a decrypted mirror of user content.

For random-name vaults, logical paths remain inside ciphertext. For legacy named vaults, path metadata may be visible.

## 9. Backup/restore

Built-in backup copies physical files:
- encrypted user files remain ciphertext;
- `.git` excluded;
- `.mdnotes/cache` excluded;
- private/recovery private keys excluded.

Restore validates manifest, safe paths, sizes and SHA-256, then restores into a **new** vault path. Automation **MUST NOT** bypass validation or overwrite an existing vault silently.

An encrypted backup without a usable private/recovery key may be intact but unreadable. MD Notes has no master key.

## 10. Threat-boundary statements

Agents MAY state that encrypted vault protects vault copies against reading without a key.

Agents **MUST NOT** state that it protects against:
- malware running under an already unlocked user session;
- compromise of OS secret storage;
- screen capture/open-note observation;
- loss of all keys;
- plaintext leakage from the current local cache exception;
- all metadata analysis.

## 11. Forbidden AI/automation actions

An agent **MUST NOT**:
- persist decrypted user content “temporarily” outside an explicitly secure in-memory flow;
- convert an encrypted vault to plaintext without explicit owner/user authorization and a designed migration;
- store keys next to the vault for convenience;
- upload private keys to Git/issues/logs;
- remove the last working key copy;
- claim lost keys can be recovered by MD Notes;
- weaken encryption or recipient handling to make a failing operation pass;
- treat a checksum/crypto failure as ignorable.

## 12. Contract changes

Any intentional change to layout, key storage, plaintext exceptions, backup behavior or threat boundaries **MUST** update:
- this contract;
- `encrypted-vault-contract.yaml`;
- ADR/project rules as appropriate;
- Ukrainian human guide and all six translations;
- relevant automated tests.

Machine-readable companion: [encrypted-vault-contract.yaml](encrypted-vault-contract.yaml).
