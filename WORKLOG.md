# WORKLOG — MD Notes

Оновлено: **29.10.2026**

## STATUS

**ACTIVE. Останній реліз v0.3.22 (29.10.2026). ADR-0010 r8 прийнято для релізу; ADR-0009 backup/restore реалізовано; release checkpoint завершено.**

Оновлено: 27.10.2026. Нижче розділи за 18-19.10 — історія (запис стану на ті дати); поточний стан — у `PROJECT_STATE.md`.

Release baseline (актуально 29.10.2026): tag/prerelease `v0.3.22`; release target `eed52893ec4bc3b1d55345b54a193b19c33462c2`; workflow `40939361323` SUCCESS; 13 assets опубліковано.

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


## Активний зріз 29.10.2026 — ADR-0010 graph routing r1

- База: `main` `e4e759e50e48042701cb495b0d806746ca4c7d36`; гілка `fix/adr0010-graph-routing-r1`.
- Факт перед змінами: open PR немає; останній prerelease — `v0.3.21`; ручний Windows runtime = **FAIL** для погодженого multiwindow UX.
- Причина в коді: `GRAPH_OPEN_NOTE` у `App.tsx` маршрутизується через загальний `openNote()`, який додає новий `OpenVaultDocument` до масиву вкладок; handler також примусово фокусує main.
- Ціль r1: graph-node navigation має міняти/замінювати активну vault-нотататку main без накопичення нових вкладок; якщо ціль уже відкрита — активувати її; dirty активну нотатку спершу безпечно зберегти через наявний conflict guard; не примушувати focus main.
- `openNote()` для дерева/звичайного відкриття вкладок не змінювати. Явний detach editor лишається єдиним шляхом створення окремого editor-вікна.
- У `GraphWindow` звичайний node click має відправляти `GRAPH_OPEN_NOTE` після локального follow/preview, щоб main реально змінював активну нотатку.
- Критерії готовності: frontend build/CI green; routing не додає вкладку на кожен graph click; graph preview/history не ламаються; runtime PASS **не** заявляти до тесту власником.
- ADR-0009 backup/restore, Git↔SyncManifest, WebDAV, physical mobile tests, encrypted-vault gaps і signing залишаються backlog і в цьому slice не починаються.

- Реалізовано в коді: `App.tsx` отримав окремий `openGraphNote()`; для нового graph-target він safe-save поточний active document і **замінює його на тому самому індексі**, замість append вкладки. Якщо target уже відкритий — лише активується. Примусовий `setFocus()` видалено.
- `GraphWindow.tsx`: звичайний `onOpen` вузла зберігає попередні `followNote + preview` і додатково надсилає `GRAPH_OPEN_NOTE` у main.
- Код-коміти: `2155d73cccb1cb5fbac48b493087eb0d218d2d7a`, `43072e5c1b8db1defa00dcc4196a739f57e6b794`. Локальна runtime-перевірка через реальний Windows app тут не виконувалась; наступне — PR + CI, потім ручний тест власником.


## Checkpoint 29.10.2026 — ADR-0010 graph routing r1 integrated

- PR #153 merged → `9a2db3532b63db2e7c9f52c05902d4f10a87cb24`.
- Final head: `0f4613f90fbbdcd1bc1eaae6ac826d9fa09f6f52`; CI `40841554655` = SUCCESS.
- Runtime FAIL v0.3.21 не перекласифіковано: цей code fix ще не перевірений власником у Windows.
- Наступна **єдина активна дія**: дати власнику тестову збірку/checkpoint і прийняти реальний сценарій: node click змінює active main note без накопичення tabs → explicit detach → main лишається доступним → відкрити кілька independent editor windows → перевірити conflict/safe close.
- До runtime verdict не починати ADR-0009 backup/restore. Не загубити Git↔SyncManifest, WebDAV, physical Android/iOS, encrypted-vault gaps, signing.
- Окремо зафіксовано технічний борг CI: frontend-зміна PR #153 запустила Rust/Tauri matrix всупереч очікуваній path-aware policy; не виправляти в цьому slice, розглянути після ADR-0010 acceptance.

- Release Please автоматично відкрив PR #154 `chore(main): release 0.3.22`; **не зливати і не публікувати 0.3.22 без окремого release checkpoint/команди власника**. Для runtime-приймання r1 спочатку потрібна тестова Windows-збірка й фактичний verdict власника.


## 29.10.2026 — Windows runtime artifact готовий

- Одноразовий PR #156 створив Windows-only test workflow і закритий без merge після успішної збірки.
- Run `40843601078`: SUCCESS. Artifact `12884352283`: `MD-Notes-ADR0010-r1-c6d5d398-Windows-x64-portable`.
- Внутрішній ZIP: `MD-Notes-ADR0010-r1-c6d5d398-Windows-x64-portable.zip`; SHA256 `3d261922637fb29db3eb5a57ed42637095966a265f3e6f8c13a6d4fb4e69de1f`.
- Тестова збірка не змінює version/tag/release; PR #154 `0.3.22` не зливати до runtime verdict.
- Наступна **єдина активна дія**: owner Windows test. PASS/FAIL зафіксувати в Issue #8 + START/STATE/WORKLOG. Якщо FAIL — виправляти ADR-0010; якщо PASS — лише тоді закривати ADR-0010 і вирішувати release checkpoint / перехід до ADR-0009.


## Активний зріз 29.10.2026 — ADR-0010 graph routing r2 після owner runtime

- База: `main` `85d046a038f2bb5e88c8b4cd68c054d80e98cfc1`; гілка `fix/adr0010-graph-routing-r2`.
- Owner runtime r1: **частковий успіх** — graph + main + кілька detached editor windows одночасно працюють. **FAIL лишається**: вибір вершин графа у цьому режимі відкриває/маршрутизує нотатки у невидиме/фонове вікно з вкладками замість видимого робочого редактора.
- ADR-0010 не закривати і runtime PASS не ставити.
- Кодова причина для r2-аудиту: `GRAPH_OPEN_NOTE` лишається безадресною global event; `App.tsx` є єдиним handler і маршрутизує graph selection у модель `openDocuments` main-вікна. Це суперечить ADR-0010 п.6 про адресні події та пояснює попадання в фоновий main/tab state.
- Ціль r2: зробити destination graph navigation явним/адресним і не створювати прихованих вкладок; не ламати вже підтверджені detached editor windows, conflict guard, safe close, graph preview/history.
- Перед кодом повторно звірено backlog: ADR-0009 backup/restore **лише спроєктовано, не реалізовано**; Git↔SyncManifest conflict policy, WebDAV, physical Android/iOS, SAF/security-scoped folders, encrypted-vault gaps, signing, групові операції з файлами/папками лишаються незавершеними. Git push реалізований, але окремо не прийнятий живим runtime-тестом.
- Поки r2 не прийнято власником, ADR-0009 не починати. Після r2 PASS наступний функціональний блок — ручний backup/restore slice за ADR-0009.

- r2 code checkpoint: `graphWindowApi.ts` тепер використовує адресний `emitTo("main", GRAPH_OPEN_NOTE, path)` замість global broadcast. Після маршрутизації main примусово `unminimize()+show()`, коротко піднімається у Z-order і фокус одразу повертається в graph; це має прибрати ефект «нотатка відкрилася у невидимому вікні» без втрати можливості послідовно клікати граф. Detached `editor-*` навмисно не отримують graph navigation і зберігають власну note identity.
- `GraphWindow.tsx` і звичайний node click, і явна кнопка preview «відкрити/редагувати» використовують один `openGraphNoteInMain()`. Наявний main-side `openGraphNote()` replace-active routing, conflict guard і latest-request guard не змінено.
- Автоматична перевірка ще попереду; runtime PASS не заявляти до нового Windows Portable test.


## Checkpoint 29.10.2026 — ADR-0010 r2 integrated + test artifact

- PR #158 merged squash → `main` `66eb9728bf9f0dadaf59d75a84ac3dde8ae30d17`.
- CI `40853765572` — SUCCESS усіх запущених jobs.
- Temporary PR #159 — CLOSED WITHOUT MERGE; Windows runtime workflow `40854135789` — SUCCESS.
- Artifact ID `12889336347`; Portable `MD-Notes-ADR0010-r2-f7b35c9b-Windows-x64-portable.zip`; verified SHA256 `3d8cf4ad903e99c25cf9002ba7d4accd45645917a0a1b9e4536eb7ba1fd6280a`.
- Release PR #154 (`0.3.22`) auto-refreshed after merge; не зливати до owner runtime verdict.
- **Статус:** code VERIFIED + integrated; runtime acceptance PENDING, ADR-0010 NOT DONE.

### Єдина наступна активна дія

Owner тестує r2 Windows Portable: graph + main + кілька detached editor windows; серія graph-node clicks має оновлювати main без накопичення вкладок, main не має лишатися невидимим за editor-вікнами, а focus після маршрутизації повертається graph. Зафіксувати PASS/FAIL в Issue #8. Тільки після фактичного PASS переходити до ADR-0009 backup/restore поточного робочого vault.


## 29.10.2026 — уточнення нумерації тестових збірок

- Продуктова версія у `main` лишається **0.3.21**. Release Please PR #154 готує **0.3.22** через інтегровані fix #153/#158; вручну версію не змінювати.
- Позначення `ADR0010-r1/r2-<SHA>` у Windows Portable — це **runtime-test revision**, а не нова продуктова версія.
- Щоб надалі це не виглядало як альтернативна схема SemVer, у PROJECT_RULES зафіксовано: офіційні release assets = `MD-Notes-X.Y.Z-...`; одноразові тести = `MD-Notes-TEST-<scope>-rN-<shortSHA>-...`.
- Вже видані r1/r2 не перейменовувати заднім числом: їхні байти/SHA та записи Issue #8 незмінні. Наступний тестовий кандидат має отримати наступну унікальну ревізію.


## Активний зріз 29.10.2026 — ADR-0010 r3 side-by-side graph/main

- Owner runtime/UX correction: r2 лікує Z-order, але базова проблема інша — detached graph за замовчуванням відкривається великим центральним вікном поверх main, тому graph-node navigation змінює main, який фізично не видно.
- Нова ціль r3: дія «відкрити/розгорнути граф окремо» має не накривати main, а автоматично розкладати `main + graph` поруч на робочій області поточного монітора. Main лишається видимим редактором; graph — видимою навігацією.
- Звичайний graph click продовжує replace-active routing у main без створення нового detached editor. Detached `editor-*` не змінюються.
- Не боротися з ручним maximize користувача: side-by-side є стартовим layout при detach/open graph; далі нативні вікна лишаються вільно переміщуваними/масштабованими.
- При r3 прибрати focus/Z-order ping-pong r2 як нормальний navigation mechanism; він більше не потрібен, коли main видимий поруч.
- Runtime PASS тільки після нового Windows TEST r3; r1/r2 не перевикористовувати.

- r3 code checkpoint: `graphWindowApi.ts` більше не створює graph як 1280×860 centered overlay. При достатній робочій області поточного монітора main і graph автоматично tile side-by-side (~58/42) з невеликим gap; використовуються monitor workArea + DPI conversion.
- Якщо graph уже існує і користувач знову викликає «відкрити граф окремо», layout повторно розкладає обидва native windows поруч. Якщо екран замалий для мінімальних ширин, лишається безпечний centered fallback graph.
- r2 focus/Z-order ping-pong прибрано з graph navigation: `openGraphNoteInMain()` тепер лише адресно надсилає note у main і відновлює його, якщо він був мінімізований. Ручний move/resize/maximize після detach поважається.
- Capability доповнено тільки необхідними window permissions: unmaximize, set-position, set-size.
- Далі: PR + CI; runtime verdict тільки після Windows TEST r3.

- Owner extension: від’єднання вкладок підсилює side-by-side модель. Реалізовано explicit per-tab detach: кожна vault-вкладка має власний `↗`; можна винести саме її в окреме editor-вікно без попередньої активації.
- Спільна `detachDocument(path)` зберігає/перевіряє конкретний документ, відкриває його через existing `openEditorWindow`, прибирає лише цю вкладку з main; якщо від’єднувалась неактивна вкладка, активна main-вкладка не змінюється.
- Верхня кнопка «Відкрити в окремому вікні» тепер використовує ту саму `detachDocument()` для активної вкладки. Drag-and-drop detach не додавався в r3; explicit `↗` є простішим і перевірюваним UX.

- Edge-case correction before issuing r3 test: per-tab detach no longer forces `activeDocument = vault`. If external Markdown is currently visible and the detached vault tab was its background vault fallback, only the vault fallback state changes; the visible external document stays visible.


## Checkpoint 29.10.2026 — ADR-0010 r3 integrated + Windows TEST

- PR #162 merged → `b0f7813c6820cfd5eefb92f0c414d723aab93147`.
- CI `40859261723` SUCCESS; Mobile smoke `40859261721` SUCCESS.
- r3 scope: side-by-side graph/main initial layout; addressed graph→main navigation without focus ping-pong; per-tab `↗` detach; background-tab/external-visible edge case fixed.
- Temporary PR #163 CLOSED WITHOUT MERGE; Windows TEST workflow `40860534312` SUCCESS.
- Artifact `MD-Notes-TEST-ADR0010-r3-b0f7813c-Windows-x64-portable`; inner ZIP same basename + `.zip`; SHA256 `26fa666bc316d360b9bb2f8de5f950c2eeb548b4b523bb9b4511e67543766fe4`.
- Status: code integrated + automated checks PASS; owner runtime verdict PENDING.
- Next single action: owner tests r3 on Windows. Do not start ADR-0009 implementation until this verdict is recorded.


## 29.10.2026 — ADR-0010 r3 owner FAIL → r4

- Owner Windows runtime verdict for r3: **FAIL**. Opening detached graph did not produce visible side-by-side `main + graph`.
- r3 remains immutable issued test revision; do not reuse or relabel it.
- Root-cause audit: r3 relied on constructor-time graph `x/y/size`, and its logical-width gate (`680 + 520`) was too strict for common Windows DPI scaling, silently selecting centered fallback.
- r4 target: reduce adaptive logical minimums and, after creating graph, wait for native `tauri://created`; only then explicitly unmaximize/resize/reposition both native windows via `Window.getByLabel`.
- Runtime PASS remains owner-only. Next issued Windows candidate must be r4.

- r4 code checkpoint: side-by-side gate lowered from `680/520` to `520/360` logical px; min logical height `500`.
- New graph creation no longer tries to trust constructor `x/y/size`. It creates centered safely, subscribes to `tauri://created`, recomputes current monitor work area, then explicitly calls native `Window.setPosition/setSize` for both `main` and `graph`.
- Existing graph reopen still re-applies tiling immediately.
- Next: PR + CI; then issue unique Windows TEST r4 only after green checks.


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

- r5 implementation checkpoint:
  - graph→main routing deliberately unchanged after owner clarification; active note switches correctly, visibility is the defect;
  - added Rust/Tauri `tile_graph_workspace` command using physical monitor work-area pixels, not JS logical coordinates;
  - native split keeps graph compact (~36%, clamped 360–680 physical px) and main in the remaining left area;
  - layout is attempted after native graph creation, again after `GraphWindow` React mount, and manually via visible `⇆` action;
  - graph window shows native layout diagnostics (`main + graph / monitor`) or an explicit error string;
  - all 7 graph locales include the re-tile action label.
- Next: PR + full CI/mobile smoke; only after green checks issue unique Windows TEST r5.


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

- r6 implementation checkpoint:
  - `KnowledgeGraphPanel` now accepts optional `onExpand`;
  - desktop `LinksPanel` passes `openGraphWindow(mode, note.path, graphStrings.windowTitle)` as `onExpand`;
  - large `Розгорнути` button uses native detached graph path on desktop instead of toggling the in-main overlay;
  - local `expanded` overlay remains only when `onExpand` is absent (mobile/non-desktop fallback);
  - existing small `↗` button remains available and uses the same native path;
  - graph→main routing and r5 native tiling logic are unchanged.
- Next: full CI + mobile smoke, merge only if green, then issue unique Windows TEST r6.


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

- r7 implementation checkpoint:
  - detaching graph now turns the main right-side links/graph panel off, freeing the main note workspace;
  - detached graph is navigation-only: no embedded note preview, no second reading surface;
  - graph node/back navigation continues sending the selected note to main;
  - detached graph width changed from ~36% capped at 680px to ~38% capped at 1400px, with main minimum 720px;
  - initial detached graph width increased to 900px before final placement;
  - technical pixel diagnostics removed from normal graph UI; `Розмістити поруч` remains available.
- Next: full CI/mobile checks, then Windows TEST r7 only if green.

### Уточнення власника до r7

- `main` — центральне основне вікно; навколо нього будується весь інтерфейс.
- У `main` можуть залишатися вкладки, права панель, граф та інша інформація — це нормально.
- Коли граф від’єднують, він має поводитися **як від’єднана вкладка**: окреме самостійне вікно, яке можна переміщати/масштабувати незалежно.
- Від’єднання графа **не повинно автоматично змінювати розмір/позицію main і не повинно ховати його панелі**.
- Отже попередня частина r7 про автоматичне приховування правої панелі та примусове розкладання main+graph скасовується.
- Залишається: окреме graph-вікно, незалежне від main; кліки по графу можуть змінювати інформацію/вкладку в main; окремі editor-вікна працюють аналогічно.

### r7 — остаточна модель після уточнення власника

- `main` лишається центральним вікном і не змінює геометрію/панелі при від’єднанні графа.
- Граф від’єднується як незалежне native-вікно, аналогічно від’єднаній вкладці нотатки.
- Кнопка desktop-дії тепер підписана `Граф в окремому вікні`, а не `Розгорнути`.
- Окремий graph не має власного preview нотатки; клік по вузлу спрямовує нотатку в main.
- Автоматичне `main + graph поруч`, зміна розмірів/позицій main і кнопка `Розмістити поруч` повністю прибрані.
- Зайві дозволи на програмне переміщення/resize вікон прибрані.
- Main може й надалі містити вкладки, праву панель і внутрішній граф — від’єднання не змінює його стан.
- Попередній r7 checkpoint про приховування панелі/примусову розкладку вважати скасованим цим уточненням.


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


## 29.10.2026 — ADR-0010 r7 owner runtime: independent graph PASS, graph→main tabs FAIL → r8

- Owner confirms the core window model is now right: graph is an independent detached window; this part is accepted as the correct direction.
- Runtime defect: graph node selection only opens a note when main has no active vault tab. With an existing tab, graph navigation no longer creates/selects the expected tab consistently.
- Owner expectation is explicit: graph node selection must behave like ordinary note opening in main.
  - if target note already has a tab → activate it;
  - if target note has no tab → create a new tab;
  - preserve other existing tabs;
  - main remains central and unchanged; graph remains independent.
- Root cause in current `openGraphNote`: it replaces the active tab instead of reusing normal `openNote` semantics.
- r8 target is narrow: make graph→main navigation delegate to ordinary tab opening; do not change graph window behavior.

- r8 implementation checkpoint:
  - removed the special graph-only tab replacement path;
  - graph selection now delegates to the existing ordinary `openNote` behavior;
  - existing target tab is activated; missing target tab is appended; all other tabs are preserved;
  - removed obsolete graph request/replacement state;
  - detached graph window behavior from r7 is unchanged.
- Next: full CI/mobile checks, merge only if green, then issue unique Windows TEST r8.


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


## 29.10.2026 — початок ADR-0009 r1

- ADR-0010 r8 owner-accepted for release.
- Реалізувати ручний переносимий backup/restore перед фінальним prerelease.
- Перед backup UI зберігає всі відкриті документи.
- Формат: ZIP з manifest.json, SHA-256 для кожного файла; файлове копіювання без `readNote`, щоб encrypted vault лишався ciphertext.
- Виключення: `.git`, `.mdnotes/cache`, тимчасові/OS-specific файли. Кошик включати як відновлювані дані.
- Restore: validate first, extract to temp, atomic rename into new vault, never overwrite.


## 29.10.2026 — функціональний checkpoint перед v0.3.22

- ADR-0010 multi-window: **прийнято власником** на r8. Main — центральне вікно; graph від’єднується незалежно; graph-вузли відкривають/активують звичайні вкладки main.
- ADR-0009 backup/restore: **реалізовано та інтегровано** PR #182 → `71ba9604c30b8399137d79011f2bd97432b9505a`.
- Backup/restore: portable ZIP + manifest/SHA-256; encrypted vault без plaintext/secret key; restore тільки в нове сховище після повної валідації.
- PR #182 CI: Linux/macOS/Windows SUCCESS; Android/iOS smoke SUCCESS; frontend/Tauri/license checks SUCCESS.
- README та user guides синхронізовано всіма 7 мовами.
- Важлива межа доказу: owner вручну перевірив multi-window r8, але **не проводив окремого ручного backup/restore runtime-тесту**; backup покривається автоматичними тестами трьох desktop OS.
- Поточний product-code main: `71ba9604c30b8399137d79011f2bd97432b9505a`.
- Наступна єдина дія: завершити Release Please PR #154 → v0.3.22 prerelease, перевірити release workflow та 13 assets, після цього синхронізувати цей checkpoint із фактичним release SHA/tag.


## 29.10.2026 — v0.3.22 release завершено

- Перед фінальною синхронізацією GitHub: release target `eed52893ec4bc3b1d55345b54a193b19c33462c2`; після docs-only README links update актуальний `main` = `a631801e97fd335591bc6da7d4e670d3d21bf2da`.
- Release Please #154 merged; `v0.3.22` prerelease опублікований.
- Release workflow `40939361323` = **SUCCESS**.
- Опубліковано 13 assets, включно з Windows portable ZIP, EXE/MSI, macOS universal, Linux packages, START/source, checksums і legal notices.
- ADR-0010 r8: practical owner acceptance; не продовжувати полірування без нового фактичного дефекту.
- ADR-0009 backup/restore: merged у PR #182; автоматичні desktop тести пройдені; окремого owner runtime backup/restore test не було.
- Продуктовий код після release не змінюється цим docs-only checkpoint.

### Наступна точна дія

Продовжувати з backlog: групові операції в дереві файлів, Git ↔ SyncManifest, WebDAV, physical-device mobile validation, encrypted-vault gaps, signing.
