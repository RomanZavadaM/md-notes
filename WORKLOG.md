# WORKLOG — MD Notes

Оновлено: **03.10.2026**

## STATUS

**ACTIVE. v0.2.2 опублікований і підтверджений ручним Windows runtime-тестом. Roadmap stage v0.3 у роботі.**

Release baseline:
- tag/prerelease `v0.2.2`;
- release commit `b422b497b4e3980c665f7ae7017f34e73788218d`;
- Windows/macOS/Linux packages + START/source + legal + `SHA256SUMS.txt` опубліковані;
- застарілий release-please PR #29 (`0.2.3`) закрито без merge: власник погодив перехід у roadmap stage v0.3.

## Runtime evidence

03.10.2026 власник повторно перевірив v0.2.2 на реальній Windows і підтвердив: **працює нормально**.

Це закриває blocker startup-freeze, який спостерігався у v0.2.1.

## Завершено й інтегровано у v0.3

### PR #30 — StorageProvider foundation
- merge `4545c938fcb70c80caa9e7595b4d89e8eead27aa`;
- CI #147 PASS;
- `StorageProvider`, `LocalFsProvider`, atomic writes, path safety, provider operations/tests.

### PR #31 — provider-backed Vault I/O
- merge `230ee8e4373fe909c178c44865c5b5cd9d7bcd3e`;
- CI #153 PASS;
- `VaultStorage`, provider-backed tree/config/read/write/unique paths;
- existing `Vault` delegates provider-neutral I/O.

### PR #32 — mobile sandbox bootstrap
- merge `588219e2310e94f2fbd04a3ba97288b8ba94813f`;
- CI #161 PASS;
- runtime platform detection;
- app-data sandbox vault for Android/iOS;
- mobile startup path without desktop folder picker;
- UK/EN/FR/DE/ES/KO/JA localization.

### PR #33 — local-first sync decision foundation
- merge `e91734df8f37d2be1a776fcad7883bbd30160b01`;
- CI #162 PASS;
- sync manifest/snapshots;
- upload/download/delete planner;
- modification wins over concurrent deletion;
- divergent change => merge/conflict;
- conflict-copy path;
- unit tests.

## Активний slice — mobile build validation

Draft PR #35: `ci: validate Android and iOS mobile builds`.

Checks:
- Android: current Tauri CLI, Android Rust target, `android init --ci`, aarch64 debug build;
- iOS: current Tauri CLI, iOS simulator Rust target, `ios init --ci`, arm64-simulator debug build;
- результат вважати build evidence, **не** runtime evidence.

## Черга після PR #35

1. Зафіксувати Android/iOS build outcome і, якщо потрібно, виправити mobile compile blockers.
2. Git sync slice:
   - підключити актуальний `gix`/gitoxide;
   - HTTPS remote + auth boundary без зберігання секрету у vault;
   - clone/fetch/pull/commit/push foundation;
   - інтеграція з `SyncManifest` / ADR-0005 conflict policy;
   - dependency licenses + `THIRD_PARTY_NOTICES.md`.
3. WebDAV slice через актуальний OpenDAL `services-webdav`.
4. Local-first queue / persistence sync state у `.mdnotes/cache/`.
5. System secret store для токенів/паролів.
6. Після стабільного sandbox+sync — optional external-folder access:
   - Android SAF + persistable URI permission;
   - iOS security-scoped bookmarks.

## Відомі обмеження

- Android/iOS runtime на реальних пристроях ще не тестувався;
- mobile CI поки не дорівнює device runtime;
- Git/WebDAV adapters ще не реалізовані;
- external SAF/bookmark adapters ще не реалізовані;
- local cache/rename/trash ще не узагальнені під external mobile providers;
- великі vault-и все ще можуть індексуватися синхронно при відкритті;
- graph slice не включає ForceAtlas2/clustering або persistence layout;
- збірки не підписані.
