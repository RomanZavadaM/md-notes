# WORKLOG — MD Notes

Оновлено: **03.10.2026**

## STATUS

**ACTIVE. Checkpoint v0.2.2 опублікований і підтверджений ручним Windows runtime-тестом. Розпочато roadmap stage v0.3.**

Release baseline:
- tag/prerelease `v0.2.2`;
- release commit `b422b497b4e3980c665f7ae7017f34e73788218d`;
- docs checkpoint in `main`: `6393cf5125f924d0b00c2d68bbde90d7e5bc88b1`;
- Windows/macOS/Linux packages + START/source + legal + `SHA256SUMS.txt` опубліковані.

## Runtime evidence

03.10.2026 власник повторно перевірив v0.2.2 на реальній Windows і підтвердив: **працює нормально**.

Це закриває blocker startup-freeze, який спостерігався у v0.2.1. Green CI/build як і раніше не замінює runtime validation, але для v0.2.2 ручне підтвердження тепер є.

## Активний v0.3 slice

Branch: `feature/storage-provider-v0.3`.

Мета: виконати передумову ADR-0003/ADR-0006 для мобільного доступу до файлів і майбутніх Git/WebDAV provider-ів: ядро не повинно залежати від конкретного `std::fs` шляху.

Вже зроблено:
- додано provider-neutral `StorageProvider` API;
- додано `LocalFsProvider` для desktop vault і mobile sandbox;
- API включає `list`, `read`, `write`, `remove`, `metadata`, `changes_since`;
- локальний write атомарний через temp + rename;
- відносні шляхи проходять існуючу перевірку виходу за межі vault;
- додано unit-тести round-trip, remove, recent changes та path escape rejection;
- API експортовано з `notes-core`.

## Наступна дія

1. відкрити draft PR і отримати clean CI для storage-provider foundation;
2. після зеленого CI перевести `Vault` file operations на `StorageProvider`, не змінюючи формат vault та поведінку desktop;
3. далі додати mobile adapters: Android SAF (`content://` + persistable URI permissions) і iOS security-scoped bookmarks;
4. після mobile file-access foundation перейти до Git sync, потім WebDAV, local-first queue/conflicts і system keyring за roadmap v0.3.

## Відомі обмеження

- `Vault` у поточному baseline ще напряму використовує `std::fs`; provider API поки є foundation, а не повна міграція;
- Android/iOS нативні adapters ще не реалізовані;
- великі vault-и все ще індексуються синхронно при ручному відкритті;
- graph slice не включає ForceAtlas2/clustering або persistence layout;
- збірки не підписані.
