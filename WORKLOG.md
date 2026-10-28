# WORKLOG — MD Notes

Оновлено: **28.10.2026**

## STATUS

**ACTIVE. Останній реліз v0.3.21 (28.10.2026). Runtime-тест власника: multiwindow UX FAIL; виправлення ADR-0010 є наступною задачею.**

Оновлено: 27.10.2026. Нижче розділи за 18-19.10 — історія (запис стану на ті дати); поточний стан — у `PROJECT_STATE.md`.

Release baseline (актуально 28.10.2026): tag/prerelease `v0.3.21`; активи Windows/macOS/Linux, START/source, legal, `SHA256SUMS.txt` опубліковані.

## Runtime / build evidence

- Windows v0.2.2 runtime: PASS від власника 03.10.2026.
- Android/iOS build smoke: CI evidence only, не physical-device runtime.
- PR #61 final head `8bac0f284dfbe2fab21ad8077b3b9d40868f6403`: CI #303 SUCCESS + Mobile smoke #99 SUCCESS.
- Release PR #52 final head `909ce415e06d13a7e3556223aaf659d9ba2be88f`: CI #309 SUCCESS + Mobile smoke #104 SUCCESS.

## Нещодавно завершено

### Standalone Markdown

- PR #54 merged: `c940dcc7aa2f8a4028b7ba4a8a15862070b7907b`.
- PR #56 merged: `a6429a40f87827e710dbd2e6ebb7b9e6df194375`.
- `.md/.markdown` open/edit/save без vault.
- macOS Finder open-event підтримується.
- Системна асоціація лише після explicit user action.

### Git pull planner / safety / apply

- PR #59 merge: `ad0444eee38d81236d1f60c20e6bfaaf73f7f6f3`.
- PR #60 merge: `d9d4d5fd3becf77d1587981733e6f442881ae6a9`.
- PR #61 merge: `ed924059fa68bc31014c5b484fb78daaeba675b0`.
- clean FastForward-only automatic apply;
- dirty worktree blocks mutation;
- LocalAhead / Diverged / Unrelated не мутуються;
- no blind hard reset / silent overwrite;
- optimistic ref update + rollback attempt on race.

### Release v0.3.1

- Release PR #52 merge/release commit: `95d95264edf6382c7eb812a0d2559f4c21455b76`.
- Tag/prerelease: `v0.3.1`.
- Release workflow #57: SUCCESS.
- Assets published:
  - Windows setup EXE;
  - Windows MSI;
  - Windows x64 portable ZIP;
  - macOS universal DMG;
  - macOS app tarball;
  - Linux AppImage;
  - Linux deb;
  - Linux rpm;
  - START/source ZIP;
  - LICENSE/COPYRIGHT/THIRD_PARTY_NOTICES;
  - SHA256SUMS.
- README UK / EN / FR / DE / ES / KO / JA синхронізовані.
- Нових user-visible UI strings у PR #61 не було; app i18n keys не потребували змін.

## GitHub Actions optimization

- PR #65 `ci: reduce redundant Actions runs` merged to `main` as `b94edb7a84896b0f8694e3d548398e1187d48c43`.
- Validation before merge: CI #317 **SUCCESS**; Mobile smoke #107 **SUCCESS** (Android + iOS).
- Docs-only pull requests no longer start the heavy desktop CI matrix.
- CI no longer repeats automatically on every push to `main`; PR validation is the primary code gate.
- Release-please PRs skip the heavy Rust/Tauri matrix and use a lightweight version-consistency check instead.
- Mobile smoke skips draft PRs and release-please PRs, while remaining required for ready/non-draft code PRs that touch app/core/mobile-relevant paths.
- Android/iOS smoke jobs now use Rust cache.
- Release builds now use npm cache.
- Docs-only and workflow-only pushes to `main` do not run release-please.
- Real release packaging is deliberately **not** configured with `cancel-in-progress`.
- PR #67 `ci: route checks by changed paths` merged to `main` as `68655c50fa43da74820ea38e723901f2ff6d15e7`.
- PR #67 validation: CI #318 **SUCCESS**; Mobile smoke #108 **SUCCESS** (Android + iOS).
- CI now classifies changed files: frontend-only changes avoid Rust/Tauri matrix; dependency licenses run only when relevant; unknown non-doc paths fall back to full validation.
- Draft PRs avoid heavy CI until `ready_for_review`.
- Mobile smoke paths are narrowed to native/core/dependency files, so pure frontend/i18n changes no longer build Android/iOS.
- PR #69 `ci: ignore docs-only commits for releases` merged as `9a2c2066554975346ac5ca584a36c565619c5821`; CI #319 **SUCCESS**.
- Release workflow #60 after that merge: **SUCCESS**, package build/assets correctly **SKIPPED** (no product release).
- Stale release-please PR #63 `chore(main): release 0.3.2`, generated only from documentation commits, was closed unmerged.
- Future docs/ci/chore-only work must not create a product version; the next release PR should appear only after release-driving product changes.

## Наступна дія (оновлено 27.10.2026)

PR #122 вже інтегрований, реліз v0.3.19 опублікований. Інтегрований #128 — запис вимог (без коду): групові дії Ctrl/Shift, вкладки редактора; перед новою реалізацією перевірити #128 і наявні гілки. Далі основний roadmap: Git state ↔ `SyncManifest` / ADR-0005; також окремий UI backlog. Наступний документаційний крок — інтегрувати цей checkpoint, після чого оновити Issue #8. Нижче — історичний запис від 19.10.

## Поточна наступна дія (історичний запис 19.10.2026)

**ADR-0007 (зашифроване сховище, age): docs-PR з ADR і змінами правил, далі зріз 0 (дослідження), 1 (`EncryptedProvider`), 2 (індексатор і вкладення), 3 (інтерфейс і ключі в системному сховищі секретів). Кожен зріз - окрема гілка й PR від актуального `main`; перевірки - CI на GitHub. Git push - одразу після зрізу 3.**

Запобіжники ADR-0007: режим опційний; збірка без feature `encrypted-vault` і звичайні сховища - як раніше; відкритий текст не пишеться на диск у зашифрованому режимі; крейт age - лише після `cargo deny check licenses`.

Нижче - вимоги безпеки для Git push, що лишаються чинними:

Safety requirements для Git push мають зберігати існуючі trust boundaries:
- HTTPS only;
- credentials тільки через системне secret storage / in-memory callback;
- не записувати token у URL, Git config, vault або логи;
- не переписувати remote history мовчки;
- force push не вводити як implicit behavior;
- спершу planner/preflight + tests, потім mutation.

## Після Git push

1. Git state ↔ `SyncManifest` / ADR-0005 conflict policy.
2. WebDAV adapter.
3. Physical Android/iOS runtime validation.
4. Optional Android SAF / iOS security-scoped external folders.

## Відомі обмеження (актуально на 27.10.2026)

- Android/iOS physical-device runtime untested;
- encrypted vault: немає вкладень/зображень, немає інтерфейсу ключа відновлення для готового сховища, кеш-індекс відкритий;
- Git push працює через системний `git` (desktop), у живому застосунку не перевірений;
- complete conflict workflow not implemented;
- WebDAV not implemented;
- SAF/bookmark external-folder adapters not implemented;
- large vault open/indexing can still be synchronous;
- graph still lacks ForceAtlas2/clustering/layout persistence;
- builds are unsigned unless explicitly stated otherwise.

## ADR-0007, зріз 0 — висновки дослідження (19.10.2026)

Де ядро читає/пише файли поза `StorageProvider`:

- `Vault::open(root)` жорстко створює `LocalFsProvider`; потрібен конструктор з довільним провайдером (`Vault::open_with_provider(root, provider)`; `root` лишається для кешу, шаблонів і службових операцій).
- Через `VaultStorage` уже ідуть: `tree`, `note_paths`, `read_note`, `write_note`, `unique_path`. Шифрування в `EncryptedProvider` працює для них без змін логіки сховища.
- Напряму через `std::fs` (потрібно перевести на провайдера або явно обмежити): `Vault::rename`, `Vault::move_to_trash`, `Vault::create_folder`, `Vault::init` (теки шаблонів), `Vault::cache_dir`; `index.rs::file_stamp` (`fs::metadata(root + rel)` - для логічного шляху `a.md` у зашифрованому сховищі такого файла немає, є `a.md.age`); `attachments.rs`, `templates.rs`, `presets.rs`, `sync.rs`, `schema.rs` (по кілька викликів `fs::`).
- `git_sync.rs` (gix над справжнім робочим деревом) ортогональний: Git бачить `*.age` як звичайні двійкові файли.
- Трейт `StorageProvider` не має `rename`; потрібне додавання методу (за замовчуванням: читання + запис + видалення; `LocalFsProvider` - `fs::rename`), інакше `EncryptedProvider` не зможе зберегти відповідність «логічний шлях ↔ файл `.age`» при перейменуванні й кошику.
- Службові шляхи `.mdnotes/` (налаштування, схема, список публічних ключів) лишаються відкритими: `EncryptedProvider` пропускає їх без шифрування. Кеш `.mdnotes/cache` і кошик `.mdnotes/trash` і так поза синхронізацією (ADR-0002).
- Назви нотаток: `Note::parse` бере `title` з властивості `title`, інакше з першого заголовка, інакше з імені файла; граф несе `title`. Для випадкових імен файлів потрібно лише перевірити, що дерево, швидке відкриття й пошук показують `title` (зріз 3).

План зрізу 1 за цими висновками: (1) `StorageProvider::rename`; (2) `Vault::open_with_provider`; (3) `EncryptedProvider` за feature `encrypted-vault` (крейт age після `cargo deny check licenses`); (4) `index.rs::file_stamp` через провайдера; (5) тести: круг туди-назад, кілька одержувачів, неправильний ключ, пошкоджений шифротекст, список без суфікса, rename/кошик, відсутність відкритого тексту у викликах внутрішнього `write`, звичайне сховище й збірка без feature - як раніше.

## ADR-0007, зрізи 1-3 виконано (19.10.2026)

- Зріз 1-2 (PR #73-#75): `StorageProvider::rename`, `Vault::open_with_provider`, `EncryptedProvider` (age, feature `encrypted-vault`), ядро через провайдера.
- Зріз 3а (PR #76): `encrypted_vault` - `encryption.json` (формат, `vaultId`, публічні ключі), створення/відкриття/додавання одержувача з перешифруванням.
- Зріз 3б (PR #77): feature застосунку `encrypted-vault` (за замовчуванням увімкнена), приватний ключ у системному сховищі секретів за `vaultId`, команди `encryption_status`, `create_encrypted_vault_cmd`, `import_encryption_key`, `add_encryption_recipient`; `open_vault` відкриває зашифроване сховище або повертає `KEY_MISSING`; події змін файлів без суфікса `.age`.
- Зріз 3в (PR #78): прапорець шифрування при створенні (+ необов'язковий публічний ключ відновлення), одноразовий показ приватного ключа з підтвердженням, діалог імпорту ключа, позначка «Зашифровано», рядки 7 мовами.
- Відомі прогалини (наступний зріз): зображення й вкладення в зашифрованому сховищі не показуються (протокол asset віддає шифротекст; потрібна команда читання байтів і blob-адреси); немає інтерфейсу «додати ключ відновлення» для вже створеного сховища (команда є); індекс `.mdnotes/cache` лишається відкритим текстом (прийняте для MVP); перевірено лише CI (Linux/Windows/macOS/Android/iOS, clippy, фронтенд), у живому застосунку на Windows ще не запускалось.

## ADR-0007, запобіжник push (19.10.2026)

- `notes_core::push_guard` (feature `encrypted-vault`): `find_plaintext` / `ensure_push_safe` перевіряють робоче дерево зашифрованого сховища: усе, крім відкритих службових файлів (`.mdnotes/` без `cache` і `trash`, `.gitignore`, `.gitattributes`), має закінчуватись на `.age` і починатись із заголовка age; інакше push відхиляється з іменем першого файла. Звичайні сховища не перевіряються. Історія попередніх комітів не скануться (зашифроване сховище створюється в порожній папці).
- Команда застосунку `push_safety` повертає список порушень; майбутня команда push зобов'язана її викликати.
- Відкрито: транспорт push. `gix` 0.88 не має клієнта send-pack; варіанти: (А) системний `git` лише для push (суперечить принципу «не залежати від системного Git», потрібен лише на desktop, токен через змінні середовища), (Б) власний клієнт smart-HTTP receive-pack із генерацією packfile (великий обсяг). Рішення за власником.

## ADR-0007, випадкові імена файлів (19.10.2026)

- Нові зашифровані сховища (`encryption.json`: `"names": "random"`) зберігають кожен файл як `<16 випадкових символів>.age` у корені (файли корзини - у `.mdnotes/trash/`). Назви нотаток і папок не видно ні в файловій системі, ні в Git. Старі сховища без поля `names` працюють як раніше (`named`).
- Шлях лежить усередині шифротексту: конверт `MDN1
<шлях>
<вміст>`. Провайдер тримає в пам'яті відповідність «шлях - файл», яку відновлює за заголовками нових і змінених файлів (розмір і час зміни), тому зміни з іншого пристрою (Git pull, синхронізована папка) підхоплюються.
- Порожні папки зберігаються маркерним файлом `.mdnotes-dir` усередині папки; для цього в `StorageProvider` додано `create_dir`. Перейменування файла лише переписує конверт у тому самому файлі.
- Обмеження: перейменування папки перешифровує всі її файли; два пристрої, що одночасно створили файл з однаковим шляхом, дадуть два файли (видимий буде один).

## Гілки репозиторію (знімок 27.10.2026; нічого не видалено)

Видаляти гілки можна лише за прямим «так» власника.

- **Злитий PR (57 гілок, безпечні до видалення за рішенням власника):**
  `build/windows-portable-release`, `chore/actions-cost-optimization`, `chore/actions-path-routing`, `chore/release-please-ignore-docs`, `chore/release-v0.3-line`, `ci/mobile-build-smoke-v0.3`, `docs/actions-optimization-checkpoint`, `docs/actions-release-policy-final`, `docs/actions-routing-policy`, `docs/graph-guide`, `docs/post-presets-sync`, `docs/post-schema-sync`, `docs/post-v0.3.0-checkpoint`, `docs/release-v0.2.2`, `docs/standalone-markdown-checkpoint`, `docs/standalone-markdown-reader-checkpoint`, `docs/v0.2-functional-complete`, `docs/v0.2.1-checkpoint`, `docs/v0.3-checkpoint`, `docs/v0.3.1-checkpoint`, `docs/v0.3.1-download-links`, `feat/graph-window`, `feat/identity-merge`, `feat/live-graph`, `feat/node-glyphs`, `feat/possible-duplicates`, `feat/registry-facts`, `feat/relation-tree`, `feat/relations-index`, `feat/reliable-path-and-grade-filter`, `feat/unified-search`, `feature/attachments-v0.2`, `feature/git-fast-forward-apply-v0.3`, `feature/git-fetch-v0.3`, `feature/git-local-commit-pipeline-v0.3`, `feature/git-local-commit-v0.3`, `feature/git-pull-plan-v0.3`, `feature/git-pull-preflight-v0.3-r2`, `feature/git-pull-repository-plan-v0.3-r3`, `feature/git-sync-gix-v0.3`, `feature/git-token-auth-v0.3`, `feature/knowledge-graph-v0.2`, `feature/macos-markdown-association-v0.3`, `feature/mobile-sandbox-v0.3`, `feature/note-types-schema-v0.2`, `feature/standalone-markdown-reader-v0.3`, `feature/storage-provider-v0.3`, `feature/sync-foundation-v0.3`, `feature/sync-state-store-v0.3`, `feature/system-secret-store-v0.3`, `feature/vault-presets-v0.2`, `feature/vault-storage-provider-v0.3`, `fix/graph-declutter`, `fix/release-please-post-v0.3.0`, `fix/release-version-policy-v0.2`, `fix/runtime-startup-freeze`, `security/csp-v0.2`
- **PR закрито без злиття (4):** `docs/standalone-markdown-complete`, `feature/git-pull-preflight-v0.3`, `feature/git-pull-repository-plan-v0.3`, `feature/git-pull-repository-plan-v0.3-r2` (схоже, роботу згодом злито іншими PR #59-#61, але це не перевірялось; перед видаленням перевірити).
- **Без PR (2):** `test/publish-windows-runtime`, `test/windows-runtime-v0.2` (тестові гілки Windows).
- **Службова гілка release-please** (`release-please--branches--main--components--md-notes`): потрібна автоматиці релізів, не видаляти.
- **Відкритий PR (1):** `feat/tree-back-step` (PR #122).

**Оновлення 27.10.2026:** гілки зі злитими PR (усього 59, разом із гілками PR #122-#125) видалено за прямим «так» власника. Лишилися: `main`, службова `release-please--...`, 4 гілки із закритим без злиття PR (`docs/standalone-markdown-complete`, `feature/git-pull-preflight-v0.3`, `feature/git-pull-repository-plan-v0.3`, `-r2`) і 2 тестові (`test/publish-windows-runtime`, `test/windows-runtime-v0.2`). Список вище - історичний знімок.

## Майбутні правки (записано 27.10.2026 зі слів власника)

- Назва сховища у верхній панелі має бути «ПочатокПошуку»; зараз на екрані «ПочатокПршуку» (описка). Назва береться з `vault.display_name()` (назва в налаштуваннях сховища або імена папки), тобто помилка в даних власника, а не в коді; виправити назву в налаштуваннях сховища або перейменувати папку. Можливо, додати в застосунок зміну назви сховища.
- Кнопку «У кошик» (🗑) на знімках 27.10.2026 видно блідою, але власник просив нічого з нею поки не робити.

- Запит власника 27.10.2026 (PR #128, merged): групові операції в дереві файлів (Ctrl/Shift, пакетне переміщення, кошик, перейменування) та декілька вкладок редактора. **Вкладки реалізовано в M1 / PR #142; групові операції не реалізовано.** Для encrypted vault — виключно через `StorageProvider`.

- **Backup/restore сховища (новий запит власника 27.10.2026; не реалізовано):** ручне створення цілісної резервної копії сховища (ZIP або інший відкритий документований архів), перевірка контрольних сум/маніфесту, відновлення за замовчуванням у нове сховище без перезапису існуючих файлів. Врахувати консистентність під час змін файлів, вкладення й налаштування. Для encrypted vault зберігати **лише шифротекст**, не експортувати приватні ключі чи секрети автоматично, попереджати про необхідність окремого ключа відновлення. Перед реалізацією описати ADR і тести round-trip, corruption, conflicts, encrypted/unencrypted. Розклад та місце копій — окреме рішення власника.

## Перевірка поведінки 27.10.2026 — граф / standalone Markdown

- **Підтверджено за кодом, UX-дефект:** у від'єднаному/розгорнутому вікні графа `GraphWindow.tsx` клік по вузлу викликає `emit(GRAPH_OPEN_NOTE, path)`. Обробник `App.tsx` відкриває нотатку в **основному** вікні (`openNote` + `setFocus()`); окреме графове вікно не відображає вміст нотатки. Якщо граф накриває головне вікно, здається, що нічого не сталося. Потрібне проєктування видимого в графовому вікні читального перегляду/панелі нотатки (або явного вибору «відкрити у головному вікні») зі збереженням навігації «Назад»; не змінювати файли при простому виборі вузла. Runtime UI ще не перевірено.
- **Підозра власника, не підтверджена на перевіреному code path:** при відкритті звичайного .md для читання він нібито потрапляє у сховище. У `App.tsx` `pickMarkdownFile/openStandaloneAt` перемикають режим standalone (`setVault(null)`, `setStandalone(document)`), `open_markdown_file` у `lib.rs` викликає `load_standalone_markdown` (`VaultStorage::open` + `read_note`); `VaultStorage::open` описаний і реалізований як non-mutating. Імпорт використовує окрему явну `import_folder`. Треба відтворити фактичний випадок на Windows, перевірити диск/дерево сховища до та після відкриття, OS association/drag-drop/поточний режим і відрізнити read-only перегляд від редактора із явним збереженням. Не робити висновок, що дефект відсутній, без runtime-тесту.
- **Вимога:** зовнішні файли не імпортувати та не копіювати у vault без явної команди користувача; режим «читання» не повинен писати у вихідний файл без редагування/підтвердження.
- Пріоритет: виправити UX графа окремим функціональним PR із перевіркою локалізацій (7 мов), тестом події та runtime-перевіркою; розслідувати standalone на реальному сценарії без небезпечних змін файлової системи.

## Активний зріз — перегляд нотатки у вікні графа (27.10.2026)

- Кандидат PR #132, гілка `fix/graph-window-note-preview-r1`: клік вузла в окремому графі показує Markdown в бічній панелі **цього** вікна, а не відкриває його приховано в головному; явна кнопка відкриває в редакторі; повернення назад перемикає перегляд. Рядки для 7 мов. Вибір виконує лише `readNote`, не зберігає/імпортує.
- **Ще не підтверджено:** frontend CI, ручний runtime у Windows, робота вкладень/внутрішніх wiki-link у панелі перегляду. Не зливати до зеленої перевірки й огляду. Зовнішні standalone .md під час code audit не імпортуються; потрібна окрема runtime-репродукція повідомлення власника.

## Checkpoint 27.10.2026 — граф і зовнішні Markdown

- PR #132 (`fix/graph-window-note-preview-r1`) **злитий у main**, merge SHA `5a21ce426b279e41aff4010ef8505c285643aa1a`, frontend CI #401 SUCCESS. При натисканні вузла графа нотатка відкривається в панелі того самого вікна (read-only), явна кнопка відкриває її в головному редакторі. Перегляд вкладень/wikilinks і Windows runtime ще не перевірено.
- PR #133 (`fix/external-markdown-tab-r1`) **злитий**, merge SHA `ed3de2333e356b0d0927d9e45a34286f5ee677b0`, CI #405 SUCCESS. Зовнішній `.md` відкривається в окремій вкладці при активному vault, без імпорту, з окремим чернетковим вмістом і збереженням у вихідний файл. Windows Explorer runtime та перевірка на відсутність побічних файлів у vault — незавершені.
- Після цих merge відкритих PR немає. Останній перевірений GitHub prerelease — v0.3.19; нові merged зміни не позначати перевіреним релізом до фактичного випуску. GitHub release workflow #110 SUCCESS, але новий тег у Releases поки не підтверджений. Потрібний runtime acceptance та release checkpoint окремо.

## Release checkpoint 27.10.2026 — v0.3.20

- PR #134 release-please злитий: `7b50d7d92646b13aec62b8aed74e293413ad9a91`; GitHub Release #112 (`40569474410`) — **SUCCESS** для Windows/macOS/Linux + assets/readme-links.
- Опубліковано prerelease `v0.3.20`, 13 assets: Windows portable ZIP `MD-Notes-0.3.20-Windows-x64-portable.zip`, EXE/MSI, macOS DMG, Linux AppImage/DEB/RPM, START, legal та `SHA256SUMS.txt`.
- Ця версія включає PR #132 preview всередині графа та PR #133 зовнішні Markdown-вкладки. **Ручна Windows runtime-перевірка ще не підтверджена.**
- Завантаження: https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.20/MD-Notes-0.3.20-Windows-x64-portable.zip

## Завершення сесії 27.10.2026 — актуальний checkpoint

- **main (перед фінальним docs PR):** `51e8f2487d09ad32a2dd8c076127d595228d43a2`. На час перевірки **відкритих PR немає**.
- **Останній опублікований prerelease:** [v0.3.20](https://github.com/RomanZavadaM/md-notes/releases/tag/v0.3.20), 13 assets, у тому числі Windows Portable [ZIP](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.20/MD-Notes-0.3.20-Windows-x64-portable.zip) і SHA256SUMS; Release #112 — успішно (Windows, macOS, Linux, публікація, README links).
- PR #132: перегляд нотатки у відокремленому графі; PR #133: зовнішній Markdown у окремій вкладці при відкритому vault, без автоматичного імпорту. Входять до v0.3.20. **Windows runtime acceptance цих змін ще не підтверджено**.
- PR #134 release-please merged, PR #137 синхронізація документації merged, PR #138 ADR-0010 (багатовіконний режим) merged. **ADR-0010 — тільки проєктне рішення, функція ще не реалізована**.
- ADR-0009 backup/restore — запропоновано, **немає реалізації**. Підготовчої гілки може вистачати без коду: звірити перед роботою; не плутати з інтегрованою функцією.
- Вимога власника на завтра: **кілька вкладок і незалежних вікон редактора нотаток**; обрати маленький безпечний зріз M1 (модель документів/вкладок), з тестами та конфліктним захистом, потім M2 (незалежні вікна). Врахувати graph, зовнішні файли, спільний vault і незбережені зміни в усіх вікнах. Backup/restore не загубити (ADR-0009) і узгодити зі станом вкладок.
- Наступний старт: перевірити фактичні `main`/open PR/releases/actions, `PROJECT_RULES.md`, `PROJECT_STATE.md`, `WORKLOG.md`, ADR-0009 та ADR-0010, Issue #8. Жодних тверджень про runtime PASS до тесту власником. Для релізів обов'язкові PR checks, tag, Portable, SHA256SUMS, документація та 7 мов.


## Активний зріз 28.10.2026 — ADR-0010 M1/M2

- Стартовий `main` звірено з GitHub: `2beffb112f3c12af70a98549b4bb7c82913c84ad`.
- **M1 завершено:** PR #142 merged → `686129279bd7cffca648d7f3b7fe3447f5b08a93`. Реалізовано масив відкритих vault-документів, окремий `draft/savedContent` на вкладку, перемикання без примусового save, безпечне закриття вкладки через save.
- **M2 завершено:** PR #144 merged → `34723be6d85654f4b4561c7318abf79d93893cee`. Реалізовано кнопку «Відкрити в окремому вікні», `EditorWindow`, динамічні Tauri `editor-*` windows, повернення головного редактора до попередньої вкладки/empty state.
- Перед detach поточна чернетка зберігається; main-tab прибирається лише після події успішного створення editor-window. У detached editor навігація не продовжується після помилки save.
- Локалізація нового action: UK/EN/FR/DE/ES/KO/JA.
- Перевірки фінального M2 head `e5a3bd6801e504148f51b1a5e8eba35ead3d7290`: CI #411 SUCCESS; Mobile smoke #180 SUCCESS (Android debug + iOS simulator).
- **Runtime acceptance ще немає.** Не позначати Windows/macOS/Linux UX як PASS лише на підставі CI.
- **M3 завершено:** conflict guard + detached safe close + main save-all/safe Quit інтегровані через PR #146/#147/#148. **Наступна точна дія:** M4 — release checkpoint і ручне Windows-приймання багатовіконності.
- Не загубити ADR-0009 backup/restore, групові операції, Git ↔ SyncManifest, WebDAV, physical Android/iOS, encrypted-vault gaps, signing.


## Активний зріз 28.10.2026 — ADR-0010 M3 завершено

- PR #146 merged → `9089fa146f6ab443d18b09a0eed662cd2336ab9b`: optimistic conflict guard main/detached.
- PR #147 merged → `c2cc7c955a80325306ae98b3a21cef34b2ebf72e`: guarded native close detached editor.
- PR #148 merged → `1b33390b9bb70f18b60bea1c7c9492c5a89a5098`: save-all main tabs/external Markdown, safe main close, graceful app Quit через close detached editors.
- PR #148 verification: CI #417 SUCCESS; Mobile smoke #185 SUCCESS.
- **Не вважати M4/runtime виконаним.** Наступна дія — release-please/checkpoint, тестовий prerelease та Windows Portable для ручної перевірки: вкладки → detach → робота main паралельно → conflict guard → close/quit.


## 28.10.2026 — v0.3.21 publication/finalization

- v0.3.21 prerelease опублікований з Windows Portable/installer/MSI, macOS universal, Linux AppImage/deb/rpm та контрольними файлами.
- M1–M3 технічна основа входить у release, але ручний Windows-тест показав: погоджений multiwindow UX не працює.
- Перевірено UI localization parity: UK/EN/FR/DE/ES/KO/JA, 98 основних ключів у кожній, розбіжностей немає.
- README + user guides синхронізовано з фактом публікації v0.3.21.
- **Наступна дія: не переходити до backup/restore. Спочатку виправити ADR-0010 multiwindow UX після FAIL v0.3.21.** Backup/restore та решту backlog не втрачати.


## 28.10.2026 — ручний runtime FAIL v0.3.21

Факт від власника: при відкритому вікні графа кліки по вузлах відкривають багато вкладок у головному вікні; граф лишився таким, як був. Погоджений сценарій фактично не реалізований.

Не повторювати попередню помилку документації: CI PASS ≠ multiwindow UX PASS.

Точна наступна робота:
1. Відтворити `GRAPH_OPEN_NOTE` → open-note routing у поточному коді.
2. Змінити звичайний graph-node click так, щоб він замінював активну нотатку main, а не додавав нову вкладку.
3. Явна «Відкрити в окремому вікні» повинна реально створювати незалежне editor-вікно; main після цього використовується для іншої нотатки.
4. Перевірити одночасно graph + main + кілька detached editor windows.
5. Лише після реального тесту власником міняти статус на PASS.
