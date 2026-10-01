# WORKLOG — MD Notes

Оновлено: **01.10.2026**

## STATUS

**ACTIVE.** Checkpoint **v0.2.0** опубліковано — DONE.

## Останній checkpoint

- GitHub prerelease [`v0.2.0`](https://github.com/RomanZavadaM/md-notes/releases/tag/v0.2.0), 01.10.2026;
- release PR #11, release commit `d3302b0527839ac2b5cbf0f09d14e6ed58500f51`;
- release run #36913195476 — SUCCESS (Windows, macOS universal, Linux, assets);
- assets: `.exe`, `.msi`, `.dmg`, `.app.tar.gz`, `.AppImage`, `.deb`, `.rpm`, `MD-Notes-0.2.0-START.zip`, `LICENSE.md`, `COPYRIGHT.md`, `THIRD_PARTY_NOTICES.md`, `SHA256SUMS.txt` (контрольні суми перевірено).

## Наступна дія

Після публікації v0.2.0 — визначити наступний slice з roadmap (рекомендовано: строга CSP, потім вкладення).

## Нещодавно завершено

- v0.2.0 scope: PR #2 (індекс), #3 (пошук, теги, панель зв'язків, стеження), #4 (оновлення посилань), #5 (Mermaid/KaTeX/зображення), #6 (перейменування в застосунку), #7 (шаблони, щоденні нотатки), #9 (правила, ліцензія, документація 7 мовами, license gate), #10 (інтерфейс 7 мовами, «Про програму») — DONE.
- v0.1.0: каркас, CI, release-please, реліз (PR #1, `447e8bf`).
- Аудит ліцензій: 501 Rust-крейт і 400 npm-пакетів; Mermaid 12 → 11.17 через EPL-2.0 `elkjs`.

## Відомі обмеження

- runtime-перевірки у вікні застосунку не виконувалися (немає локального Rust/Node на робочій машині розробки);
- збірки не підписані.
