# WORKLOG — MD Notes

Оновлено: **03.10.2026**

## STATUS

**ACTIVE. Checkpoint v0.2.2 опублікований і підтверджений ручним Windows runtime-тестом. Roadmap stage v0.3 у роботі.**

Release baseline:
- tag/prerelease `v0.2.2`;
- release commit `b422b497b4e3980c665f7ae7017f34e73788218d`;
- Windows/macOS/Linux packages + START/source + legal + `SHA256SUMS.txt` опубліковані;
- застарілий release-please PR #29 (`0.2.3`) закрито: перехід у v0.3 є погодженим minor roadmap stage, а не patch release.

## Runtime evidence

03.10.2026 власник повторно перевірив v0.2.2 на реальній Windows і підтвердив: **працює нормально**.

Це закриває blocker startup-freeze, який спостерігався у v0.2.1.

## Завершено у v0.3

### StorageProvider foundation — PR #30

Merged у `main`: `4545c938fcb70c80caa9e7595b4d89e8eead27aa`.

- provider-neutral `StorageProvider` API;
- `LocalFsProvider` для desktop vault і mobile sandbox;
- `list`, `read`, `write`, `remove`, `metadata`, `changes_since`;
- atomic local write;
- path escape rejection;
- unit-тести;
- clean CI #147.

### Provider-backed Vault I/O — PR #31

Merged у `main`: `230ee8e4373fe909c178c44865c5b5cd9d7bcd3e`.

- `VaultStorage` поверх `StorageProvider`;
- provider-backed tree traversal, note paths, config, read/write notes і unique paths;
- існуючий `Vault` делегує provider-neutral I/O через `VaultStorage`;
- локальні cache/rename/trash поки лишаються desktop/local operations;
- clean CI #153.

## Активний v0.3 slice — mobile sandbox

Branch: `feature/mobile-sandbox-v0.3`.
Draft PR: #32.

Зроблено:
- runtime platform detection (`android`, `ios`, desktop OS);
- app-data `vault` як default mobile sandbox відповідно до ADR-0006;
- safe open-or-create sandbox vault;
- Tauri command `open_mobile_sandbox_vault`;
- typed TypeScript API;
- startup gate на Android/iOS використовує локальний sandbox замість desktop folder picker;
- mobile startup strings локалізовано UK/EN/FR/DE/ES/KO/JA.

CI для повного slice має пройти перед merge.

## Наступна дія

1. отримати clean CI для PR #32 і злити його у `main`;
2. підготувати mobile build validation (Android/iOS bootstrap/build checks, без удавання runtime-тесту на реальному пристрої);
3. перейти до sync foundation: local-first sync state + conflict model за ADR-0005;
4. реалізувати Git sync (`gix`/gitoxide), далі WebDAV;
5. після стабільного sandbox+sync повернутися до optional external-folder adapters: Android SAF і iOS security-scoped bookmarks.

## Відомі обмеження

- Android/iOS runtime на реальних пристроях ще не тестувався;
- optional SAF/security-scoped bookmark adapters ще не реалізовані;
- sync ще не реалізований;
- локальні cache/rename/trash ще не узагальнені під зовнішні mobile providers;
- великі vault-и все ще індексуються синхронно при ручному відкритті;
- graph slice не включає ForceAtlas2/clustering або persistence layout;
- збірки не підписані.
