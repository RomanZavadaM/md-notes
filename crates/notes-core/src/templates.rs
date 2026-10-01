//! Note templates (`.mdnotes/templates/*.md`) and daily notes.
//!
//! Templates are plain Markdown files with placeholders:
//! `{{title}}`, `{{date}}` (`YYYY-MM-DD`), `{{time}}` (`HH:MM`) and `{{id}}`
//! (a new ULID).

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use chrono::{Local, NaiveDate};
use serde::Serialize;
use serde_json::Value;

use crate::error::{io_err, Error, Result};
use crate::note::Note;
use crate::paths;
use crate::vault::{Vault, SERVICE_DIR};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateInfo {
    /// File name without extension, e.g. `task`.
    pub name: String,
    /// Human-readable name: the type label from `schema.json` or the name.
    pub label: String,
}

impl Vault {
    fn templates_dir(&self) -> PathBuf {
        self.root().join(SERVICE_DIR).join("templates")
    }

    /// Available templates sorted by label.
    pub fn templates(&self) -> Result<Vec<TemplateInfo>> {
        let dir = self.templates_dir();
        if !dir.is_dir() {
            return Ok(Vec::new());
        }
        let labels = self.type_labels();
        let mut out = Vec::new();
        for entry in fs::read_dir(&dir).map_err(io_err(&dir))? {
            let entry = entry.map_err(io_err(&dir))?;
            let file = entry.file_name().to_string_lossy().into_owned();
            if !paths::is_note(&file) || !entry.path().is_file() {
                continue;
            }
            let name = paths::file_stem(&file).to_string();
            let label = labels.get(&name).cloned().unwrap_or_else(|| name.clone());
            out.push(TemplateInfo { name, label });
        }
        out.sort_by_key(|t| t.label.to_lowercase());
        Ok(out)
    }

    /// `types.<name>.label` values from `.mdnotes/schema.json`.
    fn type_labels(&self) -> HashMap<String, String> {
        let path = self.root().join(SERVICE_DIR).join("schema.json");
        let schema: Option<Value> = fs::read_to_string(path)
            .ok()
            .and_then(|raw| serde_json::from_str(&raw).ok());
        let mut labels = HashMap::new();
        if let Some(Value::Object(types)) = schema.as_ref().and_then(|s| s.get("types")) {
            for (name, spec) in types {
                if let Some(label) = spec.get("label").and_then(Value::as_str) {
                    labels.insert(name.clone(), label.to_string());
                }
            }
        }
        labels
    }

    fn read_template(&self, name: &str) -> Result<String> {
        let file = paths::sanitize_file_name(name);
        let path = self.templates_dir().join(format!("{file}.md"));
        if file.is_empty() || !path.is_file() {
            return Err(Error::NotFound(format!("template {name}")));
        }
        fs::read_to_string(&path).map_err(io_err(&path))
    }

    /// Creates a note in `dir` from the template `template`.
    pub fn create_from_template(&self, dir: &str, title: &str, template: &str) -> Result<Note> {
        let body = self.read_template(template)?;
        let (rel, title) = self.new_note_path(dir, title)?;
        let content = render_template(&body, &title, Local::now().date_naive());
        self.write_note(&rel, &content)
    }

    /// Opens the daily note for `date`, creating it from the `daily`
    /// template (or a minimal default) when it does not exist yet.
    pub fn open_daily(&self, date: NaiveDate) -> Result<Note> {
        let day = date.format("%Y-%m-%d").to_string();
        let dir = paths::normalize(&self.config().daily_notes_dir)?;
        let rel = paths::join(&dir, &format!("{day}.md"));
        if paths::resolve(self.root(), &rel)?.is_file() {
            return self.read_note(&rel);
        }
        let template = self
            .read_template("daily")
            .unwrap_or_else(|_| DEFAULT_DAILY.to_string());
        self.write_note(&rel, &render_template(&template, &day, date))
    }
}

const DEFAULT_DAILY: &str = "---\nid: {{id}}\ntype: daily\ndate: {{date}}\n---\n\n# {{date}}\n\n";

/// Fills template placeholders.
pub fn render_template(template: &str, title: &str, date: NaiveDate) -> String {
    template
        .replace("{{id}}", &ulid::Ulid::new().to_string())
        .replace("{{title}}", title)
        .replace("{{date}}", &date.format("%Y-%m-%d").to_string())
        .replace("{{time}}", &Local::now().format("%H:%M").to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vault_with_templates() -> (tempfile::TempDir, Vault) {
        let dir = tempfile::tempdir().unwrap();
        let vault = Vault::init(dir.path(), None).unwrap();
        let templates = dir.path().join(".mdnotes/templates");
        let task = "---\nid: \"{{id}}\"\ntype: task\ncreated: {{date}}\n---\n\n# {{title}}\n";
        fs::write(templates.join("task.md"), task).unwrap();
        fs::write(templates.join("daily.md"), "---\ntype: daily\n---\n# День {{date}}\n").unwrap();
        fs::write(templates.join("notes.txt"), "ignored").unwrap();
        let schema = r#"{"types": {"task": {"label": "Задача"}}}"#;
        fs::write(dir.path().join(".mdnotes/schema.json"), schema).unwrap();
        (dir, vault)
    }

    fn day(s: &str) -> NaiveDate {
        NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap()
    }

    #[test]
    fn lists_templates_with_labels() {
        let (_dir, vault) = vault_with_templates();
        let templates = vault.templates().unwrap();
        let names: Vec<_> = templates.iter().map(|t| t.label.as_str()).collect();
        assert_eq!(names, vec!["daily", "Задача"]);
        assert_eq!(templates[1].name, "task");
    }

    #[test]
    fn creates_notes_from_templates() {
        let (_dir, vault) = vault_with_templates();
        let note = vault.create_from_template("Задачі", "Купити: молоко", "task").unwrap();
        assert_eq!(note.path, "Задачі/Купити молоко.md");
        assert_eq!(note.title, "Купити: молоко");
        let fm = note.front_matter.unwrap();
        assert_eq!(fm["type"], "task");
        assert_eq!(fm["id"].as_str().unwrap().len(), 26);
        assert!(!note.content.contains("{{"));
        assert!(matches!(
            vault.create_from_template("", "x", "missing"),
            Err(Error::NotFound(_))
        ));
    }

    #[test]
    fn opens_or_creates_daily_notes() {
        let (_dir, vault) = vault_with_templates();
        let created = vault.open_daily(day("2026-10-01")).unwrap();
        assert_eq!(created.path, "daily/2026-10-01.md");
        assert!(created.content.contains("# День 2026-10-01"));

        vault.write_note(&created.path, "edited").unwrap();
        let again = vault.open_daily(day("2026-10-01")).unwrap();
        assert_eq!(again.content, "edited");
    }

    #[test]
    fn daily_notes_work_without_templates() {
        let dir = tempfile::tempdir().unwrap();
        let vault = Vault::open(dir.path()).unwrap();
        let note = vault.open_daily(day("2026-01-31")).unwrap();
        assert_eq!(note.title, "2026-01-31");
        assert_eq!(note.front_matter.unwrap()["type"], "daily");
    }

    #[test]
    fn renders_placeholders() {
        let out = render_template("{{title}} {{date}} {{title}}", "T", day("2026-02-03"));
        assert_eq!(out, "T 2026-02-03 T");
    }
}
