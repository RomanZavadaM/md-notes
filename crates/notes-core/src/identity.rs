//! Identity keys and merging of notes about the same subject (ADR-0008).
//!
//! A note about a legal entity or a person may carry an unambiguous code in its
//! front matter (`edrpou`, `ipn`). Two notes with the same normalised code are
//! the same subject. [`merge_into`] folds an incoming note into the existing one
//! without losing anything: missing properties are added, lists are united,
//! differing scalar values are kept as they were and listed as conflicts, and the
//! incoming text is appended once (a marker makes the merge idempotent).
//! Names are never used for matching: namesakes and spelling variants are not
//! unambiguous.

use serde_yaml::{Mapping, Value};

use crate::markdown::split_front_matter;

/// Front matter properties that identify a subject, with the digit count of a valid code.
pub const IDENTITY_KEYS: &[(&str, usize)] = &[("edrpou", 8), ("ipn", 10)];

const MARKER: &str = "merged-from:";
/// Properties merged as unions of values.
const LIST_KEYS: &[&str] = &["tags", "aliases"];
/// Properties never copied from the incoming note.
const SKIP_KEYS: &[&str] = &["id"];

fn key(name: &str) -> Value {
    Value::String(name.to_string())
}

fn code_digits(value: &Value, len: usize) -> Option<String> {
    let (text, numeric) = match value {
        Value::String(s) => (s.clone(), false),
        Value::Number(n) => (n.to_string(), true),
        _ => return None,
    };
    let digits: String = text.chars().filter(char::is_ascii_digit).collect();
    // An unquoted YAML number loses leading zeros; restore them.
    let digits = if numeric && !digits.is_empty() && digits.len() < len {
        format!("{digits:0>len$}")
    } else {
        digits
    };
    (digits.len() == len).then_some(digits)
}

/// Normalised identity of a note's front matter, e.g. `edrpou:03114848`.
pub fn identity_key(front: &Mapping) -> Option<String> {
    IDENTITY_KEYS.iter().find_map(|(name, len)| {
        let value = front.get(key(name))?;
        code_digits(value, *len).map(|digits| format!("{name}:{digits}"))
    })
}

/// Front matter mapping and body of a note, when the front matter is a valid mapping.
fn parse(content: &str) -> Option<(Mapping, &str)> {
    let (yaml, body) = split_front_matter(content);
    let yaml = yaml?;
    if yaml.trim().is_empty() {
        return Some((Mapping::new(), body));
    }
    match serde_yaml::from_str::<Value>(yaml).ok()? {
        Value::Mapping(map) => Some((map, body)),
        _ => None,
    }
}

/// Identity of a whole note given its text (front matter must be valid).
pub fn identity_of_content(content: &str) -> Option<String> {
    let (front, _) = parse(content)?;
    identity_key(&front)
}

/// Result of merging an incoming note into an existing one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MergeOutcome {
    /// New content of the existing note (equal to the old one when `changed` is false).
    pub content: String,
    /// Differing scalar values kept from the existing note.
    pub conflicts: u32,
    /// False when the incoming note was merged earlier.
    pub changed: bool,
}

fn fnv1a(text: &str) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in text.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0100_0000_01b3);
    }
    hash
}

fn text_of(value: &Value) -> String {
    let raw = match value {
        Value::String(s) => s.clone(),
        other => serde_yaml::to_string(other).unwrap_or_default(),
    };
    let flat: String = raw.split_whitespace().collect::<Vec<_>>().join(" ");
    flat.chars().take(200).collect()
}

fn same_text(a: &str, b: &str) -> bool {
    a.trim().to_lowercase() == b.trim().to_lowercase()
}

fn is_empty_value(value: &Value) -> bool {
    match value {
        Value::Null => true,
        Value::String(s) => s.trim().is_empty(),
        Value::Sequence(seq) => seq.is_empty(),
        Value::Mapping(map) => map.is_empty(),
        _ => false,
    }
}

fn as_strings(value: Option<&Value>) -> Vec<String> {
    match value {
        Some(Value::Sequence(items)) => items.iter().map(text_of).collect(),
        Some(Value::String(s)) if !s.trim().is_empty() => vec![s.trim().to_string()],
        _ => Vec::new(),
    }
}

fn union_strings(into: &mut Vec<String>, more: Vec<String>) {
    for item in more {
        if !item.is_empty() && !into.iter().any(|have| same_text(have, &item)) {
            into.push(item);
        }
    }
}

fn relation_id(value: &Value) -> String {
    let field = |name: &str| {
        value
            .as_mapping()
            .and_then(|m| m.get(key(name)))
            .map(text_of)
            .unwrap_or_default()
            .to_lowercase()
    };
    format!("{}|{}|{}", field("type"), field("to"), field("dir"))
}

fn merge_relations(existing: &mut Mapping, incoming: &Value) {
    let Value::Sequence(new_items) = incoming else {
        return;
    };
    let mut have: Vec<String> = match existing.get(key("relations")) {
        Some(Value::Sequence(items)) => items.iter().map(relation_id).collect(),
        _ => Vec::new(),
    };
    let mut merged: Vec<Value> = match existing.get(key("relations")) {
        Some(Value::Sequence(items)) => items.clone(),
        _ => Vec::new(),
    };
    for item in new_items {
        let id = relation_id(item);
        if !have.contains(&id) {
            have.push(id);
            merged.push(item.clone());
        }
    }
    existing.insert(key("relations"), Value::Sequence(merged));
}

fn body_without_title<'a>(body: &'a str, title: &str) -> &'a str {
    let trimmed = body.trim_start_matches(['\r', '\n']);
    if let Some(first_line) = trimmed.lines().next() {
        if let Some(heading) = first_line.strip_prefix("# ") {
            if same_text(heading, title) {
                return trimmed[first_line.len()..].trim_start_matches(['\r', '\n']);
            }
        }
    }
    trimmed
}

/// Folds `incoming` (read from `incoming_path` of the imported folder) into
/// `existing`. Returns `None` when either note has no valid front matter.
pub fn merge_into(existing: &str, incoming: &str, incoming_path: &str) -> Option<MergeOutcome> {
    let (mut front, body) = parse(existing)?;
    let (new_front, new_body) = parse(incoming)?;
    let marker = format!("<!-- {MARKER} {:016x} -->", fnv1a(incoming));
    if existing.contains(&marker) {
        return Some(MergeOutcome {
            content: existing.to_string(),
            conflicts: 0,
            changed: false,
        });
    }

    let old_title = front.get(key("title")).map(text_of).unwrap_or_default();
    let new_title = new_front.get(key("title")).map(text_of).unwrap_or_default();
    let mut aliases = as_strings(front.get(key("aliases")));
    let mut conflicts: Vec<(String, String, String)> = Vec::new();

    for (name, value) in &new_front {
        let Some(name) = name.as_str() else { continue };
        if SKIP_KEYS.contains(&name) || name == "title" || name == "aliases" {
            continue;
        }
        let current = front.get(key(name)).cloned();
        match current {
            None => {
                front.insert(key(name), value.clone());
            }
            Some(old) if is_empty_value(&old) => {
                front.insert(key(name), value.clone());
            }
            Some(old) if old == *value => {}
            Some(_) if name == "relations" => merge_relations(&mut front, value),
            Some(old) if LIST_KEYS.contains(&name) => {
                let mut union = as_strings(Some(&old));
                union_strings(&mut union, as_strings(Some(value)));
                front.insert(
                    key(name),
                    Value::Sequence(union.into_iter().map(Value::String).collect()),
                );
            }
            Some(old) => conflicts.push((name.to_string(), text_of(&old), text_of(value))),
        }
    }
    union_strings(&mut aliases, as_strings(new_front.get(key("aliases"))));
    if !new_title.is_empty() && !same_text(&new_title, &old_title) {
        union_strings(&mut aliases, vec![new_title.clone()]);
    }
    if !aliases.is_empty() {
        front.insert(
            key("aliases"),
            Value::Sequence(aliases.into_iter().map(Value::String).collect()),
        );
    }

    let yaml = serde_yaml::to_string(&Value::Mapping(front)).ok()?;
    let mut out = format!("---\n{yaml}---\n{}", body.trim_start_matches(['\r', '\n']));
    if !out.ends_with('\n') {
        out.push('\n');
    }
    out.push_str(&format!(
        "\n## Об'єднано з імпорту (`{incoming_path}`)\n{marker}\n"
    ));
    let text = body_without_title(new_body, &new_title).trim_end();
    if !text.is_empty() {
        out.push_str(text);
        out.push('\n');
    }
    if !conflicts.is_empty() {
        out.push_str("\n### Розбіжності при об'єднанні\n");
        for (name, old, new) in &conflicts {
            out.push_str(&format!(
                "- `{name}`: у сховищі «{old}»; в імпорті «{new}»\n"
            ));
        }
    }
    Some(MergeOutcome {
        content: out,
        conflicts: u32::try_from(conflicts.len()).unwrap_or(u32::MAX),
        changed: true,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn front(src: &str) -> Mapping {
        match serde_yaml::from_str::<Value>(src).unwrap() {
            Value::Mapping(m) => m,
            _ => panic!("not a mapping"),
        }
    }

    #[test]
    fn normalises_codes() {
        assert_eq!(
            identity_key(&front("edrpou: \"03114848\"")).as_deref(),
            Some("edrpou:03114848")
        );
        assert_eq!(
            identity_key(&front("edrpou: \"0311 4848\"")).as_deref(),
            Some("edrpou:03114848")
        );
        // An unquoted number lost its leading zero.
        assert_eq!(
            identity_key(&front("edrpou: 3114848")).as_deref(),
            Some("edrpou:03114848")
        );
        assert_eq!(
            identity_key(&front("ipn: \"1234567890\"")).as_deref(),
            Some("ipn:1234567890")
        );
        assert_eq!(identity_key(&front("edrpou: \"123\"")), None);
        assert_eq!(identity_key(&front("title: x")), None);
    }

    #[test]
    fn identity_of_content_needs_valid_front_matter() {
        assert_eq!(
            identity_of_content("---\nedrpou: \"03114848\"\n---\n# A\n").as_deref(),
            Some("edrpou:03114848")
        );
        assert_eq!(identity_of_content("# A\nno front matter\n"), None);
        assert_eq!(identity_of_content("---\n[unclosed\n---\n"), None);
    }

    const EXISTING: &str = "---\ntitle: \"ТОВ «ФІАКР-ЛЬВІВ»\"\nedrpou: \"03114848\"\ntags: [транспорт]\nstatus: unverified\nrelations:\n  - {type: засновник, to: Левик, dir: in, origin: registry}\n---\n# ТОВ «ФІАКР-ЛЬВІВ»\n\nТекст у сховищі.\n";
    const INCOMING: &str = "---\ntitle: \"ТЗОВ ФІАКР-ЛЬВІВ\"\nedrpou: \"03114848\"\ntags: [львів, транспорт]\nstatus: verified\nregistered: \"1995-04-28\"\nrelations:\n  - {type: засновник, to: левик, dir: in, origin: media}\n  - {type: керівник, to: Коцан, dir: in, origin: registry}\n---\n# ТЗОВ ФІАКР-ЛЬВІВ\n\nНовий текст з імпорту.\n";

    #[test]
    fn merges_properties_relations_aliases_and_text() {
        let out = merge_into(EXISTING, INCOMING, "Суб'єкти/Фіакр.md").unwrap();
        assert!(out.changed);
        assert_eq!(out.conflicts, 1); // status differs
        let (front, body) = parse(&out.content).unwrap();
        assert_eq!(front.get(key("status")), Some(&key("unverified")));
        assert_eq!(front.get(key("registered")), Some(&key("1995-04-28")));
        let tags = as_strings(front.get(key("tags")));
        assert_eq!(tags, vec!["транспорт".to_string(), "львів".to_string()]);
        let aliases = as_strings(front.get(key("aliases")));
        assert_eq!(aliases, vec!["ТЗОВ ФІАКР-ЛЬВІВ".to_string()]);
        // The same relation (type, to, dir) is not duplicated; the new one is added.
        match front.get(key("relations")) {
            Some(Value::Sequence(items)) => assert_eq!(items.len(), 2),
            other => panic!("no relations: {other:?}"),
        }
        assert!(body.contains("Текст у сховищі."));
        assert!(body.contains("## Об'єднано з імпорту (`Суб'єкти/Фіакр.md`)"));
        assert!(body.contains("Новий текст з імпорту."));
        assert!(!body.contains("# ТЗОВ ФІАКР-ЛЬВІВ\n"));
        assert!(body.contains("`status`: у сховищі «unverified»; в імпорті «verified»"));
    }

    #[test]
    fn merging_twice_changes_nothing() {
        let first = merge_into(EXISTING, INCOMING, "a.md").unwrap();
        let second = merge_into(&first.content, INCOMING, "a.md").unwrap();
        assert!(!second.changed);
        assert_eq!(second.content, first.content);
    }

    #[test]
    fn notes_without_front_matter_are_not_merged() {
        assert!(merge_into(EXISTING, "# Просто текст\n", "a.md").is_none());
        assert!(merge_into("# Просто текст\n", INCOMING, "a.md").is_none());
    }
}
