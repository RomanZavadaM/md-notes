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
