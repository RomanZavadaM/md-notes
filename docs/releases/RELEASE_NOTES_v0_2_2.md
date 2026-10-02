# MD Notes v0.2.2 — release notes

Дата checkpoint: **02.10.2026**  
Статус: **test / prerelease**  
Tag: `v0.2.2`  
Release commit: `b422b497b4e3980c665f7ae7017f34e73788218d`

## Основне

v0.2.2 закриває функціональний scope етапу v0.2 і публікує накопичені зміни після v0.2.1:

- runtime-startup hotfix без silent auto-reopen останнього vault;
- Windows portable package як стандартний release artifact;
- типи нотаток, відкритий `.mdnotes/schema.json` v1 і schema-driven форма властивостей;
- safe vault presets: Empty / PARA / Zettelkasten + explicit startup gate;
- global/local knowledge graph на Sigma.js + Graphology;
- політика pre-1.0 versioning виправлена: звичайні feature-checkpoint-и всередині roadmap stage піднімають patch, а перехід `0.2 → 0.3` лишається окремим продуктовим рішенням.

## Перевірка

- main CI для release commit: run `36990461817` — **PASS**;
- release workflow #30 / run `36990461869` — **SUCCESS**;
- Windows / macOS universal / Linux build jobs — **SUCCESS**;
- START/source, legal notices і `SHA256SUMS.txt` — **SUCCESS**;
- dependency license gates — PASS.

## Опубліковані пакети

### Windows x64

- `MD.Notes_0.2.2_x64-setup.exe`
- `MD.Notes_0.2.2_x64_en-US.msi`
- `MD-Notes-0.2.2-Windows-x64-portable.zip`

### macOS universal

- `MD.Notes_0.2.2_universal.dmg`
- `MD.Notes_universal.app.tar.gz`

### Linux x86_64

- `MD.Notes_0.2.2_amd64.AppImage`
- `MD.Notes_0.2.2_amd64.deb`
- `MD.Notes-0.2.2-1.x86_64.rpm`

### Source / legal / verification

- `MD-Notes-0.2.2-START.zip`
- `LICENSE.md`
- `COPYRIGHT.md`
- `THIRD_PARTY_NOTICES.md`
- `SHA256SUMS.txt`

## Межа перевірки

- користувач раніше підтвердив сильне зависання v0.2.1 на Windows після запуску;
- v0.2.2 містить startup hotfix і explicit startup gate, але **ручний runtime test v0.2.2 ще не підтверджений**;
- green CI та platform builds не подаються як заміна реального GUI/runtime validation;
- якщо зависання повторюється після ручного відкриття vault, наступний технічний напрям — оптимізація `note_paths()` / `get_tree()` і винесення дорогого index sync із критичного open path;
- збірки не підписані, тому Windows SmartScreen / macOS Gatekeeper можуть показувати попередження.

## Наступний етап

Функціональний roadmap v0.2 завершений. Перший операційний крок після checkpoint — реальна Windows-перевірка v0.2.2. Після її результату можна або закрити performance blocker, якщо він ще відтворюється, або переходити до roadmap v0.3: Android/iOS + Git/WebDAV sync.
