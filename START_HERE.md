# START_HERE — MD Notes

> Перша точка входу для нового чату або відновлення після обриву.

## Статус

**ACTIVE — останній реліз v0.3.21 (prerelease, 28.10.2026); runtime-тест показав, що погоджений багатовіконний UX НЕ реалізований.**

Оновлено: 28.10.2026. Версія береться з GitHub (`gh release list`), а не з цього файла.

- останній реліз: [v0.3.21](https://github.com/RomanZavadaM/md-notes/releases/tag/v0.3.21), prerelease; Windows Portable: `MD-Notes-0.3.21-Windows-x64-portable.zip`; installer: `MD.Notes_0.3.21_x64-setup.exe`;
- у релізі: Windows installer/MSI + portable ZIP, macOS DMG/app archive, Linux AppImage/deb/rpm, START/source, legal notices, `SHA256SUMS.txt`;
- збірки не підписані й не нотаризовані;
- README синхронізовані мовами UK / EN / FR / DE / ES / KO / JA; переклади FR/DE/ES/KO/JA машинні, не вичитані носіями (еталон — українська);
- PR #122 «Назад» у вікні графа інтегрований (`058c27a370730b36437ef8494fed8fd6a13f307f`). PR #128 інтегровано (`2da9e7ffb01355008de782df6326d98587923dff`): документаційний запис майбутніх групових операцій і вкладок; не реалізація.

## Що реалізовано (станом на v0.3.20)

Усе нижче покрито автоматичними тестами й збіркою на GitHub. Ручної перевірки у живому застосунку, крім Windows v0.2.2, власник не підтверджував.

- **Основа v0.1-v0.2:** сховище з Markdown-файлів, редактор, пошук (FTS5), посилання/зворотні посилання, теги, Mermaid/KaTeX, вкладення, шаблони, схеми властивостей, пресети, граф, 7 мов інтерфейсу.
- **Storage / mobile:** `StorageProvider`, sandbox-сховище Android/iOS (лише збірка в CI), стан синхронізації.
- **Git:** клон, статус, commit, fetch, безпечний pull (лише чистий fast-forward), **push через системний `git`** із запобіжником для зашифрованих сховищ (PR #82), панель Git у застосунку (PR #86).
- **Standalone Markdown:** одиночний файл без сховища, явна асоціація файлів.
- **Зашифроване сховище (ADR-0007, age), зрізи 1-3:** ядро, ключ у системному сховищі секретів, діалоги створення/імпорту ключа, випадкові імена файлів, запобіжник push.
- **Імпорт папки** у сховище з об’єднанням за кодом ЄДРПОУ/ІПН (ADR-0008), список можливих дублікатів.
- **Граф:** жива фізика, окреме вікно, форми вузлів і легенда, лінії й колір за оцінкою надійності, режим «звідки це прийшло», дерево зв’язків, найнадійніший шлях, фільтр за оцінкою.
- **Факти з локальної копії даних «З-під Ляди»** за точним кодом у пошуку (ЄДР, АМКУ, Prozorro).
- **Багатовкладковий редактор (ADR-0010, M1):** кілька vault-нотаток одночасно, незалежні чернетки та стан збереження; перемикання вкладок не перезаписує іншу чернетку.
- **Незалежні editor-вікна (ADR-0010, M2):** активну vault-нотатку можна явно винести в окреме Tauri-вікно; перед перенесенням вона зберігається, а головний редактор повертається до попередньої вкладки або порожнього стану. CI/mobile build PASS; ручний runtime ще не прийнятий.

## Що не реалізовано

1. Інтеграція Git state ↔ `SyncManifest` / ADR-0005 (повна політика конфліктів).
2. WebDAV.
3. Перевірка Android/iOS на фізичних пристроях (є лише збірка в CI); Android SAF / iOS security-scoped папки.
4. У зашифрованому сховищі: зображення й вкладення не показуються; немає інтерфейсу «додати ключ відновлення» до вже створеного сховища (є лише команда); кеш-індекс `.mdnotes/cache` не зашифрований; push не має власного інтерфейсу для ключів.
5. Таблиці / мова запитів (продовження), мобільні картки замість таблиць.
6. Підписування збірок.
7. ADR-0010: виправити фактичний multiwindow UX. У v0.3.21 клік по вузлах графа лише накопичує вкладки в головному вікні; погоджений сценарій «граф збоку + головний редактор + незалежні вікна нотаток» не досягнуто.

Не заявляти, що пункти вище готові.

## Startup protocol

1. Прочитати `PROJECT_RULES.md`.
2. Прочитати `PROJECT_STATE.md`.
3. Прочитати `WORKLOG.md`.
4. Перевірити фактичний GitHub: `main` SHA, open PR, останні workflow runs і latest release.
5. Прочитати останні записи GitHub Issue #8.
6. Якщо GitHub і документація суперечать одне одному — GitHub має пріоритет, документацію синхронізувати.
7. Продовжити з першої незавершеної дії `WORKLOG.md`; не повторювати вже інтегровану роботу.

## Джерела істини

- `PROJECT_RULES.md` — постійні правила;
- `PROJECT_STATE.md` — інтегрований стан `main`;
- `WORKLOG.md` — operational queue і точна наступна дія;
- GitHub Issue #8 — append-only development ledger;
- `Cargo.toml`, `app/package.json`, `app/src-tauri/tauri.conf.json` — machine source версії;
- `docs/releases/` — release notes;
- `docs/roadmap.md` — roadmap.

## Команда власника «злити у main»

**«Злити у main / зливай у main»** означає повний test-release checkpoint: нова версія, зелені checks, merge, збірки всіх підтримуваних платформ, START/source, checksums/legal, tag + GitHub prerelease, документація та ledger.

## Для нового чату

> **Продовжуємо MD Notes. Відкрий у GitHub `START_HERE.md` і продовжуй строго за ним.**

## Завершення сесії 27.10.2026 — актуальний checkpoint

- **main (перед фінальним docs PR):** `51e8f2487d09ad32a2dd8c076127d595228d43a2`. На час перевірки **відкритих PR немає**.
- **Останній опублікований prerelease:** [v0.3.20](https://github.com/RomanZavadaM/md-notes/releases/tag/v0.3.20), 13 assets, у тому числі Windows Portable [ZIP](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.20/MD-Notes-0.3.20-Windows-x64-portable.zip) і SHA256SUMS; Release #112 — успішно (Windows, macOS, Linux, публікація, README links).
- PR #132: перегляд нотатки у відокремленому графі; PR #133: зовнішній Markdown у окремій вкладці при відкритому vault, без автоматичного імпорту. Входять до v0.3.20. **Windows runtime acceptance цих змін ще не підтверджено**.
- PR #134 release-please merged, PR #137 синхронізація документації merged, PR #138 ADR-0010 (багатовіконний режим) merged. **ADR-0010 — тільки проєктне рішення, функція ще не реалізована**.
- ADR-0009 backup/restore — запропоновано, **немає реалізації**. Підготовчої гілки може вистачати без коду: звірити перед роботою; не плутати з інтегрованою функцією.
- Вимога власника на завтра: **кілька вкладок і незалежних вікон редактора нотаток**; обрати маленький безпечний зріз M1 (модель документів/вкладок), з тестами та конфліктним захистом, потім M2 (незалежні вікна). Врахувати graph, зовнішні файли, спільний vault і незбережені зміни в усіх вікнах. Backup/restore не загубити (ADR-0009) і узгодити зі станом вкладок.
- Наступний старт: перевірити фактичні `main`/open PR/releases/actions, `PROJECT_RULES.md`, `PROJECT_STATE.md`, `WORKLOG.md`, ADR-0009 та ADR-0010, Issue #8. Жодних тверджень про runtime PASS до тесту власником. Для релізів обов'язкові PR checks, tag, Portable, SHA256SUMS, документація та 7 мов.


## Checkpoint 28.10.2026 — ADR-0010 M1 + M2

- Фактичний `main` після M2: `34723be6d85654f4b4561c7318abf79d93893cee`.
- PR #142 (M1) merged: вкладки vault-нотаток та незалежні `draft/saved` стани.
- PR #144 (M2) merged: команда «Відкрити в окремому вікні», незалежне editor-вікно, повернення main до попередньої вкладки/порожнього стану, 7 мов.
- Фінальний M2 CI #411 — SUCCESS; Mobile smoke #180 — SUCCESS (Android + iOS simulator).
- Логіку графа не змінено: клік вершини й надалі змінює активну нотатку в головному редакторі без примусового focus/minimize.
- **Не заявляти runtime PASS:** багатовіконний UX ще треба перевірити вручну на Windows.
- **M3 завершено в коді:** PR #146/#147/#148; фінальний `main` `1b33390b9bb70f18b60bea1c7c9492c5a89a5098`. Наступний етап — **M4: platform/runtime acceptance та тестовий prerelease**.
- ADR-0009 backup/restore лишається незавершеним і не повинен загубитися.


## Checkpoint 28.10.2026 — ADR-0010 M3

- PR #146: conflict guard — save відхиляється, якщо файл на диску вже змінився відносно завантаженої версії.
- PR #147: detached editor не закривається з dirty draft, доки guarded save не завершився успішно.
- PR #148: main зберігає всі відкриті вкладки перед switch/quit; Quit спочатку безпечно закриває всі `editor-*` і не викликає примусовий exit, якщо editor лишився через conflict/error.
- Фінальний `main`: `1b33390b9bb70f18b60bea1c7c9492c5a89a5098`.
- CI #417 — SUCCESS; Mobile smoke #185 — SUCCESS.
- **Runtime PASS ще не заявляти. Наступна дія: M4 — сформувати тестовий prerelease/Portable та провести ручне Windows-приймання багатовіконного UX.**


## Фінальний checkpoint 28.10.2026 — v0.3.21

- Опублікований prerelease **v0.3.21** з Windows Portable/EXE/MSI, macOS universal DMG/app archive, Linux AppImage/deb/rpm, START/source, legal files і `SHA256SUMS.txt`.
- Release target commit: `297a6b356075afb4fb31ab9779c3c8a3e4fd548c`; `main` перед цим docs-checkpoint: `a420a9610ae4e726baefb5870e04692119fa316b`.
- ADR-0010 технічна основа M1–M3 входить до v0.3.21, але **ручний Windows-тест 28.10.2026 = FAIL для цільового багатовіконного UX**: клік по вузлах графа відкриває/накопичує вкладки в main, а сама схема роботи фактично не змінилася.
- UI i18n parity перевірено 28.10.2026: UK/EN/FR/DE/ES/KO/JA — по 98 основних ключів, missing/extra = 0.
- Після runtime-приймання наступний функціональний блок: ADR-0009 backup/restore; далі Git state ↔ SyncManifest, WebDAV, physical mobile validation та інший backlog.


## Runtime verdict 28.10.2026 — v0.3.21

- Власник перевірив фактичну поведінку Windows Portable.
- **FAIL як реалізація погодженого багатовіконного режиму.**
- При відкритому окремому вікні графа кліки по вузлах створюють/накопичують багато вкладок у головному вікні. Граф лишається тим самим; реальний робочий сценарій не покращився.
- Не називати v0.3.21 «повною реалізацією multiwindow». Правильне формулювання: вкладки + частина інфраструктури detached editor / safe-save є, але користувацький multiwindow UX не завершений.
- Погоджена ціль: звичайний клік вузла графа має **міняти активну нотатку у головному редакторі, а не плодити вкладки**; окреме вікно нотатки створюється лише явною командою; після detach головне вікно лишається доступним для іншої нотатки; одночасно можуть існувати graph + main + кілька незалежних editor-вікон.
- Наступний чат повинен спочатку відтворити це на коді й виправити routing/open-note model, а не продовжувати backlog після помилкового PASS.


## 29.10.2026 — ADR-0010 graph routing r1 інтегровано

- Поточний інтегрований `main`: `9a2db3532b63db2e7c9f52c05902d4f10a87cb24` (PR #153).
- Виправлено конкретну причину runtime FAIL v0.3.21: `GRAPH_OPEN_NOTE` більше не використовує загальний tab-creating `openNote()`.
- Звичайний клік вузла графа тепер просить main **замінити/перемкнути активну vault-нотататку**, а не накопичувати нові вкладки. Якщо target уже відкритий — він лише активується.
- Перед заміною dirty active document проходить наявний guarded save/conflict check; швидкі послідовні кліки мають latest-request guard.
- Примусовий focus main прибрано. Preview/history графа, звичайні вкладки та explicit «Відкрити в окремому вікні» не змінені.
- CI PR #153 / run `40841554655` — SUCCESS. **Це не runtime PASS.** Наступна точна дія — ручний Windows тест власником: graph + main + кілька explicit detached editor windows.
- До приймання r1 не переходити до ADR-0009 backup/restore. Git↔SyncManifest, WebDAV, physical mobile tests, encrypted-vault gaps і signing також залишаються backlog.


## 29.10.2026 — Windows runtime test artifact для ADR-0010 r1

- Поточний `main`: `c6d5d398ab0db0b3272dc966c7e45bd2016d3887`.
- Одноразова тестова збірка виконана без release/tag через PR #156; PR закритий **без merge** після успішного build.
- Actions run: `40843601078` = SUCCESS; artifact ID: `12884352283`; назва: `MD-Notes-ADR0010-r1-c6d5d398-Windows-x64-portable`.
- Portable ZIP: `MD-Notes-ADR0010-r1-c6d5d398-Windows-x64-portable.zip`; SHA256: `3d261922637fb29db3eb5a57ed42637095966a265f3e6f8c13a6d4fb4e69de1f`.
- Це **TEST BUILD**, не новий реліз; executable все ще має продуктову версію 0.3.21. Release Please PR #154 (`0.3.22`) лишається незлитим.
- Наступна точна дія: ручний Windows runtime verdict власника. Перевірити: graph-node click змінює active main note без накопичення tabs → explicit «Відкрити в окремому вікні» → main доступний для іншої нотатки → кілька editor windows → conflict/safe close.
- До фактичного PASS не завершувати ADR-0010 і не починати ADR-0009.


## 29.10.2026 — ADR-0010 graph routing r2 інтегровано, Windows test artifact готовий

- PR #158 merged у `main` → `66eb9728bf9f0dadaf59d75a84ac3dde8ae30d17`.
- CI run `40853765572`: SUCCESS для всіх запущених jobs (frontend, licenses, notes-core Windows/Linux/macOS, Tauri clippy Linux/macOS).
- Graph navigation тепер адресна: `emitTo("main", GRAPH_OPEN_NOTE, path)`; detached `editor-*` не отримують graph navigation і зберігають власну note identity.
- Після graph selection main виконує `unminimize()+show()`, коротко піднімається у Z-order, після чого focus повертається graph-вікну. Existing replace-active/conflict/latest-request routing не змінено.
- Temporary PR #159 закритий **без merge** після Windows runtime build `40854135789` = SUCCESS.
- Artifact ID `12889336347`: `MD-Notes-ADR0010-r2-f7b35c9b-Windows-x64-portable`; inner ZIP `MD-Notes-ADR0010-r2-f7b35c9b-Windows-x64-portable.zip`; SHA256 `3d8cf4ad903e99c25cf9002ba7d4accd45645917a0a1b9e4536eb7ba1fd6280a`.
- Це TEST BUILD, не v0.3.22. Release Please PR #154 лишається відкритим; його не зливати до owner runtime verdict.
- **ADR-0010 ще NOT DONE.** Єдина наступна дія: owner Windows runtime test r2. Перевірити, що graph-node click змінює main, main стає видимим поверх/перед detached editors, focus повертається graph і повторні кліки працюють; graph + main + кілька editor windows не ламаються.
- Після фактичного PASS перейти до ADR-0009 — backup/restore **поточного робочого vault**. До PASS не починати його реалізацію.


## 29.10.2026 — уточнення нумерації тестових збірок

- Продуктова версія у `main` лишається **0.3.21**. Release Please PR #154 готує **0.3.22** через інтегровані fix #153/#158; вручну версію не змінювати.
- Позначення `ADR0010-r1/r2-<SHA>` у Windows Portable — це **runtime-test revision**, а не нова продуктова версія.
- Щоб надалі це не виглядало як альтернативна схема SemVer, у PROJECT_RULES зафіксовано: офіційні release assets = `MD-Notes-X.Y.Z-...`; одноразові тести = `MD-Notes-TEST-<scope>-rN-<shortSHA>-...`.
- Вже видані r1/r2 не перейменовувати заднім числом: їхні байти/SHA та записи Issue #8 незмінні. Наступний тестовий кандидат має отримати наступну унікальну ревізію.


## 29.10.2026 — ADR-0010 r3: side-by-side + per-tab detach, Windows TEST ready

- PR #162 merged → product-code main `b0f7813c6820cfd5eefb92f0c414d723aab93147`.
- CI `40859261723` = SUCCESS; Mobile smoke `40859261721` = SUCCESS (Android + iOS simulator).
- Root UX correction: detached graph no longer opens as a large centered overlay when space allows. `main + graph` are tiled side-by-side in current monitor work area (~58/42); graph node clicks route to visible main without r2 focus/Z-order ping-pong.
- Any vault tab now has explicit `↗` detach. `detachDocument(path)` saves/conflict-checks exactly that tab, opens its independent editor and removes only that tab from main. Non-active tab detach does not disturb the visible main/external document.
- Temporary PR #163 closed WITHOUT MERGE after Windows runtime workflow `40860534312` = SUCCESS.
- TEST artifact ID `12891562599`: `MD-Notes-TEST-ADR0010-r3-b0f7813c-Windows-x64-portable`; ZIP `MD-Notes-TEST-ADR0010-r3-b0f7813c-Windows-x64-portable.zip`; SHA256 `26fa666bc316d360b9bb2f8de5f950c2eeb548b4b523bb9b4511e67543766fe4`.
- Product version is still 0.3.21; TEST r3 is not v0.3.22. Release Please PR #154 remains separate.
- **ADR-0010 runtime verdict PENDING.** Owner must verify side-by-side layout, graph→visible-main navigation, per-tab detach and coexistence of graph + main + multiple editor windows.
- Only after owner PASS: finalize ADR-0010/release decision, then start ADR-0009 backup/restore current working vault.


## 29.10.2026 — ADR-0010 r3 owner FAIL → r4

- Owner Windows runtime verdict for r3: **FAIL**. Opening detached graph did not produce visible side-by-side `main + graph`.
- r3 remains immutable issued test revision; do not reuse or relabel it.
- Root-cause audit: r3 relied on constructor-time graph `x/y/size`, and its logical-width gate (`680 + 520`) was too strict for common Windows DPI scaling, silently selecting centered fallback.
- r4 target: reduce adaptive logical minimums and, after creating graph, wait for native `tauri://created`; only then explicitly unmaximize/resize/reposition both native windows via `Window.getByLabel`.
- Runtime PASS remains owner-only. Next issued Windows candidate must be r4.


## 29.10.2026 — ADR-0010 r4 integrated + Windows TEST ready

- Owner runtime verdict r3: **FAIL** — detached graph did not appear side-by-side with main.
- r4 PR #166 merged → product-code main `572f4b01d9710b7527c09024fa4f27ffb6a8dfa7`; CI `40864278512` = SUCCESS.
- r4 correction: DPI-friendly logical minimums (`main 520`, `graph 360`); new graph waits for native `tauri://created`, then both native windows are explicitly unmaximized/resized/repositioned.
- Temporary TEST PR #167 closed WITHOUT MERGE; Windows runtime workflow `40865268396` = SUCCESS.
- Artifact ID `12894111712`: `MD-Notes-TEST-ADR0010-r4-572f4b01-Windows-x64-portable`.
- Inner ZIP: `MD-Notes-TEST-ADR0010-r4-572f4b01-Windows-x64-portable.zip`; SHA256 `6206a01e639feb0b86b2b3bab20c5e1128404de34f83168d96be51082570a155`.
- ZIP downloaded and independently verified: checksum matches; `unzip -t` reports no errors.
- Product SemVer remains 0.3.21; TEST r4 is not v0.3.22.
- **ADR-0010 remains NOT DONE until owner runtime verdict for r4.** Single next action: test whether opening detached graph visibly tiles `main + graph` side-by-side on Windows; also verify graph clicks update visible main and per-tab `↗` detach still works.
- Do not start ADR-0009 backup/restore until this verdict is recorded.


## 29.10.2026 — ADR-0010 r4 runtime FAIL → r5 workspace model

- Owner runtime r4: **FAIL**. Graph still covers main; detached editor windows do not change this. Clicking graph nodes accumulates vault tabs in hidden main; they become visible only after manually shrinking graph.
- r4 is immutable issued test revision. Next Windows candidate is r5.
- Owner уточнив runtime: graph click **успішно змінює активну нотатку/вкладку в main**. Результат просто не видно, доки graph не згорнути/перемістити. Тому routing не переробляти у r5.
- r5 target звужено до native window layout/visibility: `main` має лишатися видимим поруч із graph; існуючу graph→main navigation зберегти без функціональних змін.
- r5 window target: graph starts compact on the right, main on the left. Для надійності layout треба виконувати на native-рівні, а не покладатися лише на JS window positioning. Додати explicit re-tile action як ручний fallback/діагностику.
- Detached `editor-*` windows stay independent and are not part of graph destination routing.
- Owner runtime PASS remains mandatory.


## 29.10.2026 — ADR-0010 r5 integrated + Windows TEST ready

- Owner clarification: graph clicks already change the active note/tab in `main` correctly; the unresolved defect is that detached graph hides main. r5 does **not** change graph→main routing.
- PR #169 merged → product-code main `fd1db060f853965f5830d6ec4d2bc4fbea668653`.
- CI `40871542186` = SUCCESS; Mobile smoke `40871542173` = SUCCESS.
- r5 native workspace layout: Rust/Tauri command uses physical monitor work-area pixels; main is placed left, graph compact right. Layout is attempted after graph native creation and after GraphWindow mount.
- Graph window has visible `⇆` re-tile action (localized in all 7 UI languages) and shows numeric native layout diagnostic (`main + graph / monitor`) or an explicit error.
- Temporary TEST PR #170 closed WITHOUT MERGE; Windows runtime workflow `40872847286` = SUCCESS.
- Artifact ID `12896978541`: `MD-Notes-TEST-ADR0010-r5-fd1db060-Windows-x64-portable`.
- Inner ZIP: `MD-Notes-TEST-ADR0010-r5-fd1db060-Windows-x64-portable.zip`; SHA256 `d5789a02e1cdc3bdcac6e04a02e90438c9ad687529cffe976d6c96445cc3e757`.
- r5 EXE SHA256 `609af1493c8d0f72c467d6cff639d70b56c56a7953fac928f956caa6440ec9d8`, size `29,545,984` bytes; it is physically different from r4 EXE.
- ZIP downloaded and independently verified: checksum matches; archive integrity test passed.
- Product SemVer remains 0.3.21; TEST r5 is not v0.3.22.
- **ADR-0010 remains NOT DONE until owner runtime verdict for r5.** Primary test: open detached graph; main must remain visible left and graph right. If not, press `⇆` and report the exact numeric diagnostic/error shown.
- Do not start ADR-0009 backup/restore until this verdict is recorded.


## 29.10.2026 — ADR-0010 r5 integrated + Windows TEST ready

- Owner clarification after r4: graph→main routing works; graph-node click changes the active note in main. The defect is that detached graph covers main, hiding the changed note.
- r5 PR #169 merged → product-code main `fd1db060f853965f5830d6ec4d2bc4fbea668653`.
- CI `40871542186` = SUCCESS; Mobile smoke `40871542173` = SUCCESS.
- r5 does **not** redesign tabs/navigation. It moves graph workspace layout to native Rust/Tauri.
- Native `tile_graph_workspace` uses physical monitor work-area pixels, places main left and compact graph right, and is independent of Windows logical DPI scaling.
- Layout runs after graph native creation and again after GraphWindow mount; visible `⇆ Розмістити поруч` action can reapply it manually.
- Graph window shows native layout diagnostics (`main + graph / monitor`) or an explicit error string.
- Temporary TEST PR #171 closed WITHOUT MERGE; Windows runtime workflow `40878443536` = SUCCESS.
- Artifact ID `12898759586`: `MD-Notes-TEST-ADR0010-r5-fd1db060-Windows-x64-portable`.
- Inner ZIP SHA256 `c83cd9fef593dcca61e82c9e442b731138349dfe86949bf1d62645410e637e88`.
- `MD Notes.exe` SHA256 `83b1aad02e82329efd8cbfde2c73e74118f7b5d3faf7812fe5f5a105cb7f5954`; size `29545984`; build timestamp `2026-10-29 13:14:40 UTC`.
- ZIP downloaded and verified: checksum matches; archive integrity test passed.
- **ADR-0010 remains NOT DONE until owner runtime verdict for r5.** Primary acceptance: detach graph → main visible left + graph compact right; `⇆` re-tile works; status reports dimensions/error; graph clicks update visible main.
- Release Please PR #154 remains separate; do not merge/release 0.3.22 before runtime verdict.


## 29.10.2026 — ADR-0010 r5 owner screenshot diagnosis → r6

- Owner screenshots reveal the repeatedly tested action was the large `Розгорнути` button inside `KnowledgeGraphPanel`, not the small detached-window `↗` action in the panel tabs.
- `Розгорнути` currently only toggles local React `expanded` state, creating an in-main overlay that covers the editor. This bypasses `openGraphWindow()` entirely, so r3–r5 native tiling could not affect the user's actual path.
- r5 is therefore not accepted for the real owner workflow. Next issued revision is r6.
- r6 target: on desktop, `KnowledgeGraphPanel` expand action must call the same native `openGraphWindow()` path as `↗`; local overlay expansion remains only as fallback where no native callback is supplied (mobile/non-desktop).
- Preserve graph→main routing and native r5 tiling logic; do not redesign tabs.


## 29.10.2026 — ADR-0010 r6: виправлено реальний шлях «Розгорнути», Windows TEST готовий

- За скріншотами власника підтверджено: використовувалась велика кнопка `Розгорнути` всередині правої панелі графа. Вона перемикала внутрішній overlay у головному вікні та повністю обходила окреме вікно графа.
- Тому r3–r5 не могли змінити саме цей сценарій; вони працювали над окремим вікном, яке викликалось малою кнопкою `↗`.
- r6 PR #173 merged → product-code main `6002b529b84a8157d0b713ee4823c957cd532a58`.
- У desktop велика `Розгорнути` тепер відкриває той самий окремий graph, що й `↗`; старе внутрішнє розгортання лишено лише як fallback без desktop-callback.
- Навігація graph→main не змінювалась: за словами власника вона вже працює, проблема була у видимості main.
- Повний CI для PR #173 = SUCCESS.
- Тимчасовий TEST PR #174 закрито WITHOUT MERGE; Windows runtime workflow `40883602454` = SUCCESS.
- Artifact ID `12902073405`: `MD-Notes-TEST-ADR0010-r6-6002b529-Windows-x64-portable`.
- Inner ZIP SHA256 `5c68f4f788d3e4cf7355c815c3c9823b4abb5f7d17141747fc348cc67451b95b`.
- `MD Notes.exe` SHA256 `2741e4135f271ea64c5e875ef21700bd27ef552c9b936aebd91df89240b7abb1`; size `29545984`; PE timestamp `2026-10-29 13:48:30 UTC`.
- ZIP завантажено локально й перевірено: checksum збігається, archive integrity test passed.
- **ADR-0010 залишається NOT DONE до owner runtime verdict для r6.** Головна перевірка: натиснути саме велику `Розгорнути` зі скріншота → graph відкривається окремо, main лишається видимим поруч; кліки по вершинах змінюють видиму нотатку в main.
- Release Please #154 не зливати до цього verdict.


## 29.10.2026 — ADR-0010 r6 owner screenshots → r7 split-workspace correction

- Owner screenshots show r6 still expresses the wrong product model even though the large `Розгорнути` now reaches the detached graph path.
- The intended model is not «duplicate the graph in another window», but «split one workspace across two windows»: main = note work surface; graph = navigation surface.
- Current wrong behavior visible in screenshots: main keeps an internal graph panel while detached graph is open; detached graph also contains its own note preview; on ultrawide 3440px screen graph is only 680px wide and becomes cramped.
- r7 target:
  - when graph detaches, hide/collapse the graph/link side panel in main so main becomes a clean note surface;
  - detached graph contains graph/tree navigation only, no note preview pane;
  - graph clicks continue routing the selected note to main;
  - widen detached graph to about 38% of monitor work area (bounded for normal screens) instead of max 680px;
  - keep explicit detached note/editor windows independent;
  - retain `Розмістити поруч` as a user action, but remove technical size diagnostics from normal UI.
- r3–r6 are not accepted as the intended multiwindow UX. Next issued Windows candidate is r7.

### Уточнення власника до r7

- `main` — центральне основне вікно; навколо нього будується весь інтерфейс.
- У `main` можуть залишатися вкладки, права панель, граф та інша інформація — це нормально.
- Коли граф від’єднують, він має поводитися **як від’єднана вкладка**: окреме самостійне вікно, яке можна переміщати/масштабувати незалежно.
- Від’єднання графа **не повинно автоматично змінювати розмір/позицію main і не повинно ховати його панелі**.
- Отже попередня частина r7 про автоматичне приховування правої панелі та примусове розкладання main+graph скасовується.
- Залишається: окреме graph-вікно, незалежне від main; кліки по графу можуть змінювати інформацію/вкладку в main; окремі editor-вікна працюють аналогічно.


## 29.10.2026 — ADR-0010 r7: остаточна модель від’єднаного графа, Windows TEST готовий

- Остаточне уточнення власника: `main` — центральне основне вікно. В ньому можуть залишатися вкладки, права панель, внутрішній граф та інша інформація.
- Від’єднаний graph має поводитися як від’єднана вкладка: окреме самостійне native-вікно, яке користувач сам переміщує/масштабує.
- Від’єднання graph **не змінює розмір/позицію main, не ховає його панелі й не розкладає вікна автоматично**.
- Desktop-дія тепер чесно підписана `Граф в окремому вікні`, а не `Розгорнути`.
- В окремому graph немає власного preview нотатки; клік по вузлу передає відкриття/перемикання нотатки в main.
- Автоматичне `main + graph поруч`, кнопка `Розмістити поруч`, технічна діагностика розмірів і зайві дозволи на програмне переміщення/resize вікон видалені.
- PR #176 merged → product-code main `9c876050844bc492034ed95d630f033e84b2bff6`; CI + Mobile smoke = SUCCESS.
- Тимчасовий TEST PR #177 закрито WITHOUT MERGE; Windows runtime workflow `40891762871` = SUCCESS.
- Artifact ID `12905198075`: `MD-Notes-TEST-ADR0010-r7-9c876050-Windows-x64-portable`.
- Inner ZIP SHA256 `30ced5b6ec3dc16a74b439bab0bdc699e719cf53451901b14bddcd4b994017ad`.
- `MD Notes.exe` SHA256 `8b97af362eca65a64cd55605138d36d964eb0312b3319ec3742343f815833413`; size `29401600`; PE timestamp `2026-10-29 14:37:08 UTC`.
- ZIP завантажено локально й перевірено: checksum збігається; archive integrity test passed.
- **ADR-0010 лишається NOT DONE до owner runtime verdict для r7.** Перевірка: від’єднати graph → main не змінюється; graph можна вручну перемістити/розтягнути; клік по graph відкриває/перемикає нотатку в main; від’єднані нотатки можуть існувати паралельно.
- Попередні r3–r6 та ранні r7-описи про автоматичне розкладання вікон вважати історією невдалого трактування; цей розділ є актуальною моделлю.
- Release Please #154 не зливати до owner verdict.
