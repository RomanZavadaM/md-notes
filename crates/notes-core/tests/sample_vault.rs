//! Checks that the bundled `sample-vault/` stays valid: every note parses and
//! every wiki link points to an existing note.

use std::path::PathBuf;

use notes_core::Vault;

/// Links in the sample that intentionally point to missing notes.
const EXPECTED_MISSING: &[&str] = &["Нова ідея"];

fn sample_vault() -> Vault {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../sample-vault");
    Vault::open(root).expect("sample vault opens")
}

#[test]
fn sample_vault_config_is_loaded() {
    let vault = sample_vault();
    assert_eq!(vault.display_name(), "Приклад бази знань");
    assert_eq!(vault.config().daily_notes_dir, "Щоденник");
}

#[test]
fn sample_notes_are_valid_and_linked() {
    let vault = sample_vault();
    let paths = vault.note_paths().unwrap();
    assert!(paths.len() >= 10, "sample vault has notes: {paths:?}");

    for path in &paths {
        let note = vault.read_note(path).unwrap();
        assert!(
            note.front_matter_error.is_none(),
            "{path}: {:?}",
            note.front_matter_error
        );
        let fm = note.front_matter.as_ref().expect("every sample note has properties");
        assert!(fm.get("id").and_then(|v| v.as_str()).is_some(), "{path}: missing id");
        assert!(fm.get("type").and_then(|v| v.as_str()).is_some(), "{path}: missing type");

        for link in note.links.iter().filter(|l| !l.target.is_empty() && !l.embed) {
            let resolved = vault.resolve_link(&link.target).unwrap();
            if EXPECTED_MISSING.contains(&link.target.as_str()) {
                assert!(resolved.is_none(), "{path}: {} should be missing", link.target);
            } else {
                assert!(resolved.is_some(), "{path}: broken link [[{}]]", link.target);
            }
        }
    }
}

#[test]
fn sample_ids_are_unique() {
    let vault = sample_vault();
    let mut ids = Vec::new();
    for path in vault.note_paths().unwrap() {
        let note = vault.read_note(&path).unwrap();
        let id = note.front_matter.unwrap()["id"].as_str().unwrap().to_string();
        assert!(!ids.contains(&id), "duplicate id {id} in {path}");
        ids.push(id);
    }
}
