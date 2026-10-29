# ADR-0010 — вкладки та незалежні вікна нотаток

**Статус:** частково реалізовано; **runtime FAIL цільового UX у v0.3.21 (28.10.2026)**.

## Мета

В одному сховищі користувач може одночасно читати й редагувати кілька нотаток: у вкладках головного вікна або в незалежних вікнах (у тому числі на другому моніторі). Зовнішній Markdown відкривається окремим документом, без імпорту до сховища. Окреме вікно графа вже існує, але не є заміною багатовіконного редактора.

## Запропонована модель

1. **Одна сесія сховища у ядрі — багато подань.** Кожне вікно має власний фокус, вкладки, режим редактор/перегляд і навігацію, але спільні дані сховища. Не викликати `open_vault` із дочірнього вікна як спосіб перемикання нотатки, бо це може замінити глобальну сесію.
2. **Ідентичність документа:** `vault-id + vault-relative path` або `external + canonical absolute path`. Відкриття/перетягування зовнішнього .md ніколи не створює файл у vault. Імпорт — окрема підтверджена команда.
3. **Вкладки:** відкрити, перемикнути, закрити; `Ctrl+Tab`, `Ctrl+W`; окрема ознака незбережених змін. З першого зрізу дозволити відкрити кілька нотаток, але редагування у двох вікнах одного файла не повинно перезаписувати правки мовчки.
4. **Незалежні вікна:** «Відкрити в новому вікні» і «Перемістити вкладку в нове вікно» через Tauri WebviewWindow з унікальними label. Переміщення/закриття вікна не знищує сховище й не закриває інші документи.
5. **Узгодження правок:** один документ має версію (fingerprint або revision); запис дозволений лише при збігу базової версії. Коли інший редактор або зовнішня програма змінили файл — повідомити про конфлікт і запропонувати перечитати чи зберегти окрему копію. Заборонене автоматичне «останній запис перемагає». У зашифрованих сховищах зберігати лише шифротекст за ADR-0007.
6. **Фокус і маршрутизація:** клік вершини у відокремленому графі показує preview у графі; «Відкрити в редакторі» відкриває нотатку в обраному редакторському вікні або створює нове за явною командою. Події мають бути адресними за вікном/документом, не глобальним безадресним broadcast.
7. **Безпека:** перед закриттям вкладки/вікна — збереження або явне рішення щодо dirty state; збої запису блокують тихе закриття. Ніякого фонового імпорту, копіювання або мережевої синхронізації.
8. **Відновлення сесії:** після перезапуску відновлювати тільки безпечний список шляхів і розташування вікон (за згодою); не класти plaintext зашифрованих нотаток у localStorage чи logs. Невдалі/відсутні шляхи — повідомлення, а не створення нових файлів.

## Етапи й приймання

- **M1:** модель відкритих документів і вкладки в одному вікні, тести переключення та незалежних чернеток.
- **M2:** незалежне editor window і адресні події, переміщення вкладки; безпечне закриття.
- **M3:** синхронізація змін/виявлення конфліктів, зовнішні file changes, readonly mode, encrypted vault.
- **M4:** Windows runtime acceptance на 2 моніторах, перевірка Linux/macOS, 7 мов, керівництва й доступності.

Обов'язкові тести: два вікна редагують той самий файл; зовнішній .md не імпортується; інше вікно зберігає нотатку; закриття з незбереженими даними; змінений на диску файл; кілька вікон + граф; зміна сховища; encrypted vault. Не видавати runtime PASS лише за зеленим CI.

## Взаємодія з іншими задачами

Backup ADR-0009 має брати консистентний snapshot, враховуючи незбережені чернетки **всіх** відкритих вікон. Групові дії з файлами повинні повідомляти всі вікна про rename/move/trash до операції. Git pull має перевіряти незбережені правки в усіх редакторах.

## Уточнення власника 27.10.2026: граф збоку, редактор у головному вікні

- **Погоджена поведінка:** незалежне вікно графа користувач ставить збоку від головного редактора. Натискання вершини графа **змінює активну нотатку в головному вікні**, не створюючи нове вікно для кожного натискання, не згортаючи граф і не примушуючи foreground/focus головного вікна. Це відповідає наявній маршрутизації події GRAPH_OPEN_NOTE.
- Preview у вікні графа може залишатися додатковим інструментом, але **не обов'язковим** шляхом для переходу. Граф залишається видимим при розташуванні вікон поруч.
- Окрема команда «Відкрити у новому вікні» стосується вибраної нотатки і має бути **явною**, не за звичайним кліком на вершину.
- Вкладки і додаткові незалежні editor-вікна потрібні для паралельного перегляду/редагування нотаток. Розв'язати узгодження dirty-state, конфліктів і спільної сесії vault.
- Попереднє повідомлення про нібито помилку відкриття за вершиною в v0.3.20 **уточнено**: запуск і відкриття в головному вікні підтверджені; проблема — розташування вікон та відсутність повноцінного багатовіконного режиму, а не неправильний destination.

## Уточнення 27.10.2026: винесення нотатки з головного редактора

- Для активної нотатки в головному редакторі потрібна кнопка **«Відкрити в окремому вікні»**, аналогічна кнопці відокремлення графа. Типова дія — **перемістити** активну вкладку/документ у нове незалежне вікно, а не створити другий одночасно редагований екземпляр.
- Після переміщення головне вікно повертається до попередньої доступної вкладки. Якщо інших вкладок немає, показує порожній стан редактора з відкритим сховищем та навігацією; з нього можна відкривати інші нотатки.
- Відокремлена нотатка лишається повноцінним редагованим документом. Переміщення передає поточну незбережену чернетку та версію дискового документа без втрат, не робить примусового збереження чи імпорту. Якщо передача не підтверджена, початкова вкладка лишається на місці.
- Закриття відокремленого вікна при dirty state потребує явного рішення користувача; повернення у вкладку може бути окремою командою. Вкладки в інших вікнах не зачіпати.
- Окрему функцію «Дублювати подання» відкласти до готового узгодження конкурентного редагування — за замовчуванням один документ має один активний редагований екземпляр.
- Натискання вершини графа, як погоджено раніше, **оновлює активну нотатку головного редактора**, не перекидаючи чи не закриваючи відокремлені вікна.


## Runtime результат v0.3.21 — 28.10.2026

Ручний тест власника показав, що реалізація не відповідає погодженій поведінці:

- відкривається окреме вікно графа;
- при кліках по вузлах графа main накопичує багато вкладок;
- граф поводиться фактично так само, як до ADR-0010;
- користувацький сценарій «graph + main + незалежні editor-вікна» не досягнуто.

Отже M1 (вкладки) фактично є, частина M2/M3 інфраструктури присутня, але **ADR-0010 не прийнято як багатовіконний режим**.

### Обов'язкова корекція

1. Звичайний клік вузла графа повинен змінювати активну нотатку main **без створення нової вкладки**.
2. Нове editor-вікно створюється лише явною командою користувача.
3. Після винесення нотатки main залишається робочим і може відкрити іншу нотатку.
4. Мають одночасно працювати graph + main + кілька незалежних editor-вікон.
5. Runtime acceptance проводиться на цій поведінці; зелені CI самі по собі не закривають ADR.


## Runtime результат r1 — 29.10.2026

Власник перевірив Windows test artifact після PR #153.

- **PASS частини multiwindow:** одночасно працюють graph + main + кілька незалежних editor-вікон; інша перевірена поведінка за словами власника виглядає робочою.
- **FAIL routing:** у багатовіконному сценарії вибір вершини графа маршрутизує/відкриває нотатку у фоновому/невидимому вікні з вкладками замість видимого робочого редактора.
- Отже ADR-0010 все ще не прийнято повністю. r2 має реалізувати адресний destination для graph navigation та прибрати залежність звичайного graph click від прихованого tab state.
- Runtime PASS ставиться лише після повторного тесту власником.


## Runtime correction r2 — 29.10.2026

Після owner test r1 підтверджено, що graph + main + кілька detached editor windows можуть працювати одночасно, але destination graph navigation інколи опинявся у фоновому/невидимому main.

r2 інтегровано PR #158:
- `GRAPH_OPEN_NOTE` з graph адресований конкретно `main` через `emitTo`, а не broadcast;
- detached editor windows не отримують цю навігацію і не змінюють власну note identity;
- main перед показом вибраної нотатки розгортається/показується та піднімається у Z-order, після чого focus повертається graph;
- replace-active routing, guarded save/conflict check та latest-request guard лишаються з r1.

CI = PASS, Windows test artifact = готовий. **Runtime verdict r2 ще відсутній, тому ADR-0010 лишається NOT DONE.**


## UX correction r3 — side-by-side замість Z-order

Owner уточнив фактичну причину проблеми: graph відкривається окремим великим центральним вікном і закриває main. Тому навіть правильне graph→main routing виглядає як «вкладка відкрилась у невидимому вікні».

Прийнятий напрям r3:
- команда розгортання/від’єднання graph автоматично розкладає main і graph поруч у work area поточного монітора;
- main лишається видимою областю зміни нотаток, graph — видимою областю навігації;
- graph click не створює нове editor-вікно й не потребує focus ping-pong;
- ручне переміщення/resize/maximize після detach не блокується.

r2 не вважати owner PASS; його addressed routing зберігається як основа, але Z-order підняття main замінюється на правильний початковий layout.

### Від’єднання окремих вкладок у r3

Side-by-side graph/main доповнюється явним per-tab detach: у кожної vault-вкладки є `↗`, який переносить саме цю вкладку в незалежне editor-вікно. Main лишається контейнером решти вкладок. Якщо від’єднано неактивну вкладку, поточний видимий документ main не змінюється.

Це прийнятий explicit workflow для r3. Drag-and-drop вкладки за межі main можна додати пізніше окремим slice, але він не є умовою приймання ADR-0010.


## Runtime candidate r3 — 29.10.2026

r3 replaces the r2 Z-order workaround with the intended workspace model:
- detached graph and main are automatically tiled side-by-side when there is enough monitor work area;
- graph remains navigation, main remains the visible editor destination;
- graph clicks route to main without raising/focus-bouncing windows;
- every vault tab can be explicitly detached via `↗` into its own editor window;
- detaching a non-active/background tab does not disturb the currently visible document;
- manual move/resize/maximize remains available after the initial layout.

Automated checks and Windows build are green, but **ADR-0010 remains NOT DONE until owner runtime PASS**.


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
