# PROJECT_STATE — MD Notes

Оновлено: **28.10.2026**

## Поточний стан

- Статус розвитку: **ACTIVE**. Оновлено: **28.10.2026**.
- Останній опублікований реліз: **v0.3.21 prerelease**, 28.10.2026 (версія — з GitHub Releases; машинні джерела: `Cargo.toml`, `app/package.json`, `app/src-tauri/tauri.conf.json`).
- PR #122 (кнопка «Назад» у вікні графа) інтегровано 27.10.2026. PR #128 зафіксував вимоги до групових операцій/вкладок. Вкладки вже реалізовано в ADR-0010 M1 через PR #142; групові операції ще не реалізовано. M2 незалежних editor-вікон інтегровано PR #144.
- README: UK / EN / FR / DE / ES / KO / JA; керівництво користувача — ті самі 7 мов, синхронізовано з v0.3.21.
- UI: ті самі 7 мов; усі ключі присутні в усіх мовах (перевірено скриптом 27.10.2026: основний словник 98 ключів, допоміжні словники git/graph/encryption/importer/lookup/relationTree/relations/startup — без розбіжностей). FR/DE/ES/KO/JA — машинні переклади, не вичитані носіями.
- Ліцензійна модель: proprietary / All Rights Reserved, Roman Zavada / Роман Завада.

## Релізи після v0.3.1

Усі — prerelease, зібрані GitHub Actions за release-please.

| Версія | Головне |
|---|---|
| 0.3.2-0.3.4 | зашифроване сховище (age, ADR-0007 зрізи 1-3), запобіжник push, випадкові імена файлів |
| 0.3.5 | git push через системний `git`, панель Git у застосунку |
| 0.3.6 | імпорт папки у сховище |
| 0.3.7-0.3.10 | граф у стилі Obsidian, лінії й обводи за оцінкою, картки зв’язків, стрілки напрямку |
| 0.3.11 | режим «звідки це прийшло» |
| 0.3.12-0.3.13 | об’єднання за кодом при імпорті (ADR-0008), список можливих дублікатів |
| 0.3.14 | дерево зв’язків, шлях між нотатками |
| 0.3.15 | жива фізика графа, індекс типізованих зв’язків (схема індексу v2) |
| 0.3.16 | граф в окремому вікні, єдиний пошук суб’єкта |
| 0.3.17 | форми вузлів, умовні позначення |
| 0.3.18 | найнадійніший шлях, фільтр за оцінкою, факти «З-під Ляди» за кодом у пошуку |
| 0.3.19 | кнопка «Назад» у вікні графа (PR #122); релізний checkpoint 27.10.2026 |
| 0.3.20 | graph preview + зовнішній Markdown поруч із vault; prerelease 27.10.2026 |
| 0.3.21 | вкладки + технічна основа detached editor/conflict guard; runtime-тест показав, що цільовий multiwindow UX не реалізований; prerelease 28.10.2026 |

Детально — `CHANGELOG.md` і GitHub Releases. Для v0.3.1 див. `docs/releases/RELEASE_NOTES_v0_3_1.md`.

Builds не вважаються підписаними/нотаризованими, якщо конкретний release прямо не підтверджує інше.

## Інтегрований v0.3 baseline

### Storage / mobile

- PR #30 — provider-neutral `StorageProvider` + `LocalFsProvider`;
- PR #31 — provider-backed `VaultStorage`;
- PR #32 — mobile sandbox vault + mobile startup flow;
- PR #35 — Android aarch64 + iOS simulator build smoke.

### Sync foundation

- PR #33 — `SyncManifest`, snapshots, upload/download/delete planning, visible conflict-copy policy;
- PR #37 — persistent atomic `.mdnotes/cache/sync-state.json`.

### Git foundation

- PR #39 — `gix` HTTPS clone/open foundation;
- PR #41 — dirty worktree detection;
- PR #42 — local Git commit pipeline;
- PR #43 — safe public HTTPS fetch;
- PR #44 — in-memory HTTPS authentication;
- PR #45 — native system credential storage;
- PR #59 — repository pull planner;
- PR #60 — pull safety preflight;
- PR #61 — safe fast-forward apply.

Поточна safety policy:
- clean `FastForward` — може бути автоматично застосований;
- dirty worktree — не чіпати;
- `UpToDate`, `LocalAhead`, `Diverged`, `Unrelated` — не мутувати автоматично;
- blind `hard reset` і silent overwrite заборонені.

### Standalone Markdown

- PR #54 — одиночний `.md/.markdown` без vault: open/edit/save;
- PR #56 — macOS Finder open-event + explicit handler action;
- Windows/Linux/macOS desktop integration без автоматичного takeover файлової асоціації.

## Межа доказу / відомі обмеження

- Windows v0.2.2 runtime — підтверджено власником (03.10.2026); пізніші версії в живому застосунку на Windows власник не підтверджував;
- Android/iOS buildability — підтверджено CI, physical-device runtime не підтверджений;
- зашифроване сховище (ADR-0007): зрізи 1-3 у main, випадкові імена файлів, запобіжник push; не реалізовано: зображення й вкладення в зашифрованому сховищі, інтерфейс додавання ключа відновлення до готового сховища, шифрування кеш-індексу;
- Git push реалізований через системний `git` (потрібен встановлений Git, лише desktop); у живому застосунку не перевірений;
- Git sync ще не зв’язаний повністю з `SyncManifest` / ADR-0005 conflict resolution;
- WebDAV не реалізований;
- Android SAF / iOS security-scoped bookmarks не реалізовані;
- відкриття й індексація великих сховищ може лишатися синхронним;
- граф не має ForceAtlas2/кластеризації/збереження розкладки;
- release builds unsigned unless explicitly stated otherwise.

## Наступний напрям

1. **ADR-0010 — виправлення multiwindow UX після runtime FAIL v0.3.21:** graph-node click не повинен накопичувати вкладки; потрібні справжні незалежні editor-вікна за явною командою.
2. M3 уже інтегровано: conflict guard, detached safe close та app-wide save-all/safe Quit.
3. Backup/restore за ADR-0009.
4. Git state ↔ `SyncManifest` / ADR-0005.
5. WebDAV.
6. Physical-device Android/iOS runtime validation.
7. Optional Android SAF / iOS security-scoped external folders.
8. Прогалини зашифрованого сховища (вкладення, ключ відновлення).

Перед новими dependency обов’язково проходити license gates та оновлювати `THIRD_PARTY_NOTICES.md`.

## Завершення сесії 27.10.2026 — актуальний checkpoint

- **main (перед фінальним docs PR):** `51e8f2487d09ad32a2dd8c076127d595228d43a2`. На час перевірки **відкритих PR немає**.
- **Останній опублікований prerelease:** [v0.3.20](https://github.com/RomanZavadaM/md-notes/releases/tag/v0.3.20), 13 assets, у тому числі Windows Portable [ZIP](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.20/MD-Notes-0.3.20-Windows-x64-portable.zip) і SHA256SUMS; Release #112 — успішно (Windows, macOS, Linux, публікація, README links).
- PR #132: перегляд нотатки у відокремленому графі; PR #133: зовнішній Markdown у окремій вкладці при відкритому vault, без автоматичного імпорту. Входять до v0.3.20. **Windows runtime acceptance цих змін ще не підтверджено**.
- PR #134 release-please merged, PR #137 синхронізація документації merged, PR #138 ADR-0010 (багатовіконний режим) merged. **ADR-0010 — тільки проєктне рішення, функція ще не реалізована**.
- ADR-0009 backup/restore — запропоновано, **немає реалізації**. Підготовчої гілки може вистачати без коду: звірити перед роботою; не плутати з інтегрованою функцією.
- Вимога власника на завтра: **кілька вкладок і незалежних вікон редактора нотаток**; обрати маленький безпечний зріз M1 (модель документів/вкладок), з тестами та конфліктним захистом, потім M2 (незалежні вікна). Врахувати graph, зовнішні файли, спільний vault і незбережені зміни в усіх вікнах. Backup/restore не загубити (ADR-0009) і узгодити зі станом вкладок.
- Наступний старт: перевірити фактичні `main`/open PR/releases/actions, `PROJECT_RULES.md`, `PROJECT_STATE.md`, `WORKLOG.md`, ADR-0009 та ADR-0010, Issue #8. Жодних тверджень про runtime PASS до тесту власником. Для релізів обов'язкові PR checks, tag, Portable, SHA256SUMS, документація та 7 мов.


## Checkpoint 28.10.2026 — багатовіконний редактор

- `main`: `34723be6d85654f4b4561c7318abf79d93893cee` після merge PR #144.
- **M1 / PR #142:** кілька vault-нотаток у вкладках, незалежні чернетки/стан збереження, без втрати тексту при перемиканні.
- **M2 / PR #144:** явне перенесення активної нотатки у власне Tauri editor-вікно; перед перенесенням збереження; після успішного відкриття — попередня вкладка або порожній main; edit/split/preview + autosave у відокремленому вікні.
- M2 має CI #411 SUCCESS та Mobile smoke #180 SUCCESS. Це **build/CI proof, не runtime acceptance**.
- Кнопка M2 локалізована всіма 7 мовами. Поведінка відокремленого графа не змінювалась.
- M3 інтегровано PR #146/#147/#148; фінальний `main` `1b33390b9bb70f18b60bea1c7c9492c5a89a5098`, CI #417 і Mobile smoke #185 SUCCESS. Наступне: M4. ADR-0009 backup/restore не реалізований.


## Checkpoint 28.10.2026 — M3 завершено

- Conflict guard не дозволяє main-tab або detached editor мовчки перезаписати новішу дискову версію.
- Detached editor перехоплює native close для dirty draft і закривається лише після успішного guarded save.
- Main перед switch vault / Quit зберігає всі відкриті документи; app Quit спочатку проходить safe-close усіх detached editor-вікон.
- PR #146 → `9089fa146f6ab443d18b09a0eed662cd2336ab9b`; PR #147 → `c2cc7c955a80325306ae98b3a21cef34b2ebf72e`; PR #148 → `1b33390b9bb70f18b60bea1c7c9492c5a89a5098`.
- Автоматичні перевірки зелені; ручна runtime-перевірка M4 ще попереду.


## Checkpoint v0.3.21 — 28.10.2026

- GitHub prerelease v0.3.21 опублікований і містить desktop-пакети Windows/macOS/Linux та контрольні файли релізу.
- Технічна основа ADR-0010 M1–M3 є у v0.3.21, але ручний Windows runtime 28.10.2026 показав **FAIL цільового UX**. Не вважати багатовіконний режим завершеним.
- UI localization parity: 7/7 мов, 98/98 основних ключів у кожній.
- Відомі незавершені можливості не маскуються: backup/restore (ADR-0009), Git↔SyncManifest, WebDAV, physical Android/iOS runtime, encrypted-vault gaps, signing/notarization.


## Runtime correction — 28.10.2026

Фактичний тест власника спростував попереднє оптимістичне формулювання документації. При роботі з окремим графом клік по вузлах лише відкриває/накопичує вкладки в головному вікні. Отже v0.3.21 не змінив базовий робочий сценарій настільки, як було погоджено.

Статус ADR-0010: **частково реалізована інфраструктура, цільовий multiwindow UX — NOT DONE / runtime FAIL**.


## Інтегрований checkpoint 29.10.2026 — ADR-0010 graph routing r1

- PR #153 інтегровано в `main`: `9a2db3532b63db2e7c9f52c05902d4f10a87cb24`.
- Graph-node navigation тепер має окремий routing у main: target замінює активний document slot замість append нової вкладки; already-open target лише активується.
- Dirty active document перед replacement проходить conflict-guarded save; додано захист від race при швидких кліках. Примусовий focus main прибрано.
- `GraphWindow` зберігає follow/preview/history та надсилає `GRAPH_OPEN_NOTE` зі звичайного node click. Explicit detached editor залишається лише явною дією користувача.
- Автоматична перевірка PR #153: CI `40841554655` — SUCCESS. **Ручний Windows runtime acceptance ще не виконаний; статус повного ADR-0010 multiwindow = NOT DONE до тесту власником.**
- Останній опублікований prerelease лишається `v0.3.21`; новий реліз цим checkpoint не публікувався.


## Runtime-test checkpoint 29.10.2026 — ADR-0010 r1

- Для інтегрованого graph-routing r1 зібрано окремий Windows Portable TEST artifact без публікації нового релізу.
- Source/main checkpoint: `c6d5d398ab0db0b3272dc966c7e45bd2016d3887`; Actions run `40843601078` = SUCCESS; artifact `12884352283`.
- Portable SHA256: `3d261922637fb29db3eb5a57ed42637095966a265f3e6f8c13a6d4fb4e69de1f`.
- PR #156 був лише тимчасовою build-гілкою і закритий без merge. Останній published prerelease лишається `v0.3.21`; PR #154 для `0.3.22` лишається відкритим і незлитим.
- ADR-0010 все ще **NOT DONE** до ручного Windows acceptance власником.


## Інтегрований checkpoint 29.10.2026 — ADR-0010 r2

- `main`: `66eb9728bf9f0dadaf59d75a84ac3dde8ae30d17` (PR #158).
- r2 виправляє залишкову проблему видимості destination при graph navigation у multiwindow: подія адресована тільки `main`; main показується/розгортається та піднімається у Z-order, після чого focus повертається graph.
- Detached editor windows не є destination звичайного graph click; їхній документ не змінюється.
- Автоматичні checks PR #158: run `40853765572` = SUCCESS.
- Windows test artifact r2 зібраний run `40854135789` = SUCCESS; SHA256 Portable ZIP `3d8cf4ad903e99c25cf9002ba7d4accd45645917a0a1b9e4536eb7ba1fd6280a`.
- **Runtime acceptance власником ще не виконано; ADR-0010 не закрито.** Останній опублікований prerelease залишається v0.3.21.
- Наступний підтверджений функціональний блок після owner PASS: ADR-0009 — ручне резервне копіювання/відновлення поточного робочого vault. Воно спроєктоване, але коду функції ще немає.


## Інтегрований checkpoint 29.10.2026 — ADR-0010 r3

- Product code integrated in `main`: `b0f7813c6820cfd5eefb92f0c414d723aab93147` (PR #162).
- Detached graph initial layout is side-by-side with main when monitor work area allows; small screens retain safe centered fallback.
- Graph node navigation targets main; no normal focus/Z-order ping-pong.
- Every vault tab has explicit `↗` detach into an independent editor window; background-tab detach preserves the currently visible surface.
- CI `40859261723` = SUCCESS; Mobile smoke `40859261721` = SUCCESS.
- Windows TEST r3 workflow `40860534312` = SUCCESS; SHA256 Portable ZIP `26fa666bc316d360b9bb2f8de5f950c2eeb548b4b523bb9b4511e67543766fe4`.
- **Owner runtime acceptance is still pending; ADR-0010 is NOT DONE yet.**
- Next after PASS: ADR-0009 backup/restore current working vault.


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


## 29.10.2026 — ADR-0010 r8: окремий graph + нормальні вкладки main, Windows TEST готовий

- Owner runtime r7: **модель окремого graph-вікна правильна**. Main лишається центральним; graph від’єднується незалежно.
- Runtime-дефект r7: коли main уже мав вкладку, кліки по graph не відкривали очікувані нові вкладки; працював переважно випадок порожнього main.
- Причина: `openGraphNote` мав окрему логіку заміни активної вкладки замість звичайного відкриття нотатки.
- r8 PR #179 merged → product-code main `654b9d17f8f022ef5223e6c350055fbd0d0eb3ef`.
- r8 змінює лише graph→main routing:
  - якщо цільова нотатка вже відкрита → активувати її вкладку;
  - якщо не відкрита → додати нову вкладку;
  - усі інші вкладки зберігати;
  - detached graph із r7 не змінювати.
- Повний CI для PR #179 = SUCCESS.
- Тимчасовий TEST PR #180 закрито WITHOUT MERGE; Windows runtime workflow `40927473259` = SUCCESS.
- Artifact ID `12921486986`: `MD-Notes-TEST-ADR0010-r8-654b9d17-Windows-x64-portable`.
- Inner ZIP SHA256 `fb76e95cfaac1b201566582f1f4c5bf52c002d77086121fb46df3162378b4236`.
- `MD Notes.exe` SHA256 `01935160b37089122a9196dde4cab666f09fc309b9923d826145a42618a8bc0a`; size `29401088`; build timestamp `2026-10-29 18:12:56 UTC`.
- ZIP завантажено локально й перевірено: checksum збігається; archive integrity test passed.
- **ADR-0010 лишається NOT DONE до owner runtime verdict r8.** Перевірити: main з 1+ вкладками; кілька різних вузлів graph повинні додавати/активувати вкладки в main без втрати вже відкритих.
- Release Please #154 не зливати до owner verdict.


## 29.10.2026 — функціональний checkpoint перед v0.3.22

- ADR-0010 multi-window: **прийнято власником** на r8. Main — центральне вікно; graph від’єднується незалежно; graph-вузли відкривають/активують звичайні вкладки main.
- ADR-0009 backup/restore: **реалізовано та інтегровано** PR #182 → `71ba9604c30b8399137d79011f2bd97432b9505a`.
- Backup/restore: portable ZIP + manifest/SHA-256; encrypted vault без plaintext/secret key; restore тільки в нове сховище після повної валідації.
- PR #182 CI: Linux/macOS/Windows SUCCESS; Android/iOS smoke SUCCESS; frontend/Tauri/license checks SUCCESS.
- README та user guides синхронізовано всіма 7 мовами.
- Важлива межа доказу: owner вручну перевірив multi-window r8, але **не проводив окремого ручного backup/restore runtime-тесту**; backup покривається автоматичними тестами трьох desktop OS.
- Поточний product-code main: `71ba9604c30b8399137d79011f2bd97432b9505a`.
- Наступна єдина дія: завершити Release Please PR #154 → v0.3.22 prerelease, перевірити release workflow та 13 assets, після цього синхронізувати цей checkpoint із фактичним release SHA/tag.
