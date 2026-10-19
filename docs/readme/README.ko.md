# MD Notes

[🇺🇦 Українська](../../README.md) · [🇬🇧 English](README.en.md) · [🇫🇷 Français](README.fr.md) · [🇩🇪 Deutsch](README.de.md) · [🇪🇸 Español](README.es.md) · **🇰🇷 한국어** · [🇯🇵 日本語](README.ja.md)

<!-- x-release-please-start-version -->
> **릴리스 체크포인트: MD Notes v0.3.8**
>
> **개발 상태: ACTIVE — roadmap 단계 v0.3 「모바일 플랫폼과 동기화」.**

## MD Notes v0.3.8 다운로드

**빠른 설치:**

- **Windows 64-bit:** [.exe 설치 파일](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.8/MD.Notes_0.3.8_x64-setup.exe) · [MSI](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.8/MD.Notes_0.3.8_x64_en-US.msi) · [portable ZIP](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.8/MD-Notes-0.3.8-Windows-x64-portable.zip)
- **macOS Universal:** [DMG 다운로드](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.8/MD.Notes_0.3.8_universal.dmg)
- **Linux x86_64:** [AppImage](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.8/MD.Notes_0.3.8_amd64.AppImage) · [DEB](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.8/MD.Notes_0.3.8_amd64.deb) · [RPM](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.8/MD.Notes-0.3.8-1.x86_64.rpm)

[**v0.3.8의 모든 릴리스 파일 →**](https://github.com/RomanZavadaM/md-notes/releases/tag/v0.3.8)
<!-- x-release-please-end -->

> 현재 빌드는 서명/공증되지 않았으므로 운영체제가 일반적인 보안 경고를 표시할 수 있습니다.

## 제품 소개

**MD Notes**는 Markdown 개인 지식 베이스를 위한 크로스 플랫폼 local-first 앱입니다. 일반 `.md/.markdown` 파일, 첨부 파일, 공개 JSON 메타데이터를 사용하며 숨겨진 독점 형식을 만들지 않습니다.

**데이터는 사용자의 것입니다.** 노트는 사용자의 파일 시스템에 남고 어떤 편집기에서도 열 수 있으며 Git으로 버전 관리할 수 있습니다. MD Notes에는 자체 애플리케이션 서버, 분석, 원격 측정이 없습니다.

플랫폼: **Windows, macOS, Linux**. Android와 iOS는 v0.3의 활성 대상이며 CI가 빌드 가능성을 검증하지만 실제 기기 runtime 검증은 아직 완료로 선언하지 않습니다.

## 통합된 기능

### v0.2 — 완료된 기능 기준선

- 로컬 vault, 파일 트리, 생성/이름 변경/휴지통;
- CodeMirror 6, 미리보기/split, 자동 저장, 원자적 쓰기;
- SQLite/FTS5 인덱스, 검색, 태그, 빠른 열기;
- wiki link, aliases, backlinks, rename/move 시 링크 갱신;
- Mermaid 11, KaTeX, 이미지, 첨부 파일, 템플릿, daily notes;
- `.mdnotes/schema.json` 기반 속성;
- Empty / PARA / Zettelkasten preset;
- global/local knowledge graph;
- 7개 언어 UI와 라이트/다크/시스템 테마.

### v0.3 — v0.3.4 체크포인트까지 통합됨

- provider-neutral `StorageProvider` / `VaultStorage`;
- Android/iOS용 mobile sandbox vault;
- CI의 Android + iOS build smoke;
- local-first sync foundation 및 persistent sync state;
- `gix`/gitoxide 기반 HTTPS Git foundation;
- dirty worktree 감지와 로컬 Git commit pipeline;
- 안전한 public HTTPS fetch;
- token을 저장하지 않는 in-memory HTTPS authentication;
- Windows/macOS/iOS/Android/Linux 시스템 Git credential storage;
- vault 없이 `.md/.markdown` 열기/편집/저장;
- Windows/Linux/macOS Markdown desktop integration, 기본 앱 변경은 사용자의 명시적 동작 후에만 수행;
- Git pull planner: `UpToDate / FastForward / LocalAhead / Diverged / Unrelated`;
- dirty worktree를 차단하는 pull safety preflight;
- clean `FastForward`에만 적용되는 안전한 fast-forward, blind `hard reset`, merge commit, silent overwrite 없음;
- 선택형 암호화 보관함(age): 노트는 디스크에 `.age` 파일로 저장되고, 개인 키는 기기의 시스템 비밀 저장소(Windows Credential Manager, macOS Keychain, Linux Secret Service)에만 보관됩니다. 대화상자로 생성 시 키 1회 백업, 다른 기기에서 키 가져오기, 선택적 복구 키를 지원합니다.

아직 **완료되지 않음**: Git push, `SyncManifest` / ADR-0005와의 완전한 Git state 통합, WebDAV, 실제 Android/iOS 기기 runtime 검증, optional Android SAF / iOS security-scoped 외부 폴더.

**암호화 보관함**은 필수가 아닌 선택 기능입니다. 일반 보관함은 그대로 동작하며, `encrypted-vault` feature 없이 빌드하면 암호화 코드가 포함되지 않습니다. 현재 암호화 보관함에서는 이미지와 첨부 파일이 표시되지 않고, 이미 만든 보관함에 복구 키를 추가하는 UI가 없으며, `.mdnotes/cache` 인덱스는 암호화되지 않습니다. 이런 보관함의 Git push는 평문 전송을 막는 안전장치와 함께 추가될 예정입니다.

전체 계획: [roadmap](../roadmap.md) (우크라이나어, 기준 문서).

## 설치

[릴리스 페이지](https://github.com/RomanZavadaM/md-notes/releases)에서 Windows (`.exe`, `.msi`, portable ZIP), macOS (`.dmg`, `.app.tar.gz`), Linux (`.AppImage`, `.deb`, `.rpm`) 패키지를 받을 수 있습니다.

**v0.3.4** 체크포인트에는 standalone Markdown과 안전한 Git fast-forward pipeline이 포함됩니다. 특정 릴리스에서 달리 명시하지 않는 한 빌드는 아직 서명/공증되지 않았습니다.

전체 안내: **[사용자 가이드](../user-guide/USER_GUIDE.ko.md)**.

## 개발, 개인정보 및 법적 고지

Rust stable, Node.js 20+, Tauri 시스템 요구 사항이 필요합니다. 통합 gate에는 Rust fmt/clippy/tests, frontend build, dependency license check, desktop CI, 관련 변경에 대한 Android/iOS mobile smoke가 포함됩니다.

MD Notes는 local-first입니다. 동기화 secret은 vault, Markdown 파일, remote URL, Git config, 로그, `localStorage`에 저장해서는 안 됩니다. 동기화 충돌은 조용히 덮어쓰면 안 되며 자동 Git apply는 clean fast-forward 상태에서만 허용됩니다.

Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved. MD Notes는 독점 소프트웨어이며 공개 저장소는 오픈 소스 라이선스를 부여하지 않습니다. 노트는 사용자의 것입니다. [LICENSE.md](../../LICENSE.md)와 [법적 고지](../LEGAL_AND_COPYRIGHT.md)를 참고하세요.
