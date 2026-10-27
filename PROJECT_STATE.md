# PROJECT_STATE — MD Notes

Оновлено: **27.10.2026**

## Поточний стан

- Статус розвитку: **ACTIVE**. Оновлено: **27.10.2026**.
- Останній опублікований реліз: **v0.3.20 prerelease**, 27.10.2026 (версія — з GitHub Releases; машинні джерела: `Cargo.toml`, `app/package.json`, `app/src-tauri/tauri.conf.json`).
- PR #122 (кнопка «Назад» у вікні графа) інтегровано 27.10.2026. PR #128 інтегровано (`2da9e7ffb01355008de782df6326d98587923dff`) — лише запис вимог до групових операцій/вкладок; реалізація не починалась.
- README: UK / EN / FR / DE / ES / KO / JA; керівництво користувача — ті самі 7 мов (останнє оновлення керівництва: v0.3.15).
- UI: ті самі 7 мов; усі ключі присутні в усіх мовах (перевірено скриптом 27.10.2026: основний словник 97 ключів, допоміжні словники git/graph/encryption/importer/lookup/relationTree/relations/startup — без розбіжностей). FR/DE/ES/KO/JA — машинні переклади, не вичитані носіями.
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

1. Git state ↔ `SyncManifest` / ADR-0005.
2. WebDAV.
3. Physical-device Android/iOS runtime validation.
4. Optional Android SAF / iOS security-scoped external folders.
5. Прогалини зашифрованого сховища (вкладення, ключ відновлення).

Перед новими dependency обов’язково проходити license gates та оновлювати `THIRD_PARTY_NOTICES.md`.
