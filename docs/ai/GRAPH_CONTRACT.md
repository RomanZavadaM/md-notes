# MD Notes Graph Contract for AI and automation

**Contract:** `md-notes-graph/1`  
**Applies to:** current v1.0.0 semantics  
**Audience:** AI agents, importers, batch editors, analyzers and code changes that read/write graph data.

Normative words **MUST**, **MUST NOT**, **SHOULD** and **MAY** are intentional.

## 1. Source of truth

1. Markdown note files and their YAML front matter **MUST** be treated as the source of truth.
2. `.mdnotes/cache/index.db` **MUST** be treated as a rebuildable derived cache.
3. Visual graph coordinates, force layout, colours, shapes and line lengths **MUST NOT** be treated as semantic data unless a future explicit contract says otherwise.
4. An agent **MUST NOT** invent a persistent graph-only fact that is absent from notes/front matter.

## 2. Ordinary wiki-link edge

A resolved ordinary wiki link represents:

```text
source note  --->  target note
```

Rules:
- repeated identical source/target links collapse to one edge in the graph snapshot;
- self-links do not become a visible ordinary edge;
- an unresolved target does not create an invented node;
- a wiki-link direction means only “source references target” and **MUST NOT** be reinterpreted automatically as ownership, causality, control or legal direction.

## 3. Typed relation schema

A typed relation is an item in front matter `relations`.

```yaml
relations:
  - to: "Target"
    type: "owner"
    grade: "A1"
    origin: "registry"
    status: "confirmed"
    evidence: "registry extract"
    dir: "in"
```

### Fields

- `to`: **REQUIRED**. Target reference resolved with the product's internal-link resolver.
- `type`: optional relation semantics.
- `grade`: optional reliability grade.
- `origin`: optional provenance/source description.
- `status`: optional verification/lifecycle status.
- `evidence`: optional concrete supporting evidence.
- `dir`: optional. Exact value `in` reverses the stored relation direction.

Unknown optional fields in front matter **MUST be preserved** unless an explicit migration owns them.

### Direction

Default:
```text
note holding relation ---> resolved to target
```

With `dir: in`:
```text
resolved to target ---> note holding relation
```

An unresolved `to` target and a self-relation **MUST NOT** become a graph relation.

## 4. Evidence and inference

An AI **MUST NOT** create or upgrade `type`, `grade`, `origin`, `status` or `evidence` from visual graph geometry or mere co-occurrence.

When writing a relation:
- the model **SHOULD** retain the factual wording of the available source;
- `origin` **SHOULD** identify the provenance when known;
- `evidence` **SHOULD** identify the concrete supporting item when known;
- missing facts **MUST remain missing**, not guessed.

If evidence is conflicting, the agent **MUST NOT** silently collapse conflict into a single stronger fact.

## 5. Grade semantics

Format is a letter A–F plus digit 1–6.

- letter evaluates source reliability;
- digit evaluates the claim;
- A1 is the strongest stated combination;
- F6 is the weakest stated combination;
- no `grade` means “not formally graded”, not “bad”.

Current UI bands primarily use the numeric component:
- 1–2: reliable;
- 3–4: medium;
- 5–6: weak/rumour signal.

An AI **MUST NOT** generate a grade without a defined assessment basis.

## 6. Global and local snapshot semantics

### Global graph
- all indexed notes are eligible nodes;
- resolved ordinary links are edges;
- typed relations between known nodes are overlaid;
- isolated nodes are valid.

### Local graph in v1.0.0
- starts with the focus note;
- visible neighbourhood is one hop over resolved ordinary wiki-link edges;
- typed relations are overlaid only when both endpoints are already in that visible snapshot.

An agent **MUST NOT** claim that a typed relation alone currently expands the local graph.

## 7. Relation tree/path semantics

- relation tree is directional from a selected root;
- parent/child displayed `grade` is the best grade among lines connecting that pair;
- `pathGrade` is the weakest grade along the path from root to the node.

The chain rule is **weakest-link**, not arithmetic averaging.

Path existence **MUST NOT** be treated as proof of causality, ownership or any semantics not explicitly represented by relation types.

## 8. Canonical-write guidance

An automated writer **SHOULD**:
1. use a plain wiki link for an ordinary reference;
2. use `relations` for facts needing explicit type/direction/provenance/evidence;
3. avoid duplicating the same semantic fact in both endpoint notes;
4. choose one canonical relation entry and `dir` when reverse direction is needed;
5. preserve unrelated YAML/front-matter keys and note body content;
6. run/rebuild indexing after changes through supported application/core flows where possible.

## 9. Prohibited interpretations

An agent **MUST NOT** infer, without separate evidence, that:
- graph proximity means semantic similarity;
- a common neighbour means common ownership;
- absence of an edge means absence of a real-world relation;
- a red/weakly graded node is false;
- an ordinary link is a legal relation;
- two opposite links equal a symmetric typed relation.

## 10. Contract changes

Any intentional semantic change to relation fields, direction, local graph scope, grade meaning or source-of-truth rules **MUST** update:
- this contract;
- `graph-contract.yaml`;
- Ukrainian human guide;
- six translations;
- relevant tests/documentation.

Machine-readable companion: [graph-contract.yaml](graph-contract.yaml).
