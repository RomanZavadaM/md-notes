use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::{io_err, Error, Result};
use crate::note::Note;
use crate::paths;

/// Service folder inside every vault.
pub const SERVICE_DIR: &str = ".mdnotes";

/// `.mdnotes/config.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct VaultConfig {
    /// Format version of this file.
    pub version: u32,
    /// Display name; the folder name is used when absent.
    pub name: Option<String>,
    /// Folder for attachments, relative to the vault root.
    pub attachments_dir: String,
    /// Folder for daily notes, relative to the vault root.
    pub daily_notes_dir: String,
}

impl Default for VaultConfig {
    fn default() -> Self {
        Self {
            version: 1,
            name: None,
            attachments_dir: "attachments".into(),
            daily_notes_dir: "daily".into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum EntryKind {
    Dir,
    Note,
    File,
}

/// A node of the vault file tree.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TreeEntry {
    pub name: String,
    pub path: String,
    pub kind: EntryKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<TreeEntry>>,
}

/// A knowledge base stored in a local folder.
#[derive(Debug, Clone)]
pub struct Vault {
    root: PathBuf,
    config: VaultConfig,
}

impl Vault {
    /// Opens an existing folder as a vault. The folder is not modified.
    pub fn open(root: impl AsRef<Path>) -> Result<Self> {
        let root = root.as_ref().to_path_buf();
        if !root.is_dir() {
            return Err(Error::NotFound(root.display().to_string()));
        }
        let config_path = root.join(SERVICE_DIR).join("config.json");
        let config = if config_path.is_file() {
            let raw = fs::read_to_string(&config_path).map_err(io_err(&config_path))?;
            serde_json::from_str(&raw).map_err(|e| Error::Config(e.to_string()))?
        } else {
            VaultConfig::default()
        };
        Ok(Self { root, config })
    }

    /// Creates the service folder (if missing) and opens the vault.
    pub fn init(root: impl AsRef<Path>, name: Option<String>) -> Result<Self> {
        let root = root.as_ref();
        let service = root.join(SERVICE_DIR);
        let templates = service.join("templates");
        fs::create_dir_all(&templates).map_err(io_err(&templates))?;

        let config_path = service.join("config.json");
        if !config_path.exists() {
            let config = VaultConfig {
                name,
                ..VaultConfig::default()
            };
            let json = serde_json::to_string_pretty(&config).expect("config serializes") + "\n";
            write_atomic(&config_path, json.as_bytes())?;
        }
        // The index cache and the trash are local state: keep them out of Git
        // and out of sync.
        let ignore = service.join(".gitignore");
        if !ignore.exists() {
            write_atomic(&ignore, SERVICE_GITIGNORE)?;
        }
        Self::open(root)
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn config(&self) -> &VaultConfig {
        &self.config
    }

    /// `.mdnotes/cache`, created on demand. Also makes sure the service
    /// folder has a `.gitignore` that keeps local state out of Git.
    pub fn cache_dir(&self) -> Result<PathBuf> {
        let service = self.root.join(SERVICE_DIR);
        let cache = service.join("cache");
        fs::create_dir_all(&cache).map_err(io_err(&cache))?;
        let ignore = service.join(".gitignore");
        if !ignore.exists() {
            write_atomic(&ignore, SERVICE_GITIGNORE)?;
        }
        Ok(cache)
    }

    /// Config name or the folder name.
    pub fn display_name(&self) -> String {
        self.config.name.clone().unwrap_or_else(|| {
            self.root
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| self.root.display().to_string())
        })
    }

    /// Full file tree. Hidden entries (starting with `.`) are skipped, which
    /// also hides `.mdnotes` and `.git`. Folders come first, then files, both
    /// sorted case-insensitively.
    pub fn tree(&self) -> Result<Vec<TreeEntry>> {
        self.read_dir("")
    }

    fn read_dir(&self, rel: &str) -> Result<Vec<TreeEntry>> {
        let dir = paths::resolve(&self.root, rel)?;
        let mut entries = Vec::new();
        for item in fs::read_dir(&dir).map_err(io_err(&dir))? {
            let item = item.map_err(io_err(&dir))?;
            let name = item.file_name().to_string_lossy().into_owned();
            if name.starts_with('.') || name.ends_with(TMP_SUFFIX) {
                continue;
            }
            let path = paths::join(rel, &name);
            let file_type = item.file_type().map_err(io_err(&item.path()))?;
            if file_type.is_dir() {
                let children = self.read_dir(&path)?;
                entries.push(TreeEntry {
                    name,
                    path,
                    kind: EntryKind::Dir,
                    children: Some(children),
                });
            } else if file_type.is_file() {
                let kind = if paths::is_note(&name) {
                    EntryKind::Note
                } else {
                    EntryKind::File
                };
                entries.push(TreeEntry {
                    name,
                    path,
                    kind,
                    children: None,
                });
            }
        }
        entries.sort_by(|a, b| {
            (a.kind != EntryKind::Dir)
                .cmp(&(b.kind != EntryKind::Dir))
                .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
        });
        Ok(entries)
    }

    /// Paths of all notes in the vault.
    pub fn note_paths(&self) -> Result<Vec<String>> {
        fn collect(entries: &[TreeEntry], out: &mut Vec<String>) {
            for entry in entries {
                match entry.kind {
                    EntryKind::Note => out.push(entry.path.clone()),
                    EntryKind::Dir => collect(entry.children.as_deref().unwrap_or_default(), out),
                    EntryKind::File => {}
                }
            }
        }
        let mut out = Vec::new();
        collect(&self.tree()?, &mut out);
        Ok(out)
    }

    /// Finds the note a `[[target]]` points to: first by path, then by file
    /// name, case-insensitively. Aliases are resolved by the index (v0.2).
    pub fn resolve_link(&self, target: &str) -> Result<Option<String>> {
        let wanted = paths::normalize(target.trim())?.to_lowercase();
        if wanted.is_empty() {
            return Ok(None);
        }
        let wanted_stem = paths::file_stem(&wanted).to_string();
        let wanted_has_ext = paths::is_note(&wanted);
        let notes = self.note_paths()?;

        let by_path = notes.iter().find(|p| {
            let lower = p.to_lowercase();
            lower == wanted || (!wanted_has_ext && lower == format!("{wanted}.md"))
        });
        if let Some(path) = by_path {
            return Ok(Some(path.clone()));
        }
        if wanted.contains('/') {
            return Ok(None);
        }
        let stem = if wanted_has_ext {
            wanted_stem.as_str()
        } else {
            wanted.as_str()
        };
        Ok(notes
            .into_iter()
            .find(|p| paths::file_stem(p).to_lowercase() == stem))
    }

    pub fn read_note(&self, rel: &str) -> Result<Note> {
        let rel = note_path(rel)?;
        let path = paths::resolve(&self.root, &rel)?;
        if !path.is_file() {
            return Err(Error::NotFound(rel));
        }
        let content = fs::read_to_string(&path).map_err(io_err(&path))?;
        Ok(Note::parse(&rel, content))
    }

    /// Saves a note atomically (write to a temp file, then rename), creating
    /// parent folders when needed.
    pub fn write_note(&self, rel: &str, content: &str) -> Result<Note> {
        let rel = note_path(rel)?;
        let path = paths::resolve(&self.root, &rel)?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(io_err(parent))?;
        }
        write_atomic(&path, content.as_bytes())?;
        Ok(Note::parse(&rel, content.to_string()))
    }

    /// Creates a new note in `dir` with a unique file name derived from
    /// `title`, a stable `id` and the creation date.
    pub fn create_note(&self, dir: &str, title: &str) -> Result<Note> {
        let dir = paths::normalize(dir)?;
        let title = title.trim();
        let title = if title.is_empty() {
            "Без назви"
        } else {
            title
        };
        let base = match paths::sanitize_file_name(title) {
            name if name.is_empty() => "Без назви".to_string(),
            name => name,
        };
        let rel = self.unique_path(&dir, &base, ".md");
        self.write_note(&rel, &new_note_content(title))
    }

    /// Creates a folder with a unique name and returns its path.
    pub fn create_folder(&self, parent: &str, name: &str) -> Result<String> {
        let parent = paths::normalize(parent)?;
        let base = paths::sanitize_file_name(name);
        if base.is_empty() {
            return Err(Error::InvalidPath(name.to_string()));
        }
        let rel = self.unique_path(&parent, &base, "");
        let path = paths::resolve(&self.root, &rel)?;
        fs::create_dir_all(&path).map_err(io_err(&path))?;
        Ok(rel)
    }

    /// Renames or moves a note, file or folder. Links are not rewritten yet
    /// (planned for v0.2 together with the index).
    pub fn rename(&self, from: &str, to: &str) -> Result<String> {
        let from = paths::normalize(from)?;
        let to = paths::normalize(to)?;
        if from.is_empty() || to.is_empty() {
            return Err(Error::InvalidPath("vault root".into()));
        }
        let source = paths::resolve(&self.root, &from)?;
        let target = paths::resolve(&self.root, &to)?;
        if !source.exists() {
            return Err(Error::NotFound(from));
        }
        // Allow changing only the letter case of a name on case-insensitive
        // file systems.
        if target.exists() && from.to_lowercase() != to.to_lowercase() {
            return Err(Error::AlreadyExists(to));
        }
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).map_err(io_err(parent))?;
        }
        fs::rename(&source, &target).map_err(io_err(&source))?;
        Ok(to)
    }

    /// Moves an entry to `.mdnotes/trash/<timestamp>/` instead of deleting it.
    pub fn move_to_trash(&self, rel: &str) -> Result<String> {
        let rel = paths::normalize(rel)?;
        if rel.is_empty() {
            return Err(Error::InvalidPath("vault root".into()));
        }
        let source = paths::resolve(&self.root, &rel)?;
        if !source.exists() {
            return Err(Error::NotFound(rel));
        }
        let stamp = chrono::Local::now().format("%Y%m%d-%H%M%S%3f").to_string();
        let trash_rel = format!("{SERVICE_DIR}/trash/{stamp}/{rel}");
        let target = paths::resolve(&self.root, &trash_rel)?;
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).map_err(io_err(parent))?;
        }
        fs::rename(&source, &target).map_err(io_err(&source))?;
        Ok(trash_rel)
    }

    fn unique_path(&self, dir: &str, base: &str, ext: &str) -> String {
        let mut n = 0;
        loop {
            let name = if n == 0 {
                format!("{base}{ext}")
            } else {
                format!("{base} {n}{ext}")
            };
            let rel = paths::join(dir, &name);
            if !self.root.join(&rel).exists() {
                return rel;
            }
            n += 1;
        }
    }
}

const TMP_SUFFIX: &str = ".mdnotes-tmp";
const SERVICE_GITIGNORE: &[u8] = b"cache/\ntrash/\n";

fn note_path(rel: &str) -> Result<String> {
    let norm = paths::normalize(rel)?;
    if !paths::is_note(&norm) {
        return Err(Error::InvalidPath(rel.to_string()));
    }
    Ok(norm)
}

/// Content of a freshly created note.
pub fn new_note_content(title: &str) -> String {
    let id = ulid::Ulid::new().to_string();
    let today = chrono::Local::now().format("%Y-%m-%d");
    format!("---\nid: {id}\ntype: note\ncreated: {today}\n---\n\n# {title}\n\n")
}

fn write_atomic(path: &Path, bytes: &[u8]) -> Result<()> {
    let mut tmp = path.as_os_str().to_owned();
    tmp.push(TMP_SUFFIX);
    let tmp = PathBuf::from(tmp);
    fs::write(&tmp, bytes).map_err(io_err(&tmp))?;
    fs::rename(&tmp, path).map_err(io_err(path))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vault() -> (tempfile::TempDir, Vault) {
        let dir = tempfile::tempdir().unwrap();
        let vault = Vault::init(dir.path(), Some("Test".into())).unwrap();
        (dir, vault)
    }

    #[test]
    fn init_creates_service_folder() {
        let (dir, vault) = vault();
        assert!(dir.path().join(".mdnotes/config.json").is_file());
        assert!(dir.path().join(".mdnotes/templates").is_dir());
        assert_eq!(vault.display_name(), "Test");
        assert_eq!(vault.config().attachments_dir, "attachments");
    }

    #[test]
    fn open_reads_existing_config() {
        let (dir, _) = vault();
        let reopened = Vault::open(dir.path()).unwrap();
        assert_eq!(reopened.config().name.as_deref(), Some("Test"));
        assert!(Vault::open(dir.path().join("missing")).is_err());
    }

    #[test]
    fn builds_sorted_tree_without_hidden_entries() {
        let (dir, vault) = vault();
        fs::create_dir_all(dir.path().join("b-folder")).unwrap();
        fs::create_dir_all(dir.path().join(".git")).unwrap();
        fs::write(dir.path().join("A note.md"), "# A").unwrap();
        fs::write(dir.path().join("b-folder/inner.md"), "x").unwrap();
        fs::write(dir.path().join("image.png"), [0u8]).unwrap();

        let tree = vault.tree().unwrap();
        let names: Vec<_> = tree.iter().map(|e| e.name.as_str()).collect();
        assert_eq!(names, vec!["b-folder", "A note.md", "image.png"]);
        assert_eq!(
            tree[0].children.as_ref().unwrap()[0].path,
            "b-folder/inner.md"
        );
        assert_eq!(tree[2].kind, EntryKind::File);
        assert_eq!(
            vault.note_paths().unwrap(),
            vec!["b-folder/inner.md", "A note.md"]
        );
    }

    #[test]
    fn writes_and_reads_notes() {
        let (dir, vault) = vault();
        let saved = vault
            .write_note("new/dir/Note.md", "# Hello\n[[Other]]")
            .unwrap();
        assert_eq!(saved.title, "Hello");
        assert!(dir.path().join("new/dir/Note.md").is_file());
        let read = vault.read_note(r"new\dir\Note.md").unwrap();
        assert_eq!(read.path, "new/dir/Note.md");
        assert_eq!(read.links[0].target, "Other");
    }

    #[test]
    fn rejects_unsafe_and_non_note_paths() {
        let (_dir, vault) = vault();
        assert!(vault.write_note("../outside.md", "x").is_err());
        assert!(vault.write_note("file.txt", "x").is_err());
        assert!(matches!(
            vault.read_note("missing.md"),
            Err(Error::NotFound(_))
        ));
    }

    #[test]
    fn creates_notes_with_unique_names() {
        let (_dir, vault) = vault();
        let first = vault.create_note("", "Ідея: нова").unwrap();
        let second = vault.create_note("", "Ідея: нова").unwrap();
        assert_eq!(first.path, "Ідея нова.md");
        assert_eq!(second.path, "Ідея нова 1.md");
        assert_eq!(first.title, "Ідея: нова");
        let fm = first.front_matter.unwrap();
        assert_eq!(fm["type"], "note");
        assert_eq!(fm["id"].as_str().unwrap().len(), 26);
    }

    #[test]
    fn creates_folders_renames_and_trashes() {
        let (dir, vault) = vault();
        let folder = vault.create_folder("", "Проєкти").unwrap();
        assert_eq!(folder, "Проєкти");
        vault.write_note("a.md", "a").unwrap();

        let moved = vault.rename("a.md", "Проєкти/b.md").unwrap();
        assert_eq!(moved, "Проєкти/b.md");
        assert!(dir.path().join("Проєкти/b.md").is_file());

        vault.write_note("c.md", "c").unwrap();
        assert!(matches!(
            vault.rename("c.md", "Проєкти/b.md"),
            Err(Error::AlreadyExists(_))
        ));

        let trashed = vault.move_to_trash("Проєкти").unwrap();
        assert!(trashed.starts_with(".mdnotes/trash/"));
        assert!(!dir.path().join("Проєкти").exists());
        assert!(dir.path().join(&trashed).join("b.md").is_file());
    }

    #[test]
    fn resolves_links() {
        let (_dir, vault) = vault();
        vault.write_note("Projects/MD Notes.md", "x").unwrap();
        vault.write_note("Ideas.md", "x").unwrap();
        let resolve = |target: &str| vault.resolve_link(target).unwrap();
        let md_notes = Some("Projects/MD Notes.md");
        assert_eq!(resolve("md notes").as_deref(), md_notes);
        assert_eq!(resolve("Projects/MD Notes").as_deref(), md_notes);
        assert_eq!(resolve("Ideas.md").as_deref(), Some("Ideas.md"));
        assert_eq!(resolve("Missing"), None);
        assert_eq!(resolve("Other/Ideas"), None);
    }
}
