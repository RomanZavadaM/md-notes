# MD Notes v1.0.1 — стабілізаційний реліз

**Дата:** 30.10.2026  
**Лінія:** stable 1.x  
**Тип:** patch / stabilization

## Що це за реліз

v1.0.1 — перший стабілізаційний patch після повного повторного аудиту stable v1.0.0. Нових великих функцій він не додає: реліз виправляє знайдені під час аудиту проблеми у залежностях, Tauri permissions/asset loading, release automation, версіюванні та документації.

## Основні виправлення

- оновлено security-sensitive Rust dependency graph; ZIP backup/restore переведено на підтримувану гілку `zip 9.x`;
- замінено застарілий YAML backend на підтримуваний;
- Rust builds і release packaging перевіряють `Cargo.lock` через `--locked`;
- CI перевіряє RustSec advisories разом із dependency licenses; додано щотижневий dependency security audit;
- виправлено право Tauri для зовнішнього відкриття незашифрованих вкладень, обмежене лише головним вікном;
- preview зображень/PDF більше не залежить від широкого filesystem asset protocol: усі vault assets читаються через активний Rust storage provider і показуються через Blob URL;
- asset protocol вимкнено як непотрібний;
- синхронізовано `app/package-lock.json` з версією продукту;
- release-please тепер керує `version.txt` як похідним compatibility marker і синхронно оновлює Cargo, package.json, package-lock та Tauri version;
- release build відмовляється працювати при розсинхронізації lock/version metadata;
- виправлено cross-platform release/CI checks для Windows PowerShell;
- уточнено license/dependency policy, legal wording для optional age-encrypted vault і third-party notices;
- прибрано застарілі v0.3.x формулювання з актуальних stable-документів;
- виправлено локалізаційний витік українських описів мов у неукраїнському UI;
- актуалізовано multi-window, graph, encrypted-vault, backup/recovery та AI-contract документацію.

## Перевірено перед релізом

- frontend build, TypeScript і parity усіх 7 мов;
- production npm audit;
- Rust dependency licenses + advisories;
- `notes-core` default та `encrypted-vault` на Linux, Windows і macOS;
- Tauri clippy на Linux і macOS;
- Windows NSIS + MSI offline installer smoke;
- Android debug build;
- iOS simulator debug build;
- Linux AppImage / DEB / RPM package build;
- Universal macOS DMG package build;
- 432 локальні Markdown links у фінальному documentation audit;
- release-please dry-run для переходу 1.0.0 → 1.0.1 та синхронізації version metadata.

## Відомі межі

- Windows/macOS пакети не вважаються підписаними/notarized, якщо в GitHub Release прямо не зазначено інше;
- Android/iOS проходять build smoke, але physical-device/store acceptance залишається post-1.0 роботою;
- WebDAV та external-folder mobile adapters не входять до stable 1.0.1;
- для `age 0.12.1` зберігається вузько документований RustSec exception для транзитивного P-256/HPKE шляху; MD Notes використовує age X25519. Exception не є загальним вимкненням advisory gate.

## Сумісність даних

Формат звичайних Markdown-сховищ не змінюється. Формат encrypted vault і recovery contract не змінюються. Міграція даних для переходу з 1.0.0 на 1.0.1 не потрібна.

## Власність і ліцензія

MD Notes — proprietary / All Rights Reserved.  
Copyright © 2026 Roman Zavada.
