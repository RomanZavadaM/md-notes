# Changelog

## Unreleased — v0.3 development

### Нові можливості

- provider-neutral storage foundation (`StorageProvider`, `VaultStorage`);
- mobile sandbox vault flow for Android/iOS;
- local-first sync decision/conflict foundation and persistent sync state;
- `gix`/gitoxide HTTPS Git clone/open foundation;
- local Git dirty-state detection and local commit pipeline;
- safe public HTTPS fetch;
- in-memory Git HTTPS authentication without persisting credentials in Git config, vault files or remote URLs;
- system Git credential storage in the Tauri layer: Windows Credential Manager, macOS Keychain, iOS Protected Data, Android Keystore-backed storage and Linux Secret Service;
- authenticated fetch using credentials loaded only inside Rust.

### CI / платформи

- Android aarch64 debug build smoke;
- iOS simulator debug build smoke;
- dependency-license validation for newly added Git and credential-store dependencies.

### Документація

- README status refreshed for the v0.2.2 published baseline and active v0.3 development;
- localized README descriptions synchronized for UK / EN / FR / DE / ES / KO / JA;
- `START_HERE.md`, `PROJECT_STATE.md` and `WORKLOG.md` synchronized with Git PRs #30–#45.

### Ще не завершено у v0.3

- Git pull/merge policy and full conflict integration;
- Git push;
- WebDAV;
- physical-device Android/iOS runtime validation;
- optional Android SAF / iOS security-scoped external-folder adapters.

## [0.2.2](https://github.com/RomanZavadaM/md-notes/compare/v0.2.1...v0.2.2) (2026-10-02)

### Нові можливості

* add global and local knowledge graph ([#25](https://github.com/RomanZavadaM/md-notes/issues/25)) ([39040a4](https://github.com/RomanZavadaM/md-notes/commit/39040a461d0b29f09db382f7bca32483350c19ba))
* add safe vault presets ([#23](https://github.com/RomanZavadaM/md-notes/issues/23)) ([c078d07](https://github.com/RomanZavadaM/md-notes/commit/c078d075d089257c83b903eea44c317f391b765c))
* add schema-driven note properties ([7441259](https://github.com/RomanZavadaM/md-notes/commit/7441259a019eae1f7858898d8a29a9ac4d9ec7cd))

### Виправлення

* keep pre-1.0 releases on patch line ([8ca052c](https://github.com/RomanZavadaM/md-notes/commit/8ca052c6aa69c31f5af8afce186d606b97ffd22a))
* prevent unsafe vault auto-reopen on startup ([de7b9d1](https://github.com/RomanZavadaM/md-notes/commit/de7b9d1b8fa7b9f5c7884b4082d977f4a1029b01))

### Документація

* record functional v0.2 completion ([#26](https://github.com/RomanZavadaM/md-notes/issues/26)) ([16b9d51](https://github.com/RomanZavadaM/md-notes/commit/16b9d51fc5164d0ec2331cd37d39a433b5562a00))
* record the v0.2.1 release checkpoint ([#17](https://github.com/RomanZavadaM/md-notes/issues/17)) ([d74a964](https://github.com/RomanZavadaM/md-notes/commit/d74a9641edaa457203442d68d03e26996943961e))
* sync state after schema integration ([d7d6313](https://github.com/RomanZavadaM/md-notes/commit/d7d63137447ff027caff3e7be77d99629d805740))
* sync state after vault presets ([#24](https://github.com/RomanZavadaM/md-notes/issues/24)) ([e00a172](https://github.com/RomanZavadaM/md-notes/commit/e00a17272c3e5dd0cfd354bdb7bb344dc023fece))

## [0.2.1](https://github.com/RomanZavadaM/md-notes/compare/v0.2.0...v0.2.1) (2026-10-01)

### Виправлення

* complete v0.2 attachment management ([#16](https://github.com/RomanZavadaM/md-notes/issues/16)) ([6b92336](https://github.com/RomanZavadaM/md-notes/commit/6b923369b8d5e3bb4e031f201dd9de7cd8c2c594))
* enable restrictive Tauri CSP ([a8e1106](https://github.com/RomanZavadaM/md-notes/commit/a8e1106b40b5552f453c410d96fbf5f1da9a243a))

### Документація

* record the v0.2.0 release checkpoint ([86a6542](https://github.com/RomanZavadaM/md-notes/commit/86a654275f3f451904e284dede534914a8756f6b))
* record the v0.2.0 release checkpoint in WORKLOG ([014f82b](https://github.com/RomanZavadaM/md-notes/commit/014f82b7160bd3bd4675fffa1b28a192ef0d8b5f))

## [0.2.0](https://github.com/RomanZavadaM/md-notes/compare/v0.1.0...v0.2.0) (2026-10-01)

### Нові можливості

* **app:** backlinks, search, tags, quick switcher and file watching ([fcfaf72](https://github.com/RomanZavadaM/md-notes/commit/fcfaf72a566d196550a45934a37b4828281b437d))
* **app:** interface in seven languages and About dialog ([8333e0b](https://github.com/RomanZavadaM/md-notes/commit/8333e0b2865844d33899d92a8f0c08eecdfebb45))
* **app:** interface language packs UK/EN/FR/DE/ES/KO/JA and About dialog ([f16de78](https://github.com/RomanZavadaM/md-notes/commit/f16de784c05284189357ac83a850052a19b2d4a7))
* **app:** Mermaid, KaTeX, vault images and relative links in preview ([ee5594b](https://github.com/RomanZavadaM/md-notes/commit/ee5594b0a20d65b0bc0e9a8e64a8a2d3d99938c2))
* **app:** update links on rename; docs for v0.2 progress ([36ecbef](https://github.com/RomanZavadaM/md-notes/commit/36ecbefd6a08d324584ebbf7a6cdb4f1a5778d1b))
* **core:** rewrite links when notes or folders are renamed ([3eb0d1c](https://github.com/RomanZavadaM/md-notes/commit/3eb0d1c516f0a9fe8d0e17982cbf9245b971a04d))
* **core:** SQLite index with backlinks, search, tags and aliases ([b47dd70](https://github.com/RomanZavadaM/md-notes/commit/b47dd70237df77fccc777a7ce1434c61755f3e7f))
* note templates and daily notes ([a19c6e6](https://github.com/RomanZavadaM/md-notes/commit/a19c6e6b5d3a536679e5a0337ddfe6194d0d0a2c))

### Виправлення

* **app:** use Mermaid 11, which does not bundle EPL-2.0 elkjs ([4f13e37](https://github.com/RomanZavadaM/md-notes/commit/4f13e37bb1b659c706b0c0293ce11211f903cd96))

### Документація

* adopt project rules, proprietary license notices and seven-language docs ([d188c7c](https://github.com/RomanZavadaM/md-notes/commit/d188c7c615982f2274d892f3b659297ecb7c7205))
* link to the releases list, since 0.x releases are prereleases ([63a89cb](https://github.com/RomanZavadaM/md-notes/commit/63a89cb17b8716963a3bf8141060d7c307aa09cf))
* project rules, proprietary license and seven-language documentation ([8983991](https://github.com/RomanZavadaM/md-notes/commit/8983991ea61f79b4c2865afff16bbec2bb67e463))
* release documentation for v0.2.0 ([478b8cc](https://github.com/RomanZavadaM/md-notes/commit/478b8cc27870a32392b48f2153a53bc3b7005bd1))
* release notes, README and user guides for v0.2.0 in seven languages ([8e78867](https://github.com/RomanZavadaM/md-notes/commit/8e788677e63ddcccd900d11f9e0e1f3b11fdbecd))
* remove the MIT LICENSE file; LICENSE.md is the only license ([3b1de76](https://github.com/RomanZavadaM/md-notes/commit/3b1de76733741157f23c37005829f4facd946209))
* update WORKLOG ([cdc6648](https://github.com/RomanZavadaM/md-notes/commit/cdc664885eab30f3d9b99163d3b273c6d2018261))

## 0.1.0 (2026-10-01)

### Нові можливості

* initial MD Notes v0.1 scaffold ([410b38f](https://github.com/RomanZavadaM/md-notes/commit/410b38fe817def432f8211e5200e03d545e52195))
