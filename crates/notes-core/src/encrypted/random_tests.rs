//! Tests of the random-name layout (see `random.rs`).

use super::*;
use crate::index::Index;
use crate::refactor::rename_with_links;
use crate::storage::LocalFsProvider;
use crate::vault::Vault;
use std::path::Path;

fn local(dir: &Path) -> Arc<LocalFsProvider> {
    Arc::new(LocalFsProvider::new(dir).unwrap())
}

fn random(dir: &Path, identity: &age::x25519::Identity) -> EncryptedProvider {
    EncryptedProvider::from_identity(local(dir), identity.clone(), vec![])
        .unwrap()
        .with_layout(NameLayout::Random)
}

fn names(entries: Vec<StorageEntry>) -> Vec<String> {
    let mut out: Vec<String> = entries.into_iter().map(|e| e.path).collect();
    out.sort();
    out
}

/// Every file under `dir` (recursively) whose bytes contain `needle`.
fn files_containing(dir: &Path, needle: &[u8]) -> Vec<String> {
    let mut found = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(current) = stack.pop() {
        for entry in std::fs::read_dir(&current).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                stack.push(path);
            } else if std::fs::read(&path)
                .unwrap()
                .windows(needle.len())
                .any(|w| w == needle)
            {
                found.push(path.display().to_string());
            }
        }
    }
    found
}

#[test]
fn names_and_folders_are_not_on_disk() {
    let dir = tempfile::tempdir().unwrap();
    let identity = age::x25519::Identity::generate();
    let enc = random(dir.path(), &identity);
    enc.write("Прізвища/Іваненко.md", b"# secret body").unwrap();
    enc.write("Прізвища/sub/x.md", b"x").unwrap();
    enc.write("root.md", b"r").unwrap();

    assert_eq!(enc.read("Прізвища/Іваненко.md").unwrap(), b"# secret body");

    let mut on_disk = Vec::new();
    for entry in std::fs::read_dir(dir.path()).unwrap() {
        let entry = entry.unwrap();
        assert!(entry.path().is_file(), "no folders on disk");
        on_disk.push(entry.file_name().to_string_lossy().into_owned());
    }
    assert_eq!(on_disk.len(), 3);
    for name in &on_disk {
        assert_eq!(name.len(), 16 + ENCRYPTED_SUFFIX.len(), "{name}");
        assert!(name.ends_with(ENCRYPTED_SUFFIX));
        assert!(name[..16]
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()));
    }
    assert!(files_containing(dir.path(), "Прізвища".as_bytes()).is_empty());
    assert!(files_containing(dir.path(), "Іваненко".as_bytes()).is_empty());
    assert!(files_containing(dir.path(), b"secret body").is_empty());

    assert_eq!(names(enc.list("").unwrap()), vec!["root.md", "Прізвища"]);
    assert_eq!(
        names(enc.list("Прізвища").unwrap()),
        vec!["Прізвища/sub", "Прізвища/Іваненко.md"]
    );
    assert!(matches!(enc.list("Missing"), Err(Error::NotFound(_))));
}

#[test]
fn overwriting_keeps_the_same_physical_file() {
    let dir = tempfile::tempdir().unwrap();
    let identity = age::x25519::Identity::generate();
    let enc = random(dir.path(), &identity);
    enc.write("n.md", b"one").unwrap();
    let before: Vec<_> = std::fs::read_dir(dir.path())
        .unwrap()
        .map(|e| e.unwrap().file_name())
        .collect();
    enc.write("n.md", b"two").unwrap();
    let after: Vec<_> = std::fs::read_dir(dir.path())
        .unwrap()
        .map(|e| e.unwrap().file_name())
        .collect();
    assert_eq!(before.len(), 1);
    assert_eq!(before, after);
    assert_eq!(enc.read("n.md").unwrap(), b"two");
}

#[test]
fn rename_remove_and_metadata_use_logical_paths() {
    let dir = tempfile::tempdir().unwrap();
    let identity = age::x25519::Identity::generate();
    let enc = random(dir.path(), &identity);
    enc.write("one.md", b"1").unwrap();

    let meta = enc.metadata("one.md").unwrap().unwrap();
    assert_eq!(meta.path, "one.md");
    assert_eq!(meta.kind, StorageEntryKind::File);
    assert!(enc.metadata("missing.md").unwrap().is_none());

    enc.rename("one.md", "sub/two.md").unwrap();
    assert!(enc.metadata("one.md").unwrap().is_none());
    assert_eq!(enc.read("sub/two.md").unwrap(), b"1");
    assert_eq!(
        enc.metadata("sub").unwrap().unwrap().kind,
        StorageEntryKind::Dir
    );

    enc.write("sub/three.md", b"3").unwrap();
    enc.rename("sub", "moved").unwrap();
    assert_eq!(enc.read("moved/two.md").unwrap(), b"1");
    assert_eq!(enc.read("moved/three.md").unwrap(), b"3");
    assert!(enc.metadata("sub").unwrap().is_none());

    enc.write("other.md", b"o").unwrap();
    assert!(matches!(
        enc.rename("other.md", "moved/two.md"),
        Err(Error::AlreadyExists(_))
    ));

    enc.remove("moved/two.md").unwrap();
    assert!(enc.metadata("moved/two.md").unwrap().is_none());
    enc.remove("moved").unwrap();
    assert!(enc.metadata("moved").unwrap().is_none());
    assert!(matches!(enc.remove("moved"), Err(Error::NotFound(_))));
    assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 1);
}

#[test]
fn another_instance_sees_external_changes() {
    let dir = tempfile::tempdir().unwrap();
    let identity = age::x25519::Identity::generate();
    let a = random(dir.path(), &identity);
    let b = random(dir.path(), &identity);
    assert!(b.list("").unwrap().is_empty());

    a.write("from-a.md", b"hello").unwrap();
    assert_eq!(names(b.list("").unwrap()), vec!["from-a.md"]);
    assert_eq!(b.read("from-a.md").unwrap(), b"hello");

    a.write("from-a.md", b"changed").unwrap();
    assert_eq!(names(b.list("").unwrap()), vec!["from-a.md"]);
    assert_eq!(b.read("from-a.md").unwrap(), b"changed");

    a.remove("from-a.md").unwrap();
    assert!(b.list("").unwrap().is_empty());
    assert!(b.read("from-a.md").is_err());
}

#[test]
fn trash_files_live_in_the_trash_folder() {
    let dir = tempfile::tempdir().unwrap();
    let identity = age::x25519::Identity::generate();
    let enc = random(dir.path(), &identity);
    enc.write("t.md", b"TRASH-MARKER").unwrap();
    enc.rename("t.md", ".mdnotes/trash/20261019/t.md").unwrap();

    assert!(enc.metadata("t.md").unwrap().is_none());
    assert_eq!(
        enc.read(".mdnotes/trash/20261019/t.md").unwrap(),
        b"TRASH-MARKER"
    );
    let in_trash = std::fs::read_dir(dir.path().join(".mdnotes/trash"))
        .unwrap()
        .count();
    assert_eq!(in_trash, 1);
    let at_root = std::fs::read_dir(dir.path())
        .unwrap()
        .filter(|e| e.as_ref().unwrap().path().is_file())
        .count();
    assert_eq!(at_root, 0);
    assert!(files_containing(dir.path(), b"TRASH-MARKER").is_empty());
}

#[test]
fn empty_folders_survive_through_a_marker() {
    let dir = tempfile::tempdir().unwrap();
    let identity = age::x25519::Identity::generate();
    let enc = random(dir.path(), &identity);
    enc.create_dir("Порожня").unwrap();
    enc.create_dir("Порожня").unwrap();

    assert_eq!(names(enc.list("").unwrap()), vec!["Порожня"]);
    assert!(enc.list("Порожня").unwrap().is_empty());
    assert_eq!(
        enc.metadata("Порожня").unwrap().unwrap().kind,
        StorageEntryKind::Dir
    );
    assert!(files_containing(dir.path(), "Порожня".as_bytes()).is_empty());

    let fresh = random(dir.path(), &identity);
    assert_eq!(names(fresh.list("").unwrap()), vec!["Порожня"]);
    fresh.remove("Порожня").unwrap();
    assert!(fresh.metadata("Порожня").unwrap().is_none());
}

#[test]
fn service_files_pass_through_and_foreign_keys_see_nothing() {
    let dir = tempfile::tempdir().unwrap();
    let identity = age::x25519::Identity::generate();
    let enc = random(dir.path(), &identity);
    enc.write(".mdnotes/recipients.txt", b"age1public").unwrap();
    assert_eq!(
        std::fs::read(dir.path().join(".mdnotes/recipients.txt")).unwrap(),
        b"age1public"
    );
    enc.write("n.md", b"x").unwrap();

    let stranger = random(dir.path(), &age::x25519::Identity::generate());
    assert!(names(stranger.list("").unwrap())
        .iter()
        .all(|name| name.starts_with('.')));
    assert!(stranger.read("n.md").is_err());

    let write_only = EncryptedProvider::new(local(dir.path()), vec![identity.to_public()], vec![])
        .unwrap()
        .with_layout(NameLayout::Random);
    assert!(matches!(write_only.list(""), Err(Error::Crypto(_))));
}

#[test]
fn changes_since_reports_logical_paths() {
    let dir = tempfile::tempdir().unwrap();
    let identity = age::x25519::Identity::generate();
    let enc = random(dir.path(), &identity);
    enc.write("sub/n.md", b"x").unwrap();
    let changed = names(enc.changes_since(0).unwrap());
    assert_eq!(changed, vec!["sub/n.md"]);
}

#[test]
fn vault_features_work_with_random_names() {
    let dir = tempfile::tempdir().unwrap();
    let identity = age::x25519::Identity::generate();
    let provider: Arc<dyn StorageProvider> = Arc::new(random(dir.path(), &identity));
    let vault = Vault::open_with_provider(dir.path(), provider).unwrap();

    let folder = vault.create_folder("", "Проєкти").unwrap();
    assert_eq!(folder, "Проєкти");
    vault
        .write_note(
            "Проєкти/Target.md",
            "---\ntitle: Visible\n---\n# T\nuniqueword\n",
        )
        .unwrap();
    vault.write_note("a.md", "see [[Target]]\n").unwrap();

    let tree = vault.tree().unwrap();
    assert!(tree.iter().any(|e| e.name == "Проєкти"));
    assert_eq!(vault.note_paths().unwrap().len(), 2);

    let mut index = Index::open_in_memory().unwrap();
    assert_eq!(index.sync(&vault).unwrap().added, 2);
    assert_eq!(index.search("uniqueword", 10).unwrap().len(), 1);

    rename_with_links(&vault, &mut index, "Проєкти/Target.md", "Проєкти/Renamed.md").unwrap();
    assert!(vault
        .read_note("a.md")
        .unwrap()
        .content
        .contains("[[Renamed]]"));
    assert_eq!(
        vault.read_note("Проєкти/Renamed.md").unwrap().title,
        "Visible"
    );

    let trash = vault.move_to_trash("Проєкти").unwrap();
    assert!(trash.starts_with(".mdnotes/trash/"));
    assert_eq!(vault.note_paths().unwrap(), vec!["a.md"]);

    let source_dir = tempfile::tempdir().unwrap();
    let source = source_dir.path().join("pic.png");
    std::fs::write(&source, b"ATTACH-MARKER bytes").unwrap();
    let info = vault.import_attachment(&source).unwrap();
    assert!(info.path.ends_with("pic.png"));
    assert_eq!(vault.attachments().unwrap().len(), 1);
    assert!(files_containing(dir.path(), b"ATTACH-MARKER").is_empty());
    assert!(files_containing(dir.path(), "Проєкти".as_bytes()).is_empty());
    assert!(files_containing(dir.path(), b"Renamed").is_empty());
}
