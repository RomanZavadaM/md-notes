# WORKLOG — MD Notes

Оновлено: **02.10.2026**

## STATUS

**ACTIVE. Checkpoint v0.2.2 — DONE і опублікований. Функціональний scope v0.2 завершений.**

Release:
- tag/prerelease `v0.2.2`;
- release commit `b422b497b4e3980c665f7ae7017f34e73788218d`;
- main CI `36990461817` — **PASS**;
- release workflow #30 / `36990461869` — **SUCCESS**;
- Windows/macOS/Linux packages + START/source + legal + `SHA256SUMS.txt` опубліковані;
- Windows portable тепер стандартний release artifact.

## Що увійшло після v0.2.1

- runtime startup hotfix — PR #21;
- Windows portable packaging — PR #20;
- schema/property form — PR #19;
- vault presets/startup gate — PR #23;
- global/local knowledge graph — PR #25;
- pre-1.0 release policy fix — PR #27: feature-checkpoint-и всередині roadmap stage піднімають patch, а перехід `0.2 → 0.3` не відбувається автоматично.

## Опубліковані пакети v0.2.2

Windows:
- `MD.Notes_0.2.2_x64-setup.exe`
- `MD.Notes_0.2.2_x64_en-US.msi`
- `MD-Notes-0.2.2-Windows-x64-portable.zip`

macOS:
- `MD.Notes_0.2.2_universal.dmg`
- `MD.Notes_universal.app.tar.gz`

Linux:
- `MD.Notes_0.2.2_amd64.AppImage`
- `MD.Notes_0.2.2_amd64.deb`
- `MD.Notes-0.2.2-1.x86_64.rpm`

Додатково:
- `MD-Notes-0.2.2-START.zip`
- `LICENSE.md`
- `COPYRIGHT.md`
- `THIRD_PARTY_NOTICES.md`
- `SHA256SUMS.txt`

## Runtime evidence gate

Реальний тест v0.2.1 на Windows раніше виявив blocker: після запуску програма могла сильно зависати.

У v0.2.2 вже є:
- блокування silent auto-reopen останнього vault;
- explicit startup gate «відкрити останнє / відкрити існуюче / створити нове»;
- safe Empty / PARA / Zettelkasten presets;
- schema/property form;
- local/global graph.

**Але v0.2.2 ще не підтверджено реальним ручним Windows runtime-тестом. Green CI/build не вважається runtime validation.**

### Що перевірити на Windows

1. запуск v0.2.2 — UI має відкритися без зависання комп'ютера;
2. «Про програму» до відкриття vault;
3. ручне відкриття невеликого існуючого vault;
4. створення Empty/PARA/Zettelkasten у порожній папці;
5. schema/property form на тестовій нотатці;
6. local/global graph та click-node navigation;
7. повторний запуск і явне «Відкрити останнє сховище»;
8. якщо зависання повториться саме після відкриття vault — зафіксувати розмір/тип папки та перейти до оптимізації `note_paths()` / `get_tree()` / synchronous index sync.

## Наступна дія

1. користувач тестує Windows v0.2.2, бажано portable як найпростіший варіант;
2. якщо freeze відтворюється — performance blocker має пріоритет над v0.3;
3. якщо базовий runtime стабільний — можна починати roadmap v0.3: Android/iOS + Git/WebDAV sync;
4. не заявляти, що Windows freeze остаточно виправлений, доки немає реального результату користувача.

## Відомі обмеження

- Windows startup-freeze hotfix ще не підтверджено повторним реальним тестом v0.2.2;
- великі vault-и індексуються синхронно при ручному відкритті;
- startup/preset UI, schema/property form і graph UI не пройшли manual runtime validation у поточному середовищі;
- graph slice не включає ForceAtlas2/clustering або persistence layout;
- збірки не підписані;
- Android/iOS ще не збираються.
