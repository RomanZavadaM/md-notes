# Backup and recovery — how it works and a safe procedure

[🇺🇦 Українська](BACKUP_AND_RECOVERY.uk.md) · **🇬🇧 English** · [🇫🇷 Français](BACKUP_AND_RECOVERY.fr.md) · [🇩🇪 Deutsch](BACKUP_AND_RECOVERY.de.md) · [🇪🇸 Español](BACKUP_AND_RECOVERY.es.md) · [🇰🇷 한국어](BACKUP_AND_RECOVERY.ko.md) · [🇯🇵 日本語](BACKUP_AND_RECOVERY.ja.md)

> Applies to MD Notes v0.3.23. Ukrainian is canonical. Backup/restore has shipped since v0.3.22; automated desktop tests exist, but a separate full owner runtime acceptance of this procedure has not been recorded.

## 1. Backup, sync and Git are different

A **backup** is a self-contained ZIP snapshot intended for recovery. **Sync** keeps copies current and may also propagate mistakes or deletions. **Git** keeps version history but does not automatically contain every local item and never substitutes for encrypted-vault key backup.

Keep at least one backup independent from the working vault and its synchronization path.

## 2. What “Create backup…” does

The File menu command:
1. attempts to save open documents;
2. walks the physical files of the current vault;
3. excludes non-portable/service data;
4. computes SHA-256 for every included file;
5. writes the files into ZIP;
6. adds an open `manifest.json`;
7. checks again that the source did not change during the operation;
8. only then promotes the temporary archive to the selected backup name.

If files changed while the snapshot was being made, the operation should fail instead of silently creating a mixed-state backup.

## 3. Included data

Normally included: Markdown notes, attachments, portable `.mdnotes` configuration/schema/templates/views, `.mdnotes/trash/`, and encrypted-vault configuration where applicable.

## 4. Excluded data

The backup excludes:
- `.git/`;
- `.mdnotes/cache/`;
- MD Notes temporary files;
- common system noise such as `.DS_Store`, `Thumbs.db`, `desktop.ini`;
- age private/recovery private keys.

Symbolic links are rejected to prevent unexpected traversal outside the vault.

## 5. Manifest

`manifest.json` records format `md-notes-backup`, format version, app version, creation time, vault name, encrypted flag, and for every file its relative path, size and SHA-256.

The manifest is for **integrity and completeness**, not encryption. It is open metadata and may reveal some information even when encrypted-vault content itself is ciphertext.

## 6. Format v1 safety limits

Current limits:
- up to 100,000 files;
- up to 4 GiB per file;
- up to 16 GiB total;
- no absolute paths, `..` or path traversal;
- no duplicate paths;
- no case-insensitive path collisions.

## 7. Validation before restore

MD Notes does not immediately unpack into a destination vault. It first:
1. reads the manifest;
2. validates format/version;
3. validates every relative path;
4. rejects unexpected or duplicate ZIP entries;
5. checks sizes;
6. recalculates SHA-256;
7. confirms that every manifest file exists and no extras are present.

Any failure stops restore before a working vault is created.

## 8. Restore destination

After validation, files are extracted to a temporary folder and then promoted to a **new** vault. Existing folders are never silently overwritten.

Names are chosen like:
`My Vault-restored`, `My Vault-restored-2`, and so on.

This lets you compare the old and restored state before discarding anything.

## 9. Normal vault

A backup of a normal vault contains readable Markdown and attachments. Protect the ZIP as carefully as the original folder.

## 10. Encrypted vault

Backup reads the **physical on-disk files**, so age-encrypted note/attachment files remain ciphertext in the ZIP.

However:
- service metadata such as `.mdnotes/encryption.json` remains open;
- cache is excluded;
- private/recovery private keys are excluded.

Complete disaster recovery therefore requires two independent components:
1. the vault backup;
2. at least one private age key matching a recipient.

See [Encrypted vault](ENCRYPTED_VAULT.en.md).

## 11. Recommended backup model

For important data keep the working copy, regular MD Notes backups on another medium/location, and a separate recovery-key copy for encrypted vaults. The common 3-2-1 principle is a useful target for critical data.

## 12. Create-backup procedure

1. Finish or save critical edits.
2. Choose **File → Create backup…**.
3. Save the ZIP outside the vault itself.
4. Confirm the application reports success.
5. Do not manually rename entries inside the ZIP.
6. Copy important backups to independent storage.
7. For encrypted vaults, verify that the recovery key is stored separately.

## 13. Restore procedure

1. Do not delete the old/problematic vault first.
2. Choose **File → Restore backup…**.
3. Select the ZIP and destination parent folder.
4. Let manifest/checksum validation finish.
5. Open the new `*-restored` vault.
6. Verify several important notes, attachments and the graph.
7. For encrypted vaults, import a private key if required.
8. Only then decide what to do with the old copy.

## 14. Recovery drill

Periodically restore a current backup to a separate folder, open important notes, check attachments/graph, and for encrypted vaults confirm that a usable recovery key exists. Remove the test restored vault afterwards if it is no longer needed.

A backup never tested for restore provides less confidence than a tested procedure.

## 15. Common mistakes

- backup on the same failing physical disk;
- ciphertext exists but no key;
- key exists but no current vault copy;
- treating sync as a full backup;
- uploading a normal-vault ZIP publicly;
- deleting the old vault before validating the restored copy;
- creating a backup while external software is mass-rewriting the vault.

## 16. Checksum failure

Do not edit the manifest to “make it pass.” A checksum mismatch means the archive no longer matches the recorded snapshot. Preserve it for analysis and use another backup or create a fresh one from the source if possible.

## 17. AI and automation

An AI must not delete the last known-good copy before verified restore, treat sync/Git as equivalent to backup, put private keys into ZIP, bypass checksum/path validation, extract over an existing vault, or claim recovery succeeded without checking the created restored vault.

Encrypted-data rules: [ENCRYPTED_VAULT_CONTRACT.md](../ai/ENCRYPTED_VAULT_CONTRACT.md).

> This translation was generated automatically and has not been reviewed by a native speaker; Ukrainian is canonical.
