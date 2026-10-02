# WORKLOG — MD Notes

Оновлено: **02.10.2026**

## STATUS

**ACTIVE. Checkpoint v0.2.1 — DONE; `main` випереджає опублікований checkpoint.** Runtime hotfix, Windows portable packaging, schema/property form і vault presets уже інтегровані. Поточний і останній функціональний slice v0.2: **граф знань — глобальний і локальний**.

## Нещодавно завершено

### Runtime startup hotfix
- користувач на реальній Windows-збірці v0.2.1 підтвердив сильне зависання після запуску;
- PR #21 заблокував неявне startup-відкриття останнього vault;
- CI #92 — **PASS**;
- merge: `de7b9d1b8fa7b9f5c7884b4082d977f4a1029b01`.

**Не закрито доказом:** hotfix ще треба перевірити реальною новою Windows-збіркою. Якщо після ручного вибору vault зависання повториться — оптимізувати `note_paths()` / `get_tree()` і винести дорогий index sync із критичного open path.

### Windows portable
- PR #20 додає стандартний release artifact `MD-Notes-<version>-Windows-x64-portable.zip` поряд із `.exe` та `.msi`;
- CI #95 — **PASS**;
- merge: `715e29dfcf3bace2480e6b2bd824cfc80c91a668`.

### Note types / schema / property form
- PR #19: `feat: add schema-driven note properties`;
- clean CI #97 — **PASS**;
- merge: `7441259a019eae1f7858898d8a29a9ac4d9ec7cd`.

### Vault presets / startup gate
- PR #23: `feat: add safe vault presets`;
- Empty, PARA, Zettelkasten, explicit startup gate, UI 7 мовами, safety refusal для non-empty folder;
- clean CI #114 — **PASS**;
- merge: `c078d075d089257c83b903eea44c317f391b765c`;
- docs sync PR #24 / CI #117 — **PASS**, merge `e00a17272c3e5dd0cfd354bdb7bb344dc023fece`.

Runtime GUI-перевірка startup/preset flow у поточному середовищі **не виконувалась**.

## Поточний slice — knowledge graph

Branch: `feature/knowledge-graph-v0.2`.
Draft PR #25: `feat: add global and local knowledge graph`.

Реалізовано у candidate:

- `notes-core` типи `GraphNode`, `GraphEdge`, `KnowledgeGraph`;
- graph snapshot формується з existing rebuildable SQLite index, а не з нового data format;
- global graph: усі indexed notes, включно з isolated notes, + deduplicated resolved internal links;
- local graph: active note + one-hop incoming/outgoing neighbors;
- unresolved links і self-links не створюють graph edges/nodes;
- stale/missing local focus повертає empty graph;
- core unit-тести для global/local/missing-focus;
- thin Tauri command `knowledge_graph_snapshot`;
- typed frontend `KnowledgeGraph` API;
- Sigma.js 3.0.3 + Graphology 0.26.0, обидва MIT;
- `package-lock.json` оновлено штатним Lockfiles workflow у GitHub Actions; тимчасовий feature-branch push trigger після цього прибрано, `main` workflow не змінюється;
- `THIRD_PARTY_NOTICES.md` оновлено;
- `KnowledgeGraphPanel` із Sigma renderer;
- right panel має режими «Зв'язки / Локальний граф / Глобальний граф»;
- локальний focus розміщується в центрі, global має deterministic circular initial layout;
- click node відкриває відповідну нотатку;
- graph UI локалізовано UK / EN / FR / DE / ES / KO / JA;
- mobile/narrow layout має окрему мінімальну висоту canvas без горизонтального блокування;
- `docs/KNOWLEDGE_GRAPH.md` описує модель, режими й межі.

### Поточна перевірка

Clean CI #129 запущено для завершеного candidate. На момент запису Conventional PR title уже **PASS**, решта jobs виконуються.
Runtime GUI-перевірка graph UI у поточному середовищі **не виконувалась** і не вважається виконаною.

## Наступна дія

1. дочекатися повного CI #129;
2. виправити всі Rust/TypeScript/Sigma/license нестикування на цій самій гілці;
3. після green checks оновити PR #25 body і перевести draft → ready;
4. технічно інтегрувати PR #25 у `main`;
5. синхронізувати roadmap / PROJECT_STATE / START_HERE / Issue #8;
6. після graph slice функціональний scope v0.2 завершений;
7. release checkpoint робити лише за окремою командою власника **«зливай у main»**;
8. окремо перед/під час наступного checkpoint обов'язково перевірити Windows runtime hotfix на реальній новій збірці.

## Останній опублікований checkpoint

- GitHub prerelease [`v0.2.1`](https://github.com/RomanZavadaM/md-notes/releases/tag/v0.2.1), 01.10.2026;
- release commit `26661d8c088df51ce162018d2a3b525502cf30e1`;
- release workflow #19 / run `36924302514` — **SUCCESS**;
- Windows/macOS/Linux packages + `MD-Notes-0.2.1-START.zip` + legal notices + `SHA256SUMS.txt` published.

## Відомі обмеження

- runtime freeze hotfix інтегровано, але ще не підтверджено реальною новою Windows-збіркою;
- великі сховища все ще індексуються синхронно при ручному відкритті;
- startup/preset UI, schema/property form і graph UI ще не пройшли manual runtime validation у поточному середовищі;
- graph slice не включає ForceAtlas2/clustering або збереження layout — positions є UI state, не user data;
- збірки не підписані;
- Android/iOS ще не збираються.
