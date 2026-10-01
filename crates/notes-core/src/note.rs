use serde::Serialize;
use serde_json::Value;

use crate::markdown::{
    extract_inline_tags, extract_title, extract_wikilinks, mask_code, parse_front_matter,
    split_front_matter, WikiLink,
};
use crate::paths;

/// A parsed note: raw content plus everything the UI needs to show it.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    /// Vault-relative path.
    pub path: String,
    /// `title` property, first `# Heading` or the file name.
    pub title: String,
    /// Full file content, front matter included.
    pub content: String,
    /// Parsed front matter, if present and valid.
    pub front_matter: Option<Value>,
    /// YAML error message when the front matter is invalid.
    pub front_matter_error: Option<String>,
    pub links: Vec<WikiLink>,
    /// Tags from the `tags` property followed by inline `#tags`.
    pub tags: Vec<String>,
}

impl Note {
    pub fn parse(path: &str, content: String) -> Note {
        let (yaml, body) = split_front_matter(&content);
        let (front_matter, front_matter_error) = match yaml.map(parse_front_matter) {
            Some(Ok(value)) => (Some(value), None),
            Some(Err(err)) => (None, Some(err)),
            None => (None, None),
        };
        let masked = mask_code(body);

        let title = front_matter
            .as_ref()
            .and_then(|fm| fm.get("title"))
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|t| !t.is_empty())
            .map(str::to_string)
            .or_else(|| extract_title(body, &masked))
            .unwrap_or_else(|| paths::file_stem(path).to_string());

        let mut tags = front_matter
            .as_ref()
            .map(front_matter_tags)
            .unwrap_or_default();
        for tag in extract_inline_tags(&masked) {
            if !tags.contains(&tag) {
                tags.push(tag);
            }
        }
        let links = extract_wikilinks(&masked);

        Note {
            path: path.to_string(),
            title,
            content,
            front_matter,
            front_matter_error,
            links,
            tags,
        }
    }
}

/// Reads the `tags` property: a YAML list or a comma/space separated string.
fn front_matter_tags(fm: &Value) -> Vec<String> {
    let raw: Vec<&str> = match fm.get("tags") {
        Some(Value::Array(items)) => items.iter().filter_map(Value::as_str).collect(),
        Some(Value::String(s)) => s.split([',', ' ']).collect(),
        _ => Vec::new(),
    };
    let mut tags: Vec<String> = Vec::new();
    for tag in raw {
        let tag = tag.trim().trim_start_matches('#');
        if !tag.is_empty() && !tags.iter().any(|t| t == tag) {
            tags.push(tag.to_string());
        }
    }
    tags
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_full_note() {
        let src = concat!(
            "---\nid: 01J\ntitle: Заголовок\ntags: [project, '#rust']\n---\n",
            "# Інший\nДив. [[Ідея]] #project #new\n"
        );
        let note = Note::parse("dir/file.md", src.to_string());
        assert_eq!(note.title, "Заголовок");
        assert_eq!(note.tags, vec!["project", "rust", "new"]);
        assert_eq!(note.links.len(), 1);
        assert_eq!(note.links[0].target, "Ідея");
        assert_eq!(note.front_matter.unwrap()["id"], "01J");
        assert!(note.front_matter_error.is_none());
    }

    #[test]
    fn falls_back_to_heading_and_file_name() {
        assert_eq!(Note::parse("a.md", "# Heading\n".into()).title, "Heading");
        assert_eq!(
            Note::parse("dir/My note.md", "text".into()).title,
            "My note"
        );
    }

    #[test]
    fn reports_invalid_front_matter() {
        let note = Note::parse("a.md", "---\ntags: [oops\n---\nbody".into());
        assert!(note.front_matter.is_none());
        assert!(note.front_matter_error.is_some());
    }

    #[test]
    fn reads_string_tags() {
        let note = Note::parse("a.md", "---\ntags: one, two three\n---\n".into());
        assert_eq!(note.tags, vec!["one", "two", "three"]);
    }
}
