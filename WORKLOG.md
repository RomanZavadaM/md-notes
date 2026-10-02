# WORKLOG — MD Notes

Оновлено: **02.10.2026**

## STATUS

**ACTIVE. Функціональний scope v0.2 завершений у `main`; останній опублікований checkpoint — v0.2.1.**

Інтегровані після v0.2.1 slices:
- runtime startup hotfix — PR #21;
- Windows portable packaging — PR #20;
- schema/property form — PR #19;
- vault presets/startup gate — PR #23;
- global/local knowledge graph — PR #25.

Поточний операційний пріоритет: **реальний Windows runtime validation актуального `main` перед наступним release checkpoint або переходом до v0.3**.

## Останній завершений slice — knowledge graph

PR #25 `feat: add global and local knowledge graph`:

- `notes-core`: `GraphNode`, `GraphEdge`, `KnowledgeGraph`;
- global graph — усі indexed notes, включно з isolated, + deduplicated resolved internal links;
- local graph — active note + one-hop incoming/outgoing neighbors;
- unresolved/self-links виключені, stale focus → empty graph;
- thin Tauri `knowledge_graph_snapshot` + typed frontend API;
- Sigma.js 3.0.3 + Graphology 0.26.0, обидва MIT;
- `package-lock.json` згенеровано repository Lockfiles workflow;
- `THIRD_PARTY_NOTICES.md` оновлено, license gate PASS;
- right panel: «Зв'язки / Локальний граф / Глобальний граф»;
- click node → open note;
- UI UK / EN / FR / DE / ES / KO / JA;
- responsive graph canvas;
- `docs/KNOWLEDGE_GRAPH.md`;
- clean CI #132 / run `36981607612` — **PASS**: Windows/macOS/Linux core, frontend TypeScript/Vite, Tauri clippy, dependency licenses, PR title;
- squash merge у `main`: `39040a461d0b29f09db382f7bca32483350c19ba`.

Runtime GUI graph validation у поточному середовищі не виконувалась.

## Runtime evidence gate

Реальний тест v0.2.1 на Windows раніше виявив blocker: після запуску програма могла сильно зависати.

З того часу в `main` інтегровано:
- блокування silent auto-reopen `mdnotes.lastVault`;
- explicit startup gate «відкрити останнє / відкрити існуюче / створити нове»;
- last vault відкривається лише через одноразовий explicit-open marker після дії користувача.

**Але нова поведінка ще не перевірена реальною Windows-збіркою. Green CI/build не вважається runtime validation.**

### Що потрібно перевірити на Windows

1. запуск актуальної збірки без автоматичного відкриття великого/останнього vault — UI має залишатися responsive;
2. «Про програму» до відкриття vault;
3. явне відкриття невеликого існуючого vault;
4. створення Empty/PARA/Zettelkasten у порожній папці;
5. відмова preset на непорожній папці без зміни її файлів;
6. schema/property form на тестовій нотатці;
7. local/global graph і click-node navigation;
8. повторний запуск і явне «Відкрити останнє сховище»;
9. якщо зависання повториться саме після відкриття vault — зафіксувати розмір/тип папки й перейти до оптимізації `note_paths()` / `get_tree()` / synchronous index sync.

## Наступна дія

1. підготувати актуальну Windows test build **без створення GitHub Release**, якщо це можливо в межах CI/PR workflow; або дочекатися команди власника на повний checkpoint;
2. отримати реальний runtime результат від користувача;
3. якщо freeze відтворюється — пріоритетно виправити open-vault performance blocker;
4. якщо runtime базово стабільний — за рішенням власника або створити наступний test-release checkpoint («зливай у main»), або почати roadmap v0.3;
5. **не оголошувати v0.2 release-ready до runtime evidence**.

## Останній опублікований checkpoint

- GitHub prerelease [`v0.2.1`](https://github.com/RomanZavadaM/md-notes/releases/tag/v0.2.1), 01.10.2026;
- release commit `26661d8c088df51ce162018d2a3b525502cf30e1`;
- release workflow #19 / run `36924302514` — **SUCCESS**;
- Windows/macOS/Linux packages + `MD-Notes-0.2.1-START.zip` + legal notices + `SHA256SUMS.txt` published.

## Відомі обмеження

- Windows startup-freeze hotfix ще не підтверджено повторним реальним тестом;
- великі vault-и індексуються синхронно при ручному відкритті;
- startup/preset UI, schema/property form і graph UI не пройшли manual runtime validation у поточному середовищі;
- graph slice не включає ForceAtlas2/clustering або persistence layout;
- збірки не підписані;
- Android/iOS ще не збираються.
