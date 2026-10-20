use std::collections::{BTreeSet, HashSet};

use serde::Serialize;

use serde_json::Value;

use crate::index::Index;
use crate::vault::Vault;
use crate::Result;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphNode {
    pub path: String,
    pub title: String,
    pub note_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphEdge {
    pub source: String,
    pub target: String,
}

/// A typed relation read from the `relations` list of a note's front matter
/// (for example ownership with a grade A1-F6). It connects two notes of the
/// graph and carries what the viewer needs to style the line.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphRelation {
    pub source: String,
    pub target: String,
    pub kind: String,
    pub grade: Option<String>,
    pub origin: Option<String>,
    pub status: Option<String>,
    pub evidence: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KnowledgeGraph {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
    pub relations: Vec<GraphRelation>,
}

/// Builds a graph snapshot from the existing SQLite index.
///
/// With no focus, every indexed note is returned. With a focus path, the
/// result is the one-hop neighborhood: the active note plus notes connected
/// to it by a resolved outgoing or incoming link. The Markdown files remain
/// the source of truth; this function only reads the rebuildable index cache.
pub fn knowledge_graph(index: &Index, focus: Option<&str>) -> Result<KnowledgeGraph> {
    let notes = index.notes()?;
    let focus_exists =
        focus.is_none_or(|focus_path| notes.iter().any(|note| note.path == focus_path));

    // A focus path can be stale (for example after an external delete). In
    // that case the local graph should simply be empty instead of inventing a
    // node that is not present in the index.
    if !focus_exists {
        return Ok(KnowledgeGraph {
            nodes: Vec::new(),
            edges: Vec::new(),
            relations: Vec::new(),
        });
    }

    let mut edges = BTreeSet::new();
    for target in &notes {
        for backlink in index.backlinks(&target.path)? {
            if backlink.path != target.path {
                edges.insert(GraphEdge {
                    source: backlink.path,
                    target: target.path.clone(),
                });
            }
        }
    }

    let visible: Option<HashSet<String>> = focus.map(|focus_path| {
        let mut paths = HashSet::from([focus_path.to_string()]);
        for edge in &edges {
            if edge.source == focus_path {
                paths.insert(edge.target.clone());
            }
            if edge.target == focus_path {
                paths.insert(edge.source.clone());
            }
        }
        paths
    });

    let nodes = notes
        .into_iter()
        .filter(|note| {
            visible
                .as_ref()
                .is_none_or(|paths| paths.contains(&note.path))
        })
        .map(|note| GraphNode {
            path: note.path,
            title: note.title,
            note_type: note.note_type,
        })
        .collect::<Vec<_>>();

    let known_paths: HashSet<_> = nodes.iter().map(|node| node.path.as_str()).collect();
    let edges = edges
        .into_iter()
        .filter(|edge| {
            known_paths.contains(edge.source.as_str()) && known_paths.contains(edge.target.as_str())
        })
        .collect();

    Ok(KnowledgeGraph {
        nodes,
        edges,
        relations: Vec::new(),
    })
}

/// Adds typed relations from the notes' front matter to a snapshot.
///
/// Each entry of a note's `relations` list needs a `to` (a note title, file
/// name or path, resolved like a `[[link]]`) and may carry `type`, `grade`,
/// `origin`, `status` and `evidence`; `dir: in` means the relation points
/// from the `to` note to the note that holds it (default: from the holder).
/// Relations whose target does not resolve to a note
/// of the snapshot are ignored. Notes that cannot be read are skipped.
pub fn with_relations(vault: &Vault, mut graph: KnowledgeGraph) -> Result<KnowledgeGraph> {
    let known: HashSet<String> = graph.nodes.iter().map(|node| node.path.clone()).collect();
    let mut relations = BTreeSet::new();
    for node in &graph.nodes {
        let Ok(note) = vault.read_note(&node.path) else {
            continue;
        };
        let Some(list) = note
            .front_matter
            .as_ref()
            .and_then(|front| front.get("relations"))
            .and_then(Value::as_array)
        else {
            continue;
        };
        for item in list {
            let Some(to) = item.get("to").and_then(Value::as_str) else {
                continue;
            };
            let Some(target) = vault.resolve_link(to)? else {
                continue;
            };
            if target == node.path || !known.contains(&target) {
                continue;
            }
            let text = |key: &str| item.get(key).and_then(Value::as_str).map(str::to_string);
            let reversed = item.get("dir").and_then(Value::as_str) == Some("in");
            let (source, target) = if reversed {
                (target, node.path.clone())
            } else {
                (node.path.clone(), target)
            };
            relations.insert(GraphRelation {
                source,
                target,
                kind: text("type").unwrap_or_default(),
                grade: text("grade"),
                origin: text("origin"),
                status: text("status"),
                evidence: text("evidence"),
            });
        }
    }
    graph.relations = relations.into_iter().collect();
    Ok(graph)
}

/// Same as [`with_relations`], but reads the typed relations from the index
/// instead of opening every note file: the relation tree and the path finder
/// stay fast on large vaults.
pub fn with_indexed_relations(index: &Index, mut graph: KnowledgeGraph) -> Result<KnowledgeGraph> {
    let known: HashSet<&str> = graph.nodes.iter().map(|node| node.path.as_str()).collect();
    let relations: BTreeSet<GraphRelation> = index
        .relations()?
        .into_iter()
        .filter(|r| known.contains(r.source.as_str()) && known.contains(r.target.as_str()))
        .collect();
    graph.relations = relations.into_iter().collect();
    Ok(graph)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Index, Vault};

    fn indexed_vault() -> (tempfile::TempDir, Vault, Index) {
        let dir = tempfile::tempdir().unwrap();
        let vault = Vault::init(dir.path(), Some("Graph".into())).unwrap();
        vault
            .write_note("A.md", "# A\n\n[[B]]\n[[B]]\n[[Missing]]\n")
            .unwrap();
        vault.write_note("B.md", "# B\n\n[[C]]\n").unwrap();
        vault.write_note("C.md", "# C\n\n[[A]]\n").unwrap();
        vault.write_note("Isolated.md", "# Isolated\n").unwrap();
        let mut index = Index::open_in_memory().unwrap();
        index.sync(&vault).unwrap();
        (dir, vault, index)
    }

    #[test]
    fn global_graph_contains_all_notes_and_deduplicated_resolved_edges() {
        let (_dir, _vault, index) = indexed_vault();
        let graph = knowledge_graph(&index, None).unwrap();

        let paths: HashSet<_> = graph.nodes.iter().map(|node| node.path.as_str()).collect();
        assert_eq!(paths.len(), 4);
        assert!(paths.contains("A.md"));
        assert!(paths.contains("B.md"));
        assert!(paths.contains("C.md"));
        assert!(paths.contains("Isolated.md"));

        assert_eq!(
            graph.edges,
            vec![
                GraphEdge {
                    source: "A.md".into(),
                    target: "B.md".into(),
                },
                GraphEdge {
                    source: "B.md".into(),
                    target: "C.md".into(),
                },
                GraphEdge {
                    source: "C.md".into(),
                    target: "A.md".into(),
                },
            ]
        );
    }

    #[test]
    fn local_graph_is_one_hop_around_focus() {
        let (_dir, _vault, index) = indexed_vault();
        let graph = knowledge_graph(&index, Some("A.md")).unwrap();

        let paths: HashSet<_> = graph.nodes.iter().map(|node| node.path.as_str()).collect();
        assert_eq!(paths, HashSet::from(["A.md", "B.md", "C.md"]));
        assert_eq!(graph.edges.len(), 3);
    }

    #[test]
    fn relations_from_front_matter_are_resolved_and_filtered() {
        let dir = tempfile::tempdir().unwrap();
        let vault = Vault::init(dir.path(), None).unwrap();
        vault
            .write_note(
                "Person.md",
                "---\ntype: person\nrelations:\n  - {type: \"owner\", to: \"Company\", grade: \"B2\", origin: \"registry\", status: \"unverified\"}\n  - {type: \"ghost\", to: \"Nobody\", grade: \"F6\"}\n  - {type: \"self\", to: \"Person\"}\n  - {grade: \"A1\"}\n---\n# Person\n",
            )
            .unwrap();
        vault
            .write_note("Company.md", "---\ntype: company\n---\n# Company\n")
            .unwrap();
        let mut index = Index::open_in_memory().unwrap();
        index.sync(&vault).unwrap();

        let graph = with_relations(&vault, knowledge_graph(&index, None).unwrap()).unwrap();
        assert_eq!(
            graph.relations,
            vec![GraphRelation {
                source: "Person.md".into(),
                target: "Company.md".into(),
                kind: "owner".into(),
                grade: Some("B2".into()),
                origin: Some("registry".into()),
                status: Some("unverified".into()),
                evidence: None,
            }]
        );
    }

    #[test]
    fn dir_in_points_the_relation_at_the_holder_and_keeps_evidence() {
        let dir = tempfile::tempdir().unwrap();
        let vault = Vault::init(dir.path(), None).unwrap();
        vault
            .write_note(
                "Company.md",
                "---\ntype: company\nrelations:\n  - {type: \"owned by\", to: \"Person\", dir: \"in\", grade: \"A1\", evidence: \"EDR extract\"}\n---\n# Company\n",
            )
            .unwrap();
        vault
            .write_note("Person.md", "---\ntype: person\n---\n# Person\n")
            .unwrap();
        let mut index = Index::open_in_memory().unwrap();
        index.sync(&vault).unwrap();

        let graph = with_relations(&vault, knowledge_graph(&index, None).unwrap()).unwrap();
        assert_eq!(graph.relations.len(), 1);
        assert_eq!(graph.relations[0].source, "Person.md");
        assert_eq!(graph.relations[0].target, "Company.md");
        assert_eq!(graph.relations[0].evidence.as_deref(), Some("EDR extract"));
    }

    #[test]
    fn local_graph_for_missing_focus_is_empty() {
        let (_dir, _vault, index) = indexed_vault();
        assert_eq!(
            knowledge_graph(&index, Some("Missing.md")).unwrap(),
            KnowledgeGraph {
                nodes: Vec::new(),
                edges: Vec::new(),
                relations: Vec::new(),
            }
        );
    }
}
