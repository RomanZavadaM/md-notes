//! Possible duplicates: notes that probably describe the same subject (ADR-0008, point 4).
//!
//! Names are not unambiguous, so nothing here merges or changes a note: it only
//! lists notes whose titles or aliases are equal after normalisation (case,
//! quotes, punctuation, hyphens, legal-form words such as `ТОВ`, Latin letters that
//! look like Cyrillic ones) so that a person can decide. Two notes whose identity
//! codes differ are different subjects and are never grouped.

use std::collections::BTreeMap;

use serde::Serialize;

use crate::error::Result;
use crate::identity;
use crate::vault::Vault;

/// Legal-form words dropped from the start or the end of a name.
const LEGAL_FORMS: &[&str] = &[
    "тов", "тзов", "пп", "ат", "пат", "прат", "кп", "дп", "фоп", "ооо", "llc", "ltd", "inc", "gmbh",
    "corp",
];

/// A Latin letter that looks like a Cyrillic one is folded onto the Cyrillic letter.
fn fold_letter(c: char) -> char {
    match c {
        'a' => 'а',
        'b' => 'в',
        'c' => 'с',
        'e' => 'е',
        'h' => 'н',
        'i' => 'і',
        'k' => 'к',
        'm' => 'м',
        'o' => 'о',
        'p' => 'р',
        't' => 'т',
        'x' => 'х',
        'y' => 'у',
        other => other,
    }
}

/// Comparison key of a name: lowercase words without punctuation and legal-form words.
/// Returns an empty string when nothing meaningful is left.
pub fn name_key(name: &str) -> String {
    let mut words: Vec<String> = name
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
        .map(str::to_lowercase)
        .collect();
    while words
        .first()
        .is_some_and(|w| LEGAL_FORMS.contains(&w.as_str()))
    {
        words.remove(0);
    }
    while words
        .last()
        .is_some_and(|w| LEGAL_FORMS.contains(&w.as_str()))
    {
        words.pop();
    }
    let folded: Vec<String> = words
        .iter()
        .map(|w| w.chars().map(fold_letter).collect::<String>())
        .collect();
    let key = folded.join(" ");
    if key.chars().count() < 4 {
        String::new()
    } else {
        key
    }
}

/// What is needed from a note to look for duplicates.
#[derive(Debug, Clone)]
pub struct Entry {
    pub path: String,
    pub title: String,
    pub aliases: Vec<String>,
    /// Normalised identity code, e.g. `edrpou:03114848`.
    pub identity: Option<String>,
}

/// One note of a group.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DuplicateNote {
    pub path: String,
    pub title: String,
    pub identity: Option<String>,
}

/// Notes that probably describe the same subject.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DuplicateGroup {
    pub notes: Vec<DuplicateNote>,
    /// True when every note carries the same identity code: they should have been merged.
    pub same_code: bool,
}

fn find(parent: &mut [usize], mut i: usize) -> usize {
    while parent[i] != i {
        parent[i] = parent[parent[i]];
        i = parent[i];
    }
    i
}

fn conflicting(a: &Entry, b: &Entry) -> bool {
    matches!((&a.identity, &b.identity), (Some(x), Some(y)) if x != y)
}

/// Groups entries whose titles or aliases share a name key (pure function, no I/O).
pub fn group_entries(entries: &[Entry]) -> Vec<DuplicateGroup> {
    let mut by_key: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for (i, entry) in entries.iter().enumerate() {
        let mut keys: Vec<String> = std::iter::once(&entry.title)
            .chain(entry.aliases.iter())
            .map(|n| name_key(n))
            .filter(|k| !k.is_empty())
            .collect();
        keys.sort();
        keys.dedup();
        for key in keys {
            by_key.entry(key).or_default().push(i);
        }
    }
    let mut parent: Vec<usize> = (0..entries.len()).collect();
    for members in by_key.values() {
        for (n, &a) in members.iter().enumerate() {
            for &b in &members[n + 1..] {
                if !conflicting(&entries[a], &entries[b]) {
                    let (ra, rb) = (find(&mut parent, a), find(&mut parent, b));
                    if ra != rb {
                        parent[rb] = ra;
                    }
                }
            }
        }
    }
    let mut components: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for i in 0..entries.len() {
        let root = find(&mut parent, i);
        components.entry(root).or_default().push(i);
    }
    let mut groups: Vec<DuplicateGroup> = components
        .into_values()
        .filter(|members| members.len() > 1)
        .map(|members| {
            let notes: Vec<DuplicateNote> = members
                .iter()
                .map(|&i| DuplicateNote {
                    path: entries[i].path.clone(),
                    title: entries[i].title.clone(),
                    identity: entries[i].identity.clone(),
                })
                .collect();
            let same_code = notes[0].identity.is_some()
                && notes.iter().all(|n| n.identity == notes[0].identity);
            DuplicateGroup { notes, same_code }
        })
        .collect();
    for group in &mut groups {
        group.notes.sort_by(|a, b| a.path.cmp(&b.path));
    }
    groups.sort_by(|a, b| a.notes[0].path.cmp(&b.notes[0].path));
    groups
}

impl Vault {
    /// Lists groups of notes that probably describe the same subject. Read-only.
    pub fn possible_duplicates(&self) -> Result<Vec<DuplicateGroup>> {
        let mut entries = Vec::new();
        for path in self.note_paths()? {
            let Ok(note) = self.read_note(&path) else {
                continue;
            };
            let aliases = note
                .front_matter
                .as_ref()
                .and_then(|fm| fm.get("aliases"))
                .and_then(|v| v.as_array())
                .map(|list| {
                    list.iter()
                        .filter_map(|v| v.as_str().map(str::to_string))
                        .collect()
                })
                .unwrap_or_default();
            entries.push(Entry {
                path,
                title: note.title.clone(),
                aliases,
                identity: identity::identity_of_content(&note.content),
            });
        }
        Ok(group_entries(&entries))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(path: &str, title: &str, aliases: &[&str], identity: Option<&str>) -> Entry {
        Entry {
            path: path.to_string(),
            title: title.to_string(),
            aliases: aliases.iter().map(|a| a.to_string()).collect(),
            identity: identity.map(str::to_string),
        }
    }

    #[test]
    fn names_are_compared_without_case_quotes_hyphens_and_legal_forms() {
        assert_eq!(name_key("ТОВ «ФІАКР-ЛЬВІВ»"), name_key("Фіакр Львів"));
        assert_eq!(name_key("ТЗОВ \"Фіакр-Львів\" "), name_key("фіакр львів"));
        assert_ne!(name_key("Фіакр Львів"), name_key("Фіакр Київ"));
    }

    #[test]
    fn latin_lookalike_letters_are_folded() {
        assert_eq!(name_key("Прим-Транс"), name_key("Пpим-Tранс"));
    }

    #[test]
    fn very_short_or_empty_names_are_ignored() {
        assert_eq!(name_key("ТОВ"), "");
        assert_eq!(name_key("АБ"), "");
    }

    #[test]
    fn notes_with_one_name_are_grouped_and_codes_are_shown() {
        let entries = vec![
            entry("a.md", "ТОВ «ФІАКР-ЛЬВІВ»", &[], Some("edrpou:03114848")),
            entry("b.md", "Фіакр Львів", &[], None),
            entry("c.md", "Інша компанія", &[], None),
        ];
        let groups = group_entries(&entries);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].notes.len(), 2);
        assert!(!groups[0].same_code);
    }

    #[test]
    fn an_alias_matches_a_title() {
        let entries = vec![
            entry("a.md", "Львівський автобусний завод", &["ЛАЗ"], None),
            entry("b.md", "Лаз-центр", &["Львівський автобусний завод"], None),
        ];
        assert_eq!(group_entries(&entries).len(), 1);
    }

    #[test]
    fn different_identity_codes_are_never_grouped() {
        let entries = vec![
            entry("a.md", "Фіакр Львів", &[], Some("edrpou:03114848")),
            entry("b.md", "ТОВ Фіакр-Львів", &[], Some("edrpou:12345678")),
        ];
        assert!(group_entries(&entries).is_empty());
    }

    #[test]
    fn the_same_code_is_flagged() {
        let entries = vec![
            entry("a.md", "Фіакр Львів", &[], Some("edrpou:03114848")),
            entry("b.md", "ТОВ Фіакр-Львів", &[], Some("edrpou:03114848")),
        ];
        let groups = group_entries(&entries);
        assert_eq!(groups.len(), 1);
        assert!(groups[0].same_code);
    }

    #[test]
    fn a_vault_scan_finds_the_pair() {
        let dir = tempfile::tempdir().unwrap();
        let vault = Vault::init(dir.path(), None).unwrap();
        vault
            .write_note(
                "A.md",
                "---\ntitle: \"ТОВ «ФІАКР-ЛЬВІВ»\"\n---\n# ТОВ «ФІАКР-ЛЬВІВ»\n",
            )
            .unwrap();
        vault
            .write_note("B.md", "# Фіакр Львів\n\nТекст.\n")
            .unwrap();
        vault.write_note("C.md", "# Зовсім інше\n").unwrap();
        let groups = vault.possible_duplicates().unwrap();
        assert_eq!(groups.len(), 1);
        let paths: Vec<&str> = groups[0].notes.iter().map(|n| n.path.as_str()).collect();
        assert_eq!(paths, vec!["A.md", "B.md"]);
    }
}
