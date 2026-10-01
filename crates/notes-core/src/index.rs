//! SQLite index of a vault.
//!
//! The index makes backlinks, full-text search, tags and alias resolution
//! fast. It is a cache, not a source of truth: it can be deleted at any time
//! and is rebuilt from the files by [`Index::sync`] (see ADR-0002).

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::time::UNIX_EPOCH;

use rusqlite::{params, Connection, Transaction};
use serde::Serialize;
use serde_json::Value;

use crate::error::{io_err, Result};
use crate::markdown::{
    extract_wikilinks, extract_wikilinks_with_context, mask_code, split_front_matter,
};
use crate::note::Note;
use crate::paths;
use crate::vault::Vault;

/// Bump when the schema changes: the index is then rebuilt from scratch.
const SCHEMA_VERSION: i64 = 1;

const SCHEMA: &str = "
CREATE TABLE notes (
    path TEXT PRIMARY KEY,
    id TEXT,
    title TEXT NOT NULL,
    type TEXT,
    mtime INTEGER NOT NULL,
    size INTEGER NOT NULL
);
CREATE TABLE links (
    source TEXT NOT NULL,
    target TEXT NOT NULL,
    heading TEXT,
    context TEXT NOT NULL,
    resolved TEXT
);
CREATE INDEX links_source ON links(source);
CREATE INDEX links_resolved ON links(resolved);
CREATE TABLE tags (path TEXT NOT NULL, tag TEXT NOT NULL);
CREATE INDEX tags_path ON tags(path);
CREATE INDEX tags_tag ON tags(tag);
CREATE TABLE aliases (path TEXT NOT NULL, alias TEXT NOT NULL);
CREATE INDEX aliases_path ON aliases(path);
CREATE VIRTUAL TABLE fts USING fts5(path UNINDEXED, title, body);
";

const DROP_ALL: &str = "
DROP TABLE IF EXISTS notes;
DROP TABLE IF EXISTS links;
DROP TABLE IF EXISTS tags;
DROP TABLE IF EXISTS aliases;
DROP TABLE IF EXISTS fts;
";

/// Markers around matched words in [`SearchHit::snippet`].
pub const MATCH_START: char = '\u{2}';
pub const MATCH_END: char = '\u{3}';

/// Short description of a note for lists, the quick switcher and filters.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteSummary {
    pub path: String,
    pub title: String,
    pub note_type: Option<String>,
    pub tags: Vec<String>,
    pub aliases: Vec<String>,
}

/// A note that links to another note.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Backlink {
    pub path: String,
    pub title: String,
    /// The line containing the link.
    pub context: String,
}

/// A link that does not point to an existing note.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UnresolvedLink {
    pub source: String,
    pub target: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchHit {
    pub path: String,
    pub title: String,
    /// Text fragment; matches are wrapped in [`MATCH_START`]/[`MATCH_END`].
    pub snippet: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TagCount {
    pub tag: String,
    pub count: u32,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncStats {
    pub added: u32,
    pub updated: u32,
    pub removed: u32,
}

/// Resolves `[[targets]]` to note paths: by path, then by file name, then
/// by alias. All comparisons ignore case.
#[derive(Debug, Default)]
struct Resolver {
    by_path: HashMap<String, String>,
    by_stem: HashMap<String, String>,
    by_alias: HashMap<String, String>,
}

impl Resolver {
    fn resolve(&self, target: &str) -> Option<String> {
        let wanted = paths::normalize(target.trim()).ok()?.to_lowercase();
        if wanted.is_empty() {
            return None;
        }
        if let Some(path) = self.by_path.get(&wanted) {
            return Some(path.clone());
        }
        if wanted.contains('/') {
            return None;
        }
        let stem = if paths::is_note(&wanted) {
            paths::file_stem(&wanted)
        } else {
            wanted.as_str()
        };
        self.by_stem
            .get(stem)
            .or_else(|| self.by_alias.get(&wanted))
            .cloned()
    }
}

pub struct Index {
    conn: Connection,
    resolver: Resolver,
}

impl Index {
    /// Opens (or creates) the index of a vault in `.mdnotes/cache/index.db`.
    pub fn open_for(vault: &Vault) -> Result<Self> {
        let db = vault.cache_dir()?.join("index.db");
        match Self::open(&db) {
            Ok(index) => Ok(index),
            // A corrupted cache is not worth an error: start from scratch.
            Err(_) => {
                fs::remove_file(&db).map_err(io_err(&db))?;
                Self::open(&db)
            }
        }
    }

    pub fn open(db: &Path) -> Result<Self> {
        Self::init(Connection::open(db)?)
    }

    pub fn open_in_memory() -> Result<Self> {
        Self::init(Connection::open_in_memory()?)
    }

    fn init(conn: Connection) -> Result<Self> {
        conn.pragma_update_and_check(None, "journal_mode", "WAL", |row| {
            row.get::<_, String>(0)
        })?;
        let version: i64 = conn.pragma_query_value(None, "user_version", |row| row.get(0))?;
        if version != SCHEMA_VERSION {
            conn.execute_batch(DROP_ALL)?;
            conn.execute_batch(SCHEMA)?;
            conn.pragma_update(None, "user_version", SCHEMA_VERSION)?;
        }
        let mut index = Self {
            conn,
            resolver: Resolver::default(),
        };
        index.resolver = index.load_resolver()?;
        Ok(index)
    }

    /// Brings the index in line with the files: re-reads notes whose size or
    /// modification time changed and forgets deleted ones.
    pub fn sync(&mut self, vault: &Vault) -> Result<SyncStats> {
        let mut known: HashMap<String, (i64, i64)> = HashMap::new();
        {
            let mut stmt = self.conn.prepare("SELECT path, mtime, size FROM notes")?;
            let rows = stmt.query_map([], |row| {
                Ok((row.get::<_, String>(0)?, (row.get(1)?, row.get(2)?)))
            })?;
            for row in rows {
                let (path, stamp) = row?;
                known.insert(path, stamp);
            }
        }

        let mut stats = SyncStats::default();
        let tx = self.conn.transaction()?;
        let mut seen = HashSet::new();
        for path in vault.note_paths()? {
            let stamp = file_stamp(vault, &path)?;
            match known.get(&path) {
                Some(old) if *old == stamp => {}
                Some(_) => {
                    upsert(&tx, &vault.read_note(&path)?, stamp)?;
                    stats.updated += 1;
                }
                None => {
                    upsert(&tx, &vault.read_note(&path)?, stamp)?;
                    stats.added += 1;
                }
            }
            seen.insert(path);
        }
        for path in known.keys().filter(|p| !seen.contains(*p)) {
            delete(&tx, path)?;
            stats.removed += 1;
        }
        tx.commit()?;

        if stats != SyncStats::default() {
            self.refresh_links()?;
        }
        Ok(stats)
    }

    /// Re-indexes one note after it was saved.
    pub fn update_note(&mut self, vault: &Vault, note: &Note) -> Result<()> {
        let stamp = file_stamp(vault, &note.path)?;
        let tx = self.conn.transaction()?;
        upsert(&tx, note, stamp)?;
        tx.commit()?;
        self.refresh_links()
    }

    /// Forgets a note that was deleted or moved away.
    pub fn remove_note(&mut self, path: &str) -> Result<()> {
        let tx = self.conn.transaction()?;
        delete(&tx, path)?;
        tx.commit()?;
        self.refresh_links()
    }

    /// Rebuilds the resolver and re-resolves every link.
    fn refresh_links(&mut self) -> Result<()> {
        self.resolver = self.load_resolver()?;
        let tx = self.conn.transaction()?;
        {
            let mut select = tx.prepare("SELECT rowid, target FROM links")?;
            let mut update = tx.prepare("UPDATE links SET resolved = ?1 WHERE rowid = ?2")?;
            let rows = select.query_map([], |row| {
                Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
            })?;
            for row in rows {
                let (rowid, target) = row?;
                update.execute(params![self.resolver.resolve(&target), rowid])?;
            }
        }
        tx.commit()?;
        Ok(())
    }

    fn load_resolver(&self) -> Result<Resolver> {
        let mut resolver = Resolver::default();
        let mut stmt = self.conn.prepare("SELECT path FROM notes ORDER BY path")?;
        let notes = stmt.query_map([], |row| row.get::<_, String>(0))?;
        for path in notes {
            let path = path?;
            let lower = path.to_lowercase();
            let stem = paths::file_stem(&lower).to_string();
            let without_ext = paths::join(paths::parent(&lower), &stem);
            resolver.by_path.insert(lower, path.clone());
            resolver.by_path.entry(without_ext).or_insert(path.clone());
            // Shorter paths win when several notes share a file name.
            let replace = resolver
                .by_stem
                .get(&stem)
                .is_none_or(|existing| existing.len() > path.len());
            if replace {
                resolver.by_stem.insert(stem, path);
            }
        }
        let mut stmt = self.conn.prepare("SELECT path, alias FROM aliases ORDER BY path")?;
        let aliases = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;
        for row in aliases {
            let (path, alias) = row?;
            resolver.by_alias.entry(alias.to_lowercase()).or_insert(path);
        }
        Ok(resolver)
    }

    /// Path of the note a `[[target]]` points to.
    pub fn resolve(&self, target: &str) -> Option<String> {
        self.resolver.resolve(target)
    }

    /// All notes sorted by title.
    pub fn notes(&self) -> Result<Vec<NoteSummary>> {
        let mut tags = self.grouped("SELECT path, tag FROM tags ORDER BY rowid")?;
        let mut aliases = self.grouped("SELECT path, alias FROM aliases ORDER BY rowid")?;
        let mut stmt = self.conn.prepare(
            "SELECT path, title, type FROM notes
             ORDER BY title COLLATE NOCASE, path",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, Option<String>>(2)?,
            ))
        })?;
        let mut out = Vec::new();
        for row in rows {
            let (path, title, note_type) = row?;
            out.push(NoteSummary {
                tags: tags.remove(&path).unwrap_or_default(),
                aliases: aliases.remove(&path).unwrap_or_default(),
                path,
                title,
                note_type,
            });
        }
        Ok(out)
    }

    fn grouped(&self, sql: &str) -> Result<HashMap<String, Vec<String>>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        let mut stmt = self.conn.prepare(sql)?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;
        for row in rows {
            let (path, value) = row?;
            map.entry(path).or_default().push(value);
        }
        Ok(map)
    }

    /// Notes linking to `path`, one entry per linking line.
    pub fn backlinks(&self, path: &str) -> Result<Vec<Backlink>> {
        let mut stmt = self.conn.prepare(
            "SELECT DISTINCT l.source, n.title, l.context
             FROM links l JOIN notes n ON n.path = l.source
             WHERE l.resolved = ?1 AND l.source != ?1
             ORDER BY n.title COLLATE NOCASE, l.source",
        )?;
        let rows = stmt.query_map([path], |row| {
            Ok(Backlink {
                path: row.get(0)?,
                title: row.get(1)?,
                context: row.get(2)?,
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// Links whose target note does not exist.
    pub fn unresolved_links(&self) -> Result<Vec<UnresolvedLink>> {
        let mut stmt = self.conn.prepare(
            "SELECT DISTINCT source, target FROM links
             WHERE resolved IS NULL AND target != ''
             ORDER BY source, target",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(UnresolvedLink {
                source: row.get(0)?,
                target: row.get(1)?,
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// Tags with the number of notes using them, most used first.
    pub fn tags(&self) -> Result<Vec<TagCount>> {
        let mut stmt = self.conn.prepare(
            "SELECT tag, COUNT(DISTINCT path) AS n FROM tags
             GROUP BY tag ORDER BY n DESC, tag COLLATE NOCASE",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(TagCount {
                tag: row.get(0)?,
                count: row.get(1)?,
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// Full-text search over titles and bodies. Every word of `query` must
    /// match (as a prefix); titles weigh more than body text.
    pub fn search(&self, query: &str, limit: u32) -> Result<Vec<SearchHit>> {
        let Some(expr) = fts_query(query) else {
            return Ok(Vec::new());
        };
        let mut stmt = self.conn.prepare(
            "SELECT path, title, snippet(fts, 2, ?2, ?3, '…', 16)
             FROM fts WHERE fts MATCH ?1
             ORDER BY bm25(fts, 0.0, 10.0, 1.0) LIMIT ?4",
        )?;
        let start = MATCH_START.to_string();
        let end = MATCH_END.to_string();
        let rows = stmt.query_map(params![expr, start, end, limit], |row| {
            Ok(SearchHit {
                path: row.get(0)?,
                title: row.get(1)?,
                snippet: row.get(2)?,
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }
}

/// Turns user input into a safe FTS5 expression: each word becomes a quoted
/// prefix term, so operators and punctuation in the input have no effect.
fn fts_query(input: &str) -> Option<String> {
    let terms: Vec<String> = input
        .split(|c: char| !c.is_alphanumeric() && c != '_' && c != '-')
        .map(|w| w.trim_matches('-'))
        .filter(|w| !w.is_empty())
        .map(|w| format!("\"{w}\"*"))
        .collect();
    (!terms.is_empty()).then(|| terms.join(" "))
}

fn file_stamp(vault: &Vault, rel: &str) -> Result<(i64, i64)> {
    let path = paths::resolve(vault.root(), rel)?;
    let meta = fs::metadata(&path).map_err(io_err(&path))?;
    let mtime = meta
        .modified()
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_millis() as i64)
        .unwrap_or_default();
    Ok((mtime, meta.len() as i64))
}

fn delete(tx: &Transaction, path: &str) -> Result<()> {
    for sql in [
        "DELETE FROM notes WHERE path = ?1",
        "DELETE FROM links WHERE source = ?1",
        "DELETE FROM tags WHERE path = ?1",
        "DELETE FROM aliases WHERE path = ?1",
        "DELETE FROM fts WHERE path = ?1",
    ] {
        tx.execute(sql, [path])?;
    }
    Ok(())
}

fn upsert(tx: &Transaction, note: &Note, (mtime, size): (i64, i64)) -> Result<()> {
    delete(tx, &note.path)?;
    let fm = note.front_matter.as_ref();
    let prop = |key: &str| fm.and_then(|v| v.get(key)).and_then(Value::as_str);
    tx.execute(
        "INSERT INTO notes (path, id, title, type, mtime, size)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![note.path, prop("id"), note.title, prop("type"), mtime, size],
    )?;

    let (_, body) = split_front_matter(&note.content);
    let masked = mask_code(body);
    let mut insert_link = tx.prepare(
        "INSERT INTO links (source, target, heading, context) VALUES (?1, ?2, ?3, ?4)",
    )?;
    for (link, context) in extract_wikilinks_with_context(body, &masked) {
        insert_link.execute(params![note.path, link.target, link.heading, context])?;
    }
    // Links listed in properties such as `project: "[[MD Notes]]"`.
    for target in front_matter_links(fm) {
        insert_link.execute(params![note.path, target, None::<String>, ""])?;
    }

    let mut insert_tag = tx.prepare("INSERT INTO tags (path, tag) VALUES (?1, ?2)")?;
    for tag in &note.tags {
        insert_tag.execute(params![note.path, tag])?;
    }
    let mut insert_alias = tx.prepare("INSERT INTO aliases (path, alias) VALUES (?1, ?2)")?;
    for alias in string_list(fm.and_then(|v| v.get("aliases"))) {
        insert_alias.execute(params![note.path, alias])?;
    }
    tx.execute(
        "INSERT INTO fts (path, title, body) VALUES (?1, ?2, ?3)",
        params![note.path, note.title, body],
    )?;
    Ok(())
}

/// Strings of a YAML value that is a string or a list of strings.
fn string_list(value: Option<&Value>) -> Vec<String> {
    match value {
        Some(Value::String(s)) => vec![s.clone()],
        Some(Value::Array(items)) => items
            .iter()
            .filter_map(Value::as_str)
            .map(str::to_string)
            .collect(),
        _ => Vec::new(),
    }
}

/// Targets of `[[links]]` found in top-level string properties.
fn front_matter_links(fm: Option<&Value>) -> Vec<String> {
    let Some(Value::Object(map)) = fm else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for value in map.values() {
        for s in string_list(Some(value)) {
            let masked = mask_code(&s);
            for link in extract_wikilinks(&masked) {
                if !link.target.is_empty() && !out.contains(&link.target) {
                    out.push(link.target);
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> (tempfile::TempDir, Vault, Index) {
        let dir = tempfile::tempdir().unwrap();
        let vault = Vault::init(dir.path(), None).unwrap();
        let index = Index::open_in_memory().unwrap();
        (dir, vault, index)
    }

    #[test]
    fn syncs_incrementally() {
        let (_dir, vault, mut index) = setup();
        vault.write_note("a.md", "# A\n[[b]]").unwrap();
        vault.write_note("b.md", "# B").unwrap();
        let stats = index.sync(&vault).unwrap();
        assert_eq!((stats.added, stats.updated, stats.removed), (2, 0, 0));
        assert_eq!(index.sync(&vault).unwrap(), SyncStats::default());

        vault.write_note("b.md", "# B changed, longer").unwrap();
        vault.move_to_trash("a.md").unwrap();
        let stats = index.sync(&vault).unwrap();
        assert_eq!((stats.added, stats.updated, stats.removed), (0, 1, 1));
        assert_eq!(index.notes().unwrap().len(), 1);
    }

    #[test]
    fn finds_backlinks_with_context() {
        let (_dir, vault, mut index) = setup();
        vault.write_note("Ideas/Target.md", "# Target").unwrap();
        vault
            .write_note("Source.md", "intro\nSee [[target|it]] here.\n`[[Target]]`")
            .unwrap();
        vault
            .write_note("Task.md", "---\nproject: \"[[Target]]\"\n---\nbody")
            .unwrap();
        index.sync(&vault).unwrap();

        let backlinks = index.backlinks("Ideas/Target.md").unwrap();
        assert_eq!(backlinks.len(), 2);
        assert_eq!(backlinks[0].path, "Source.md");
        assert_eq!(backlinks[0].context, "See [[target|it]] here.");
        assert_eq!(backlinks[1].path, "Task.md");
    }

    #[test]
    fn resolves_aliases_and_tracks_unresolved_links() {
        let (_dir, vault, mut index) = setup();
        vault
            .write_note("Local-first.md", "---\naliases: [локальний пріоритет]\n---\n")
            .unwrap();
        vault
            .write_note("a.md", "[[Локальний пріоритет]] [[Missing]]")
            .unwrap();
        index.sync(&vault).unwrap();

        let resolved = index.resolve("ЛОКАЛЬНИЙ пріоритет");
        assert_eq!(resolved.as_deref(), Some("Local-first.md"));
        assert_eq!(index.backlinks("Local-first.md").unwrap().len(), 1);
        let unresolved = index.unresolved_links().unwrap();
        assert_eq!(unresolved.len(), 1);
        assert_eq!(unresolved[0].target, "Missing");

        // Creating the missing note resolves the link.
        let created = vault.write_note("Missing.md", "x").unwrap();
        index.update_note(&vault, &created).unwrap();
        assert!(index.unresolved_links().unwrap().is_empty());
    }

    #[test]
    fn searches_titles_and_bodies() {
        let (_dir, vault, mut index) = setup();
        vault
            .write_note("one.md", "# Синхронізація\nЛокальна копія даних.")
            .unwrap();
        vault
            .write_note("two.md", "# Інше\nПро синхронізацію у фоні.")
            .unwrap();
        vault.write_note("three.md", "# Third\nnothing").unwrap();
        index.sync(&vault).unwrap();

        let hits = index.search("синхроніз", 10).unwrap();
        assert_eq!(hits.len(), 2);
        assert_eq!(hits[0].path, "one.md", "title matches rank first");
        assert!(hits[1].snippet.contains(MATCH_START));
        assert!(index.search("\"(*", 10).unwrap().is_empty());
        assert_eq!(index.search("копія даних", 10).unwrap().len(), 1);
    }

    #[test]
    fn lists_notes_and_tags() {
        let (_dir, vault, mut index) = setup();
        vault
            .write_note("b.md", "---\ntype: task\ntags: [work]\n---\n# Beta #urgent")
            .unwrap();
        vault.write_note("a.md", "# Alpha #work").unwrap();
        index.sync(&vault).unwrap();

        let notes = index.notes().unwrap();
        assert_eq!(notes[0].title, "Alpha");
        assert_eq!(notes[1].note_type.as_deref(), Some("task"));
        assert_eq!(notes[1].tags, vec!["work", "urgent"]);

        let tags = index.tags().unwrap();
        assert_eq!(tags.len(), 2);
        assert_eq!(tags[0].tag, "work");
        assert_eq!(tags[0].count, 2);
    }

    #[test]
    fn remove_note_forgets_it() {
        let (_dir, vault, mut index) = setup();
        vault.write_note("a.md", "[[b]]").unwrap();
        vault.write_note("b.md", "b").unwrap();
        index.sync(&vault).unwrap();
        assert_eq!(index.backlinks("b.md").unwrap().len(), 1);
        index.remove_note("a.md").unwrap();
        assert!(index.backlinks("b.md").unwrap().is_empty());
        assert_eq!(index.notes().unwrap().len(), 1);
    }

    #[test]
    fn fts_query_escapes_input() {
        assert_eq!(fts_query("hello wor").as_deref(), Some("\"hello\"* \"wor\"*"));
        assert_eq!(fts_query("a\"b OR (c)").as_deref(), Some("\"a\"* \"b\"* \"OR\"* \"c\"*"));
        assert_eq!(fts_query(" -- !! "), None);
    }
}
