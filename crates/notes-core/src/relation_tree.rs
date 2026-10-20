//! Relation tree and path finder over the typed relations of a vault graph.
//!
//! A typed relation is an entry of a note's `relations` list (ownership, role,
//! material that supports a claim) with an optional grade such as `B2`
//! (letter: reliability of the source, A best .. F worst; digit: credibility
//! of the claim, 1 best .. 6 worst). [`relation_tree`] unfolds the chain above
//! a subject (who owns it, who owns them) or below it (what it owns), and
//! [`relation_path`] finds the shortest chain between two notes. Both carry the
//! *weakest link* grade of the chain: a chain is as trustworthy as its least
//! trustworthy step. Nothing is written; plain `[[links]]` are not used.

use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

use serde::Serialize;

use crate::graph::{GraphNode, GraphRelation, KnowledgeGraph};

/// A tree never has more nodes than this; `truncated` tells when it was cut.
pub const MAX_TREE_NODES: usize = 400;

/// Which way the chain is unfolded from the subject.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TreeDirection {
    /// Notes that relate *to* the subject (its owners, sources), then theirs.
    Up,
    /// Notes the subject relates *to* (what it owns), then theirs.
    Down,
}

impl TreeDirection {
    pub fn parse(text: &str) -> Option<TreeDirection> {
        match text {
            "up" => Some(TreeDirection::Up),
            "down" => Some(TreeDirection::Down),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TreeNode {
    pub path: String,
    pub title: String,
    pub note_type: Option<String>,
    /// Relation kinds on the line from the parent (several kinds are joined).
    pub kinds: Vec<String>,
    /// Best grade among the relations between this note and its parent.
    pub grade: Option<String>,
    /// Weakest grade along the whole chain from the subject to this note.
    pub path_grade: Option<String>,
    pub origin: Option<String>,
    pub status: Option<String>,
    pub evidence: Option<String>,
    /// The note already appears higher or earlier in the tree: not unfolded again.
    pub repeated: bool,
    /// The depth limit stopped here although the note has further relations.
    pub has_more: bool,
    pub children: Vec<TreeNode>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RelationTree {
    pub root: TreeNode,
    /// Number of nodes including the root.
    pub nodes: usize,
    pub truncated: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PathStep {
    pub path: String,
    pub title: String,
    pub note_type: Option<String>,
    /// Relation kind between the previous step and this one (none for the first step).
    pub kind: Option<String>,
    pub grade: Option<String>,
    /// True when the relation points from the previous step to this one.
    pub forward: bool,
    pub origin: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RelationPath {
    pub steps: Vec<PathStep>,
    /// Weakest grade among the steps of the chain.
    pub weakest: Option<String>,
}

/// Reliability letter (A-F) and credibility digit (1-6) of a grade like `B2`.
fn grade_parts(grade: &str) -> (Option<char>, Option<char>) {
    let mut letter = None;
    let mut digit = None;
    for c in grade.chars() {
        let upper = c.to_ascii_uppercase();
        if letter.is_none() && ('A'..='F').contains(&upper) {
            letter = Some(upper);
        } else if digit.is_none() && ('1'..='6').contains(&c) {
            digit = Some(c);
        }
    }
    (letter, digit)
}

/// Sort key: smaller is better; a note without a grade is worse than any grade.
fn grade_key(grade: Option<&str>) -> (u8, u8) {
    let (letter, digit) = grade.map(grade_parts).unwrap_or((None, None));
    (
        digit.map_or(9, |d| d as u8 - b'0'),
        letter.map_or(9, |l| l as u8 - b'A'),
    )
}

/// The weaker of two grades, letter and digit separately: `weakest("B2", "C1")` is `C2`.
pub fn weakest_grade(a: Option<&str>, b: Option<&str>) -> Option<String> {
    let (la, da) = a.map(grade_parts).unwrap_or((None, None));
    let (lb, db) = b.map(grade_parts).unwrap_or((None, None));
    let letter = la.into_iter().chain(lb).max();
    let digit = da.into_iter().chain(db).max();
    if letter.is_none() && digit.is_none() {
        return None;
    }
    let mut out = String::new();
    if let Some(l) = letter {
        out.push(l);
    }
    if let Some(d) = digit {
        out.push(d);
    }
    Some(out)
}

struct Link {
    other: String,
    kinds: Vec<String>,
    grade: Option<String>,
    origin: Option<String>,
    status: Option<String>,
    evidence: Option<String>,
}

struct Builder<'a> {
    relations: &'a [GraphRelation],
    nodes: HashMap<&'a str, &'a GraphNode>,
    direction: TreeDirection,
    max_depth: u8,
    expanded: HashSet<String>,
    count: usize,
    truncated: bool,
}

impl<'a> Builder<'a> {
    fn title_of(&self, path: &str) -> (String, Option<String>) {
        match self.nodes.get(path) {
            Some(node) => (node.title.clone(), node.note_type.clone()),
            None => (path.to_string(), None),
        }
    }

    /// Notes one step away in the chosen direction, one entry per note, best line first.
    fn links(&self, path: &str) -> Vec<Link> {
        let mut grouped: BTreeMap<String, Vec<&GraphRelation>> = BTreeMap::new();
        for relation in self.relations {
            let other = match self.direction {
                TreeDirection::Up if relation.target == path => &relation.source,
                TreeDirection::Down if relation.source == path => &relation.target,
                _ => continue,
            };
            if other != path {
                grouped.entry(other.clone()).or_default().push(relation);
            }
        }
        let mut links: Vec<Link> = grouped
            .into_iter()
            .map(|(other, items)| {
                let best = items
                    .iter()
                    .min_by_key(|r| grade_key(r.grade.as_deref()))
                    .copied()
                    .unwrap_or(items[0]);
                let mut kinds: Vec<String> = Vec::new();
                for item in &items {
                    if !item.kind.is_empty() && !kinds.contains(&item.kind) {
                        kinds.push(item.kind.clone());
                    }
                }
                Link {
                    other,
                    kinds,
                    grade: best.grade.clone(),
                    origin: best.origin.clone(),
                    status: best.status.clone(),
                    evidence: best.evidence.clone(),
                }
            })
            .collect();
        links.sort_by(|a, b| {
            grade_key(a.grade.as_deref())
                .cmp(&grade_key(b.grade.as_deref()))
                .then_with(|| self.title_of(&a.other).0.cmp(&self.title_of(&b.other).0))
        });
        links
    }

    fn build(
        &mut self,
        path: &str,
        link: Option<&Link>,
        parent_chain: Option<&str>,
        depth: u8,
    ) -> TreeNode {
        let (title, note_type) = self.title_of(path);
        let grade = link.and_then(|l| l.grade.clone());
        let path_grade = match link {
            Some(l) => weakest_grade(parent_chain, l.grade.as_deref()),
            None => None,
        };
        let repeated = self.expanded.contains(path);
        let mut node = TreeNode {
            path: path.to_string(),
            title,
            note_type,
            kinds: link.map(|l| l.kinds.clone()).unwrap_or_default(),
            grade,
            path_grade: path_grade.clone(),
            origin: link.and_then(|l| l.origin.clone()),
            status: link.and_then(|l| l.status.clone()),
            evidence: link.and_then(|l| l.evidence.clone()),
            repeated,
            has_more: false,
            children: Vec::new(),
        };
        if repeated {
            return node;
        }
        self.expanded.insert(path.to_string());
        let links = self.links(path);
        if depth >= self.max_depth {
            node.has_more = !links.is_empty();
            return node;
        }
        for child in &links {
            if self.count >= MAX_TREE_NODES {
                self.truncated = true;
                break;
            }
            self.count += 1;
            let built = self.build(&child.other, Some(child), path_grade.as_deref(), depth + 1);
            node.children.push(built);
        }
        node
    }
}

/// Unfolds the chain of typed relations above or below `root` up to `max_depth` steps.
/// Returns `None` when `root` is not a note of the graph.
pub fn relation_tree(
    graph: &KnowledgeGraph,
    root: &str,
    direction: TreeDirection,
    max_depth: u8,
) -> Option<RelationTree> {
    if !graph.nodes.iter().any(|node| node.path == root) {
        return None;
    }
    let mut builder = Builder {
        relations: &graph.relations,
        nodes: graph.nodes.iter().map(|n| (n.path.as_str(), n)).collect(),
        direction,
        max_depth: max_depth.clamp(1, 8),
        expanded: HashSet::new(),
        count: 1,
        truncated: false,
    };
    let root_node = builder.build(root, None, None, 0);
    Some(RelationTree {
        root: root_node,
        nodes: builder.count,
        truncated: builder.truncated,
    })
}

/// Shortest chain of typed relations between two notes, ignoring the direction of
/// the lines, at most `max_hops` steps. `None` when there is no such chain.
pub fn relation_path(
    graph: &KnowledgeGraph,
    from: &str,
    to: &str,
    max_hops: usize,
) -> Option<RelationPath> {
    let info: HashMap<&str, &GraphNode> =
        graph.nodes.iter().map(|n| (n.path.as_str(), n)).collect();
    if !info.contains_key(from) || !info.contains_key(to) {
        return None;
    }
    let step_of = |path: &str, relation: Option<(&GraphRelation, bool)>| {
        let node = info.get(path);
        PathStep {
            path: path.to_string(),
            title: node.map_or_else(|| path.to_string(), |n| n.title.clone()),
            note_type: node.and_then(|n| n.note_type.clone()),
            kind: relation
                .map(|(r, _)| r.kind.clone())
                .filter(|kind| !kind.is_empty()),
            grade: relation.and_then(|(r, _)| r.grade.clone()),
            forward: relation.is_some_and(|(_, forward)| forward),
            origin: relation.and_then(|(r, _)| r.origin.clone()),
            status: relation.and_then(|(r, _)| r.status.clone()),
        }
    };
    if from == to {
        return Some(RelationPath {
            steps: vec![step_of(from, None)],
            weakest: None,
        });
    }
    let mut adjacent: HashMap<&str, Vec<(&str, usize, bool)>> = HashMap::new();
    for (index, relation) in graph.relations.iter().enumerate() {
        adjacent
            .entry(relation.source.as_str())
            .or_default()
            .push((relation.target.as_str(), index, true));
        adjacent
            .entry(relation.target.as_str())
            .or_default()
            .push((relation.source.as_str(), index, false));
    }
    let mut previous: HashMap<&str, (&str, usize, bool)> = HashMap::new();
    let mut seen: HashSet<&str> = HashSet::from([from]);
    let mut queue: VecDeque<(&str, usize)> = VecDeque::from([(from, 0)]);
    let mut found = false;
    while let Some((current, hops)) = queue.pop_front() {
        if current == to {
            found = true;
            break;
        }
        if hops >= max_hops {
            continue;
        }
        for &(next, index, forward) in adjacent.get(current).into_iter().flatten() {
            if seen.insert(next) {
                previous.insert(next, (current, index, forward));
                queue.push_back((next, hops + 1));
            }
        }
    }
    if !found {
        return None;
    }
    let mut steps = Vec::new();
    let mut weakest: Option<String> = None;
    let mut cursor = to;
    while cursor != from {
        let (before, index, forward) = previous[cursor];
        let relation = &graph.relations[index];
        weakest = weakest_grade(weakest.as_deref(), relation.grade.as_deref());
        steps.push(step_of(cursor, Some((relation, forward))));
        cursor = before;
    }
    steps.push(step_of(from, None));
    steps.reverse();
    Some(RelationPath { steps, weakest })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(path: &str, title: &str, note_type: &str) -> GraphNode {
        GraphNode {
            path: path.to_string(),
            title: title.to_string(),
            note_type: Some(note_type.to_string()),
        }
    }

    fn rel(source: &str, target: &str, kind: &str, grade: Option<&str>) -> GraphRelation {
        GraphRelation {
            source: source.to_string(),
            target: target.to_string(),
            kind: kind.to_string(),
            grade: grade.map(str::to_string),
            origin: Some("official".to_string()),
            status: None,
            evidence: None,
        }
    }

    /// D --(A1)--> A, B --(B2)--> A, C --(C3)--> B, X alone.
    fn graph() -> KnowledgeGraph {
        KnowledgeGraph {
            nodes: vec![
                node("A.md", "Альфа", "company"),
                node("B.md", "Бета", "company"),
                node("C.md", "Гамма", "person"),
                node("D.md", "Дельта", "person"),
                node("X.md", "Ікс", "company"),
            ],
            edges: Vec::new(),
            relations: vec![
                rel("D.md", "A.md", "власник", Some("A1")),
                rel("B.md", "A.md", "власник", Some("B2")),
                rel("C.md", "B.md", "власник", Some("C3")),
            ],
        }
    }

    #[test]
    fn weakest_grade_takes_the_worse_letter_and_digit_separately() {
        assert_eq!(weakest_grade(Some("B2"), Some("C1")), Some("C2".to_string()));
        assert_eq!(weakest_grade(None, Some("B2")), Some("B2".to_string()));
        assert_eq!(weakest_grade(Some("a1"), Some("A1")), Some("A1".to_string()));
        assert_eq!(weakest_grade(None, None), None);
        assert_eq!(weakest_grade(Some("x"), None), None);
    }

    #[test]
    fn the_tree_above_a_subject_lists_the_best_lines_first_with_chain_grades() {
        let tree = relation_tree(&graph(), "A.md", TreeDirection::Up, 5).unwrap();
        assert_eq!(tree.nodes, 4);
        assert!(!tree.truncated);
        let titles: Vec<&str> = tree.root.children.iter().map(|c| c.title.as_str()).collect();
        assert_eq!(titles, vec!["Дельта", "Бета"]); // A1 before B2
        let beta = &tree.root.children[1];
        assert_eq!(beta.grade.as_deref(), Some("B2"));
        assert_eq!(beta.path_grade.as_deref(), Some("B2"));
        let gamma = &beta.children[0];
        assert_eq!(gamma.title, "Гамма");
        assert_eq!(gamma.grade.as_deref(), Some("C3"));
        assert_eq!(gamma.path_grade.as_deref(), Some("C3")); // the weakest link of the chain
        assert_eq!(gamma.kinds, vec!["власник".to_string()]);
    }

    #[test]
    fn the_tree_below_a_subject_follows_outgoing_relations() {
        let tree = relation_tree(&graph(), "C.md", TreeDirection::Down, 5).unwrap();
        let chain: Vec<&str> = [&tree.root.children[0], &tree.root.children[0].children[0]]
            .iter()
            .map(|n| n.title.as_str())
            .collect();
        assert_eq!(chain, vec!["Бета", "Альфа"]);
    }

    #[test]
    fn the_depth_limit_marks_further_relations() {
        let tree = relation_tree(&graph(), "A.md", TreeDirection::Up, 1).unwrap();
        let beta = tree.root.children.iter().find(|c| c.title == "Бета").unwrap();
        assert!(beta.has_more);
        assert!(beta.children.is_empty());
        let delta = tree.root.children.iter().find(|c| c.title == "Дельта").unwrap();
        assert!(!delta.has_more);
    }

    #[test]
    fn a_cycle_is_unfolded_once() {
        let mut g = graph();
        g.relations.push(rel("A.md", "C.md", "пов'язаний", Some("D4")));
        let tree = relation_tree(&g, "A.md", TreeDirection::Up, 8).unwrap();
        let mut repeated = 0;
        let mut stack = vec![&tree.root];
        while let Some(n) = stack.pop() {
            if n.repeated {
                repeated += 1;
                assert!(n.children.is_empty());
            }
            stack.extend(n.children.iter());
        }
        assert_eq!(repeated, 1);
    }

    #[test]
    fn several_lines_between_two_notes_become_one_with_the_best_grade() {
        let mut g = graph();
        g.relations.push(rel("B.md", "A.md", "директор", Some("A2")));
        let tree = relation_tree(&g, "A.md", TreeDirection::Up, 1).unwrap();
        let beta = tree.root.children.iter().find(|c| c.title == "Бета").unwrap();
        assert_eq!(beta.grade.as_deref(), Some("A2"));
        assert_eq!(beta.kinds, vec!["власник".to_string(), "директор".to_string()]);
    }

    #[test]
    fn an_unknown_root_gives_no_tree() {
        assert!(relation_tree(&graph(), "Nope.md", TreeDirection::Up, 3).is_none());
    }

    #[test]
    fn the_path_between_two_notes_ignores_line_direction_and_reports_the_weakest_link() {
        let path = relation_path(&graph(), "A.md", "C.md", 6).unwrap();
        let titles: Vec<&str> = path.steps.iter().map(|s| s.title.as_str()).collect();
        assert_eq!(titles, vec!["Альфа", "Бета", "Гамма"]);
        assert_eq!(path.weakest.as_deref(), Some("C3"));
        assert_eq!(path.steps[1].kind.as_deref(), Some("власник"));
        assert!(!path.steps[1].forward); // Бета -> Альфа points against the walking direction
        assert!(path.steps[0].kind.is_none());
    }

    #[test]
    fn no_path_for_disconnected_or_too_distant_notes() {
        assert!(relation_path(&graph(), "A.md", "X.md", 6).is_none());
        assert!(relation_path(&graph(), "A.md", "C.md", 1).is_none());
        assert!(relation_path(&graph(), "A.md", "Missing.md", 6).is_none());
        let same = relation_path(&graph(), "A.md", "A.md", 6).unwrap();
        assert_eq!(same.steps.len(), 1);
    }
}
