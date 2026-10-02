use std::collections::{BTreeSet, HashMap, HashSet};

use serde::Serialize;

use crate::index::Index;
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KnowledgeGraph {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

/// Builds a graph snapshot from the existing SQLite index.
///
/// With no focus, every indexed note is returned. With a focus path, the
/// result is the one-hop neighborhood: the active note plus notes connected
/// to it by a resolved outgoing or incoming link. The Markdown files remain
/// the source of truth; this function only reads the rebuildable index cache.
pub fn knowledge_graph(index: &Index, focus: Option<&str>) -> Result<KnowledgeGraph> {
    let notes = index.notes()?;
    let note_map: HashMap<_, _> = notes
        .iter()
        .map(|note| (note.path.as_str(), note))
        .collect();

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
        .filter(|note| visible.as_ref().is_none_or(|paths| paths.contains(&note.path)))
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

    // A focus path can be stale (for example after an external delete). In
    // that case the local graph should simply be empty instead of inventing a
    // node that is not present in the index.
    if let Some(focus_path) = focus {
        if !note_map.contains_key(focus_path) {
            return Ok(KnowledgeGraph {
                nodes: Vec::new(),
                edges: Vec::new(),
            });
        }
    }

    Ok(KnowledgeGraph { nodes, edges })
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
    fn local_graph_for_missing_focus_is_empty() {
        let (_dir, _vault, index) = indexed_vault();
        assert_eq!(
            knowledge_graph(&index, Some("Missing.md")).unwrap(),
            KnowledgeGraph {
                nodes: Vec::new(),
                edges: Vec::new(),
            }
        );
    }
}
