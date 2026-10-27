# WORKLOG — MD Notes

Оновлено: **18.10.2026**

## STATUS

**ACTIVE. Останній реліз v0.3.18 (21.10.2026). Зашифроване сховище (зрізи 1-3), git push через системний `git`, панель Git, імпорт з об’єднанням, граф і дерево зв’язків — у main. Відкритий PR #122.**

Оновлено: 27.10.2026. Нижче розділи за 18-19.10 — історія (запис стану на ті дати); поточний стан — у `PROJECT_STATE.md`.

Release baseline (на момент запису): tag/prerelease `v0.3.18`; активи Windows/macOS/Linux, START/source, legal, `SHA256SUMS.txt` опубліковані.

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

Злити PR #122, за потреби випустити наступну версію через release-please; далі за `PROJECT_STATE.md` — Git state ↔ `SyncManifest`. Нижче — історичний запис від 19.10 (зрізи 0-3 і push уже виконано).

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
