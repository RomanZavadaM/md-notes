//! SQLite index of a vault.
//!
//! The index makes backlinks, full-text search, tags and alias resolution
//! fast. It is a cache, not a source of truth: it can be deleted at any time
//! and is rebuilt from the files by [`Index::sync`] (see ADR-0002).

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

use rusqlite::{params, Connection, Transaction};
use serde::Serialize;
use serde_json::Value;

use crate::duplicates::name_key;
use crate::error::{io_err, Result};
use crate::graph::GraphRelation;
use crate::identity;
use crate::markdown::{
    extract_wikilinks, extract_wikilinks_with_context, mask_code, split_front_matter,
};
use crate::note::Note;
use crate::paths;
use crate::vault::Vault;

/// Bump when the schema changes: the index is then rebuilt from scratch.
const SCHEMA_VERSION: i64 = 2;

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
CREATE TABLE relations (
    source TEXT NOT NULL,
    target_raw TEXT NOT NULL,
    target TEXT,
    reversed INTEGER NOT NULL,
    kind TEXT NOT NULL,
    grade TEXT,
    origin TEXT,
    status TEXT,
    evidence TEXT
);
CREATE INDEX relations_source ON relations(source);
CREATE INDEX relations_target ON relations(target);
CREATE TABLE identifiers (key TEXT NOT NULL, value TEXT NOT NULL, path TEXT NOT NULL);
CREATE INDEX identifiers_lookup ON identifiers(key, value);
CREATE INDEX identifiers_path ON identifiers(path);
CREATE VIRTUAL TABLE names USING fts5(path UNINDEXED, name);
";

const DROP_ALL: &str = "
DROP TABLE IF EXISTS notes;
DROP TABLE IF EXISTS links;
DROP TABLE IF EXISTS tags;
DROP TABLE IF EXISTS aliases;
DROP TABLE IF EXISTS fts;
DROP TABLE IF EXISTS relations;
DROP TABLE IF EXISTS identifiers;
DROP TABLE IF EXISTS names;
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

/// A note found by name: `exact` when its title or alias equals the query after
/// normalisation (case, quotes, hyphens, legal forms such as `ТОВ`), otherwise a
/// prefix match. Name matches are candidates, never proof of identity (ADR-0008).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NameHit {
    pub path: String,
    pub title: String,
    pub exact: bool,
}

/// A result of [`Index::lookup`]: a note found by an exact identifier or by name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LookupHit {
    pub path: String,
    pub title: String,
    pub note_type: Option<String>,
    /// `identifier` (the note carries exactly this code) or `name` (title or alias).
    pub kind: String,
    /// Identifier key (`edrpou`, `ipn`) for identifier hits.
    pub key: Option<String>,
    /// Exact match: the same code, or the same name after normalisation.
    /// Anything else is only a candidate for a person to confirm (ADR-0008).
    pub exact: bool,
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
        conn.pragma_update_and_check(None, "journal_mode", "WAL", |row| row.get::<_, String>(0))?;
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
        {
            let mut select = tx.prepare("SELECT rowid, target_raw FROM relations")?;
            let mut update = tx.prepare("UPDATE relations SET target = ?1 WHERE rowid = ?2")?;
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
        let mut stmt = self
            .conn
            .prepare("SELECT path, alias FROM aliases ORDER BY path")?;
        let aliases = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;
        for row in aliases {
            let (path, alias) = row?;
            resolver
                .by_alias
                .entry(alias.to_lowercase())
                .or_insert(path);
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

    /// Typed relations of the whole vault (`relations` in front matter) whose other
    /// end resolves to a note, as directed lines `source -> target`. Read from the
    /// index: no note file is opened.
    pub fn relations(&self) -> Result<Vec<GraphRelation>> {
        let mut stmt = self.conn.prepare(
            "SELECT CASE WHEN reversed = 1 THEN target ELSE source END,
                    CASE WHEN reversed = 1 THEN source ELSE target END,
                    kind, grade, origin, status, evidence
             FROM relations WHERE target IS NOT NULL AND target != source",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(GraphRelation {
                source: row.get(0)?,
                target: row.get(1)?,
                kind: row.get(2)?,
                grade: row.get(3)?,
                origin: row.get(4)?,
                status: row.get(5)?,
                evidence: row.get(6)?,
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// Notes that carry exactly this identifier (`key` such as `edrpou`, `value`
    /// as normalised digits). An exact match: the basis of "follow the code".
    pub fn notes_with_identifier(&self, key: &str, value: &str) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare(
            "SELECT DISTINCT path FROM identifiers WHERE key = ?1 AND value = ?2 ORDER BY path",
        )?;
        let rows = stmt.query_map(params![key, value], |row| row.get::<_, String>(0))?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// Identifiers of one note as `(key, value)` pairs.
    pub fn identifiers_of(&self, path: &str) -> Result<Vec<(String, String)>> {
        let mut stmt = self
            .conn
            .prepare("SELECT key, value FROM identifiers WHERE path = ?1 ORDER BY key, value")?;
        let rows = stmt.query_map([path], |row| Ok((row.get(0)?, row.get(1)?)))?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// Title and type of a note.
    fn summary_of(&self, path: &str) -> Result<Option<(String, Option<String>)>> {
        let mut stmt = self
            .conn
            .prepare("SELECT title, type FROM notes WHERE path = ?1")?;
        let mut rows = stmt.query_map([path], |row| Ok((row.get(0)?, row.get(1)?)))?;
        Ok(rows.next().transpose()?)
    }

    /// One entry point for "find this subject": a number of 8 or 10 digits is looked
    /// up as an exact identifier (a code with lost leading zeros of 6-7 digits is
    /// restored to 8); anything else is looked up by name, exact matches first.
    /// Name matches are candidates, never proof of identity.
    pub fn lookup(&self, query: &str, limit: u32) -> Result<Vec<LookupHit>> {
        let compact: String = query
            .chars()
            .filter(|c| !c.is_whitespace() && *c != '-')
            .collect();
        let mut hits = Vec::new();
        if !compact.is_empty() && compact.chars().all(|c| c.is_ascii_digit()) {
            let restored = if (6..8).contains(&compact.len()) {
                format!("{compact:0>8}")
            } else {
                compact
            };
            for (key, len) in identity::IDENTITY_KEYS {
                if restored.len() != *len {
                    continue;
                }
                for path in self.notes_with_identifier(key, &restored)? {
                    if let Some((title, note_type)) = self.summary_of(&path)? {
                        hits.push(LookupHit {
                            path,
                            title,
                            note_type,
                            kind: "identifier".to_string(),
                            key: Some((*key).to_string()),
                            exact: true,
                        });
                    }
                }
            }
            return Ok(hits);
        }
        for hit in self.find_by_name(query, limit)? {
            let note_type = self.summary_of(&hit.path)?.and_then(|(_, kind)| kind);
            hits.push(LookupHit {
                path: hit.path,
                title: hit.title,
                note_type,
                kind: "name".to_string(),
                key: None,
                exact: hit.exact,
            });
        }
        Ok(hits)
    }

    /// Notes whose title or alias matches `query` by name (exact matches first).
    pub fn find_by_name(&self, query: &str, limit: u32) -> Result<Vec<NameHit>> {
        let key = name_key(query);
        let Some(expr) = fts_query(&key) else {
            return Ok(Vec::new());
        };
        let mut stmt = self.conn.prepare(
            "SELECT names.path, notes.title, names.name
             FROM names JOIN notes ON notes.path = names.path
             WHERE names MATCH ?1 LIMIT ?2",
        )?;
        let rows = stmt.query_map(params![expr, limit.saturating_mul(4)], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })?;
        let mut hits: Vec<NameHit> = Vec::new();
        for row in rows {
            let (path, title, name) = row?;
            let exact = name == key;
            match hits.iter_mut().find(|hit| hit.path == path) {
                Some(hit) => hit.exact |= exact,
                None => hits.push(NameHit { path, title, exact }),
            }
        }
        hits.sort_by(|a, b| {
            b.exact
                .cmp(&a.exact)
                .then_with(|| a.title.to_lowercase().cmp(&b.title.to_lowercase()))
        });
        hits.truncate(limit as usize);
        Ok(hits)
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
    let entry = vault
        .provider()
        .metadata(rel)?
        .ok_or_else(|| crate::error::Error::NotFound(rel.to_string()))?;
    let mtime = entry.modified_ms.map(|ms| ms as i64).unwrap_or_default();
    Ok((mtime, entry.size as i64))
}

fn delete(tx: &Transaction, path: &str) -> Result<()> {
    for sql in [
        "DELETE FROM notes WHERE path = ?1",
        "DELETE FROM links WHERE source = ?1",
        "DELETE FROM tags WHERE path = ?1",
        "DELETE FROM aliases WHERE path = ?1",
        "DELETE FROM fts WHERE path = ?1",
        "DELETE FROM relations WHERE source = ?1",
        "DELETE FROM identifiers WHERE path = ?1",
        "DELETE FROM names WHERE path = ?1",
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
    let mut insert_link =
        tx.prepare("INSERT INTO links (source, target, heading, context) VALUES (?1, ?2, ?3, ?4)")?;
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
    let aliases = string_list(fm.and_then(|v| v.get("aliases")));
    for alias in &aliases {
        insert_alias.execute(params![note.path, alias])?;
    }

    // Typed relations, identifiers and normalised names (index v2): the relation
    // tree, the path finder and exact lookups read these tables, not the files.
    let mut insert_relation = tx.prepare(
        "INSERT INTO relations (source, target_raw, reversed, kind, grade, origin, status, evidence)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
    )?;
    for item in relation_items(fm) {
        insert_relation.execute(params![
            note.path,
            item.to,
            item.reversed,
            item.kind,
            item.grade,
            item.origin,
            item.status,
            item.evidence
        ])?;
    }
    if let Some(front) = fm {
        let mut insert_identifier =
            tx.prepare("INSERT INTO identifiers (key, value, path) VALUES (?1, ?2, ?3)")?;
        for (key, value) in identity::codes_in_json(front) {
            insert_identifier.execute(params![key, value, note.path])?;
        }
    }
    let mut insert_name = tx.prepare("INSERT INTO names (path, name) VALUES (?1, ?2)")?;
    let mut seen_names = HashSet::new();
    for name in std::iter::once(&note.title).chain(aliases.iter()) {
        let key = name_key(name);
        if !key.is_empty() && seen_names.insert(key.clone()) {
            insert_name.execute(params![note.path, key])?;
        }
    }
    tx.execute(
        "INSERT INTO fts (path, title, body) VALUES (?1, ?2, ?3)",
        params![note.path, note.title, body],
    )?;
    Ok(())
}

/// One entry of a note's `relations` list, as stored in the index.
struct RelationRow {
    to: String,
    reversed: bool,
    kind: String,
    grade: Option<String>,
    origin: Option<String>,
    status: Option<String>,
    evidence: Option<String>,
}

fn relation_items(fm: Option<&Value>) -> Vec<RelationRow> {
    let Some(list) = fm
        .and_then(|front| front.get("relations"))
        .and_then(Value::as_array)
    else {
        return Vec::new();
    };
    list.iter()
        .filter_map(|item| {
            let text = |key: &str| item.get(key).and_then(Value::as_str).map(str::to_string);
            Some(RelationRow {
                to: text("to")?,
                reversed: item.get("dir").and_then(Value::as_str) == Some("in"),
                kind: text("type").unwrap_or_default(),
                grade: text("grade"),
                origin: text("origin"),
                status: text("status"),
                evidence: text("evidence"),
            })
        })
        .collect()
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
            .write_note(
                "Local-first.md",
                "---\naliases: [локальний пріоритет]\n---\n",
            )
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

    const OWNER: &str = "---\ntitle: Власник\nedrpou: 3114848\n---\n# Власник\n";
    const COMPANY: &str = "---\ntitle: \"ТОВ «ФІАКР-ЛЬВІВ»\"\nedrpou: \"12345678\"\naliases: [Фіакр]\nrelations:\n  - {to: Власник, type: засновник, grade: B2, dir: in}\n  - {to: Нікого, type: керівник}\n---\n# ТОВ «ФІАКР-ЛЬВІВ»\n";

    fn relations_vault() -> (tempfile::TempDir, Vault, Index) {
        let (dir, vault, mut index) = setup();
        vault.write_note("Власник.md", OWNER).unwrap();
        vault.write_note("Фіакр.md", COMPANY).unwrap();
        index.sync(&vault).unwrap();
        (dir, vault, index)
    }

    #[test]
    fn indexed_relations_match_the_ones_read_from_files() {
        let (_dir, vault, index) = relations_vault();
        let from_index = index.relations().unwrap();
        assert_eq!(from_index.len(), 1); // the relation to a missing note is not resolved
        assert_eq!(from_index[0].source, "Власник.md"); // `dir: in` points to the holder
        assert_eq!(from_index[0].target, "Фіакр.md");
        assert_eq!(from_index[0].grade.as_deref(), Some("B2"));

        let graph = crate::graph::knowledge_graph(&index, None).unwrap();
        let by_files = crate::graph::with_relations(&vault, graph.clone()).unwrap();
        let by_index = crate::graph::with_indexed_relations(&index, graph).unwrap();
        assert_eq!(by_files.relations, by_index.relations);
    }

    #[test]
    fn relations_follow_edits_and_removals() {
        let (_dir, vault, mut index) = relations_vault();
        vault.write_note("Нікого.md", "# Нікого\n").unwrap();
        index.sync(&vault).unwrap();
        assert_eq!(index.relations().unwrap().len(), 2); // the target now exists

        vault.write_note("Фіакр.md", "# Фіакр\n").unwrap();
        index.sync(&vault).unwrap();
        assert!(index.relations().unwrap().is_empty());

        vault.write_note("Фіакр.md", COMPANY).unwrap();
        index.sync(&vault).unwrap();
        index.remove_note("Власник.md").unwrap();
        assert_eq!(index.relations().unwrap().len(), 1); // only the one to «Нікого»
    }

    #[test]
    fn identifiers_are_found_by_exact_value_even_without_leading_zeros() {
        let (_dir, _vault, index) = relations_vault();
        assert_eq!(
            index.notes_with_identifier("edrpou", "03114848").unwrap(),
            vec!["Власник.md".to_string()]
        );
        assert_eq!(
            index.notes_with_identifier("edrpou", "12345678").unwrap(),
            vec!["Фіакр.md".to_string()]
        );
        assert!(index
            .notes_with_identifier("edrpou", "3114848")
            .unwrap()
            .is_empty());
        assert!(index
            .notes_with_identifier("ipn", "03114848")
            .unwrap()
            .is_empty());
        assert_eq!(
            index.identifiers_of("Власник.md").unwrap(),
            vec![("edrpou".to_string(), "03114848".to_string())]
        );
    }

    #[test]
    fn names_are_found_without_legal_forms_and_exact_matches_come_first() {
        let (_dir, vault, mut index) = relations_vault();
        vault
            .write_note("Фіакр Львів Плюс.md", "# Фіакр Львів Плюс\n")
            .unwrap();
        index.sync(&vault).unwrap();
        let hits = index.find_by_name("ТЗОВ Фіакр-Львів", 10).unwrap();
        let paths: Vec<&str> = hits.iter().map(|h| h.path.as_str()).collect();
        assert_eq!(paths, vec!["Фіакр.md", "Фіакр Львів Плюс.md"]);
        assert!(hits[0].exact);
        assert!(!hits[1].exact);
        assert!(index
            .find_by_name("Невідомий суб'єкт", 10)
            .unwrap()
            .is_empty());
        assert!(index.find_by_name("ТОВ", 10).unwrap().is_empty()); // only a legal form
                                                                    // the alias finds the same note once
        let by_alias = index.find_by_name("Фіакр", 10).unwrap();
        assert_eq!(by_alias.iter().filter(|h| h.path == "Фіакр.md").count(), 1);
    }

    #[test]
    fn lookup_finds_exact_codes_restores_lost_zeros_and_labels_name_matches() {
        let (_dir, _vault, index) = relations_vault();
        let by_code = index.lookup("03114848", 10).unwrap();
        assert_eq!(by_code.len(), 1);
        assert_eq!(by_code[0].path, "Власник.md");
        assert_eq!(by_code[0].kind, "identifier");
        assert_eq!(by_code[0].key.as_deref(), Some("edrpou"));
        assert!(by_code[0].exact);

        // the same code typed without the lost leading zero and with spaces
        let restored = index.lookup(" 3114 848 ", 10).unwrap();
        assert_eq!(restored.len(), 1);
        assert_eq!(restored[0].path, "Власник.md");

        // a number is never searched as a name; an unknown code gives nothing
        assert!(index.lookup("99999999", 10).unwrap().is_empty());
        assert!(index.lookup("12", 10).unwrap().is_empty());

        let by_name = index.lookup("ТЗОВ Фіакр-Львів", 10).unwrap();
        assert_eq!(by_name[0].path, "Фіакр.md");
        assert_eq!(by_name[0].kind, "name");
        assert!(by_name[0].exact);
        assert_eq!(by_name[0].note_type, None);
        assert!(index.lookup("   ", 10).unwrap().is_empty());
    }

    #[test]
    fn a_large_vault_answers_relation_queries_from_the_index_quickly() {
        let (_dir, vault, mut index) = setup();
        for i in 0..600 {
            let next = (i + 1) % 600;
            vault
                .write_note(
                    &format!("N{i}.md"),
                    &format!(
                        "---\ntitle: Суб'єкт {i}\nedrpou: \"{i:08}\"\nrelations:\n  - {{to: N{next}, type: власник, grade: B2}}\n---\n# Суб'єкт {i}\n"
                    ),
                )
                .unwrap();
        }
        index.sync(&vault).unwrap();
        let started = std::time::Instant::now();
        let graph = crate::graph::knowledge_graph(&index, None).unwrap();
        let graph = crate::graph::with_indexed_relations(&index, graph).unwrap();
        assert_eq!(graph.relations.len(), 600);
        let tree = crate::relation_tree::relation_tree(
            &graph,
            "N0.md",
            crate::relation_tree::TreeDirection::Down,
            8,
        )
        .unwrap();
        assert!(tree.nodes > 1);
        assert_eq!(
            index
                .notes_with_identifier("edrpou", "00000042")
                .unwrap()
                .len(),
            1
        );
        // generous bound: the point is that no note file is read (a file-reading pass over 600 notes is far slower)
        assert!(
            started.elapsed() < std::time::Duration::from_secs(5),
            "{:?}",
            started.elapsed()
        );
    }

    #[test]
    fn fts_query_escapes_input() {
        assert_eq!(
            fts_query("hello wor").as_deref(),
            Some("\"hello\"* \"wor\"*")
        );
        assert_eq!(
            fts_query("a\"b OR (c)").as_deref(),
            Some("\"a\"* \"b\"* \"OR\"* \"c\"*")
        );
        assert_eq!(fts_query(" -- !! "), None);
    }
}
