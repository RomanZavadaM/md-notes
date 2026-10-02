//! Open note-type schema stored in `.mdnotes/schema.json`.
//!
//! The schema only describes top-level YAML front-matter fields. Notes remain
//! ordinary Markdown files and unknown front-matter properties are preserved.

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::error::{io_err, Error, Result};
use crate::markdown::{parse_front_matter, split_front_matter};
use crate::note::Note;
use crate::vault::{Vault, SERVICE_DIR};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaDocument {
    #[serde(default = "schema_version")]
    pub version: u32,
    #[serde(default)]
    pub types: BTreeMap<String, NoteTypeSpec>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteTypeSpec {
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub template: Option<String>,
    #[serde(default)]
    pub properties: BTreeMap<String, PropertySpec>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertySpec {
    #[serde(default)]
    pub label: Option<String>,
    #[serde(rename = "type")]
    pub kind: PropertyKind,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub options: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PropertyKind {
    String,
    Text,
    Number,
    Boolean,
    Date,
    Select,
    Tags,
}

fn schema_version() -> u32 {
    1
}

impl Default for PropertyKind {
    fn default() -> Self {
        Self::String
    }
}

impl Vault {
    fn schema_path(&self) -> PathBuf {
        self.root().join(SERVICE_DIR).join("schema.json")
    }

    /// Reads `.mdnotes/schema.json`. A missing file means an empty schema.
    pub fn schema(&self) -> Result<SchemaDocument> {
        let path = self.schema_path();
        if !path.is_file() {
            return Ok(SchemaDocument {
                version: schema_version(),
                types: BTreeMap::new(),
            });
        }
        let raw = fs::read_to_string(&path).map_err(io_err(&path))?;
        let schema: SchemaDocument = serde_json::from_str(&raw)
            .map_err(|e| Error::Config(format!("invalid schema.json: {e}")))?;
        if schema.version != schema_version() {
            return Err(Error::Config(format!(
                "unsupported schema.json version: {}",
                schema.version
            )));
        }
        validate_schema(&schema)?;
        Ok(schema)
    }

    /// Applies top-level property changes to Markdown content without touching
    /// the file system. This lets the editor preserve unsaved body changes and
    /// keep its normal autosave path as the only writer.
    pub fn format_note_properties(
        &self,
        content: &str,
        patch: &BTreeMap<String, Value>,
    ) -> Result<String> {
        let parsed = Note::parse("note.md", content.to_string());
        if let Some(error) = &parsed.front_matter_error {
            return Err(Error::Config(format!(
                "cannot edit properties while front matter is invalid: {error}"
            )));
        }

        let (yaml, body) = split_front_matter(content);
        let mut object = match yaml {
            Some(raw) => match parse_front_matter(raw).map_err(Error::Config)? {
                Value::Object(map) => map,
                _ => {
                    return Err(Error::Config(
                        "front matter must be a YAML object to use the property form".into(),
                    ))
                }
            },
            None => Map::new(),
        };

        for (key, value) in patch {
            if value.is_null() {
                object.remove(key);
            } else {
                object.insert(key.clone(), value.clone());
            }
        }

        let schema = self.schema()?;
        validate_note_properties(&schema, &object)?;

        let yaml = serde_yaml::to_string(&Value::Object(object))
            .map_err(|e| Error::Config(format!("cannot serialize front matter: {e}")))?;
        Ok(format!(
            "---\n{}---\n{}",
            yaml.trim_start_matches("---\n"),
            body
        ))
    }

    /// File-writing variant used by non-editor callers.
    pub fn update_note_properties(
        &self,
        path: &str,
        patch: &BTreeMap<String, Value>,
    ) -> Result<Note> {
        let note = self.read_note(path)?;
        let content = self.format_note_properties(&note.content, patch)?;
        self.write_note(path, &content)
    }
}

fn validate_schema(schema: &SchemaDocument) -> Result<()> {
    for (name, note_type) in &schema.types {
        if name.trim().is_empty() {
            return Err(Error::Config("schema type name cannot be empty".into()));
        }
        for (property, spec) in &note_type.properties {
            if property.trim().is_empty() {
                return Err(Error::Config(format!(
                    "schema property name in type {name} cannot be empty"
                )));
            }
            if spec.kind == PropertyKind::Select && spec.options.is_empty() {
                return Err(Error::Config(format!(
                    "select property {name}.{property} must define options"
                )));
            }
        }
    }
    Ok(())
}

fn validate_note_properties(schema: &SchemaDocument, object: &Map<String, Value>) -> Result<()> {
    let Some(type_name) = object.get("type").and_then(Value::as_str) else {
        return Ok(());
    };
    let Some(note_type) = schema.types.get(type_name) else {
        // Unknown types are allowed so existing/open YAML data is never locked out.
        return Ok(());
    };

    for (name, spec) in &note_type.properties {
        match object.get(name) {
            Some(value) => validate_value(type_name, name, spec, value)?,
            None if spec.required => {
                return Err(Error::Config(format!(
                    "required property is missing: {type_name}.{name}"
                )))
            }
            None => {}
        }
    }
    Ok(())
}

fn validate_value(type_name: &str, name: &str, spec: &PropertySpec, value: &Value) -> Result<()> {
    let valid = match spec.kind {
        PropertyKind::String | PropertyKind::Text | PropertyKind::Date | PropertyKind::Select => {
            value.is_string()
        }
        PropertyKind::Number => value.is_number(),
        PropertyKind::Boolean => value.is_boolean(),
        PropertyKind::Tags => {
            value.is_string()
                || value
                    .as_array()
                    .is_some_and(|items| items.iter().all(Value::is_string))
        }
    };
    if !valid {
        return Err(Error::Config(format!(
            "invalid value type for {type_name}.{name}"
        )));
    }
    if spec.kind == PropertyKind::Select {
        let selected = value.as_str().unwrap_or_default();
        if !spec.options.iter().any(|option| option == selected) {
            return Err(Error::Config(format!(
                "invalid option for {type_name}.{name}: {selected}"
            )));
        }
    }
    if spec.required && value.as_str().is_some_and(|s| s.trim().is_empty()) {
        return Err(Error::Config(format!(
            "required property is empty: {type_name}.{name}"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn schema_json() -> &'static str {
        r#"{
  "version": 1,
  "types": {
    "task": {
      "label": "Задача",
      "template": "task",
      "properties": {
        "status": {"type": "select", "required": true, "options": ["todo", "done"]},
        "due": {"type": "date"},
        "estimate": {"type": "number"},
        "done": {"type": "boolean"},
        "tags": {"type": "tags"}
      }
    }
  }
}"#
    }

    fn vault_with_schema() -> (tempfile::TempDir, Vault) {
        let dir = tempfile::tempdir().unwrap();
        let vault = Vault::init(dir.path(), None).unwrap();
        fs::write(dir.path().join(".mdnotes/schema.json"), schema_json()).unwrap();
        (dir, vault)
    }

    #[test]
    fn reads_open_schema() {
        let (_dir, vault) = vault_with_schema();
        let schema = vault.schema().unwrap();
        let task = &schema.types["task"];
        assert_eq!(task.label.as_deref(), Some("Задача"));
        assert_eq!(task.template.as_deref(), Some("task"));
        assert_eq!(task.properties["status"].kind, PropertyKind::Select);
    }

    #[test]
    fn missing_schema_is_empty() {
        let dir = tempfile::tempdir().unwrap();
        let vault = Vault::open(dir.path()).unwrap();
        let schema = vault.schema().unwrap();
        assert_eq!(schema.version, 1);
        assert!(schema.types.is_empty());
    }

    #[test]
    fn formats_properties_without_dropping_unknown_fields_body_or_unsaved_text() {
        let (_dir, vault) = vault_with_schema();
        let content = "---\ntype: task\nstatus: todo\ncustom: keep\n---\n# Body\nunsaved text\n";
        let patch = BTreeMap::from([
            ("status".into(), Value::String("done".into())),
            ("due".into(), Value::String("2026-10-02".into())),
        ]);
        let formatted = vault.format_note_properties(content, &patch).unwrap();
        let note = Note::parse("Task.md", formatted);
        let fm = note.front_matter.unwrap();
        assert_eq!(fm["status"], "done");
        assert_eq!(fm["custom"], "keep");
        assert_eq!(fm["due"], "2026-10-02");
        assert!(note.content.ends_with("# Body\nunsaved text\n"));
    }

    #[test]
    fn can_add_front_matter_to_plain_content() {
        let (_dir, vault) = vault_with_schema();
        let patch = BTreeMap::from([
            ("type".into(), Value::String("task".into())),
            ("status".into(), Value::String("todo".into())),
        ]);
        let formatted = vault.format_note_properties("# Body\n", &patch).unwrap();
        let note = Note::parse("Task.md", formatted);
        assert_eq!(note.front_matter.unwrap()["type"], "task");
        assert!(note.content.ends_with("# Body\n"));
    }

    #[test]
    fn rejects_invalid_select_and_required_removal() {
        let (_dir, vault) = vault_with_schema();
        let content = "---\ntype: task\nstatus: todo\n---\n";
        let bad = BTreeMap::from([("status".into(), Value::String("maybe".into()))]);
        assert!(vault.format_note_properties(content, &bad).is_err());
        let remove = BTreeMap::from([("status".into(), Value::Null)]);
        assert!(vault.format_note_properties(content, &remove).is_err());
    }

    #[test]
    fn refuses_to_overwrite_invalid_yaml() {
        let (_dir, vault) = vault_with_schema();
        let patch = BTreeMap::from([("title".into(), Value::String("x".into()))]);
        assert!(vault
            .format_note_properties("---\ntags: [oops\n---\nbody\n", &patch)
            .is_err());
    }
}
