# MD Notes

[🇺🇦 Українська](../../README.md) · [🇬🇧 English](README.en.md) · [🇫🇷 Français](README.fr.md) · [🇩🇪 Deutsch](README.de.md) · [🇪🇸 Español](README.es.md) · **🇰🇷 한국어** · [🇯🇵 日本語](README.ja.md)

> **최신 공개 checkpoint: [MD Notes v0.2.2](https://github.com/RomanZavadaM/md-notes/releases/tag/v0.2.2)**
>
> **개발 상태: ACTIVE — roadmap 단계 v0.3 「모바일 플랫폼과 동기화」.**

## 제품 소개

**MD Notes**는 Markdown 개인 지식 베이스를 위한 크로스 플랫폼 local-first 앱입니다. 일반 `.md` 파일과 첨부 파일을 사용하며 숨겨진 독점 데이터 형식을 만들지 않습니다.

**데이터는 사용자의 것입니다.** 노트는 어떤 편집기에서도 열 수 있고, 사용자의 파일 시스템에 저장할 수 있으며 Git으로 버전 관리할 수 있습니다. MD Notes에는 자체 애플리케이션 서버, 분석, 원격 측정이 없습니다.

플랫폼: **Windows, macOS, Linux**. Android와 iOS는 v0.3에서 활발히 개발 중이며 CI가 빌드 가능성을 확인하지만, 실제 기기 런타임 검증이 완료되었다고 주장하지는 않습니다.

## 이미 구현된 기능

### v0.2 — 완료된 기능 기준선

- 로컬 vault, 파일 트리, 생성/이름 변경/휴지통;
- CodeMirror 6, 미리보기/split, 자동 저장, 원자적 쓰기;
- SQLite/FTS5 인덱스, 검색, 태그, 빠른 열기;
- wiki link, aliases, backlinks, rename/move 시 링크 갱신;
- Mermaid 11, KaTeX, 이미지, 첨부 파일, 템플릿, daily notes;
- 열린 `.mdnotes/schema.json` 기반 속성;
- Empty / PARA / Zettelkasten preset;
- global/local knowledge graph;
- 7개 언어 UI와 라이트/다크/시스템 테마.

### v0.3 — 개발 중

이미 `main`에 통합됨:

- provider-neutral `StorageProvider` / `VaultStorage`;
- Android/iOS용 mobile sandbox vault;
- CI의 Android + iOS build smoke;
- local-first sync decision foundation 및 persistent sync state;
- `gix`/gitoxide 기반 HTTPS Git foundation;
- dirty worktree 감지;
- 사용자/시스템 Git config에 의존하지 않는 로컬 Git commit pipeline;
- 안전한 public HTTPS fetch;
- token을 Git config, vault, remote URL에 저장하지 않는 in-memory HTTPS authentication.

현재 security checkpoint는 Tauri 계층에 **시스템 Git credential storage**를 추가합니다: Windows Credential Manager, macOS Keychain, iOS Protected Data, Android Keystore-backed storage, Linux Secret Service. 프런트엔드는 credential을 저장/확인/삭제할 수 있지만 token을 다시 읽을 수는 없습니다. 인증 fetch는 네트워크 호출 직전에 Rust 내부에서만 secret을 읽습니다.

아직 **완료되지 않음**: Git pull/merge 정책, push, conflict policy와의 완전한 통합, WebDAV, 실제 Android/iOS 기기 runtime 검증, optional Android SAF / iOS security-scoped 외부 폴더.

전체 계획: [roadmap](../roadmap.md) (우크라이나어, 기준 문서).

## 설치

[릴리스 페이지](https://github.com/RomanZavadaM/md-notes/releases)에서 Windows (`.exe`, `.msi`, portable ZIP), macOS (`.dmg`, `.app.tar.gz`), Linux (`.AppImage`, `.deb`, `.rpm`) 패키지를 받을 수 있습니다.

최신 공개 prerelease는 **v0.2.2**입니다. 현재 빌드는 서명/공증되지 않았으므로 운영체제가 일반적인 보안 경고를 표시할 수 있습니다.

전체 안내: **[사용자 가이드](../user-guide/USER_GUIDE.ko.md)**.

## 개발, 개인정보 및 법적 고지

Rust stable, Node.js 20+, Tauri 시스템 요구 사항이 필요합니다. 통합 gate에는 Rust format/clippy/tests, frontend build, dependency license check, desktop CI, mobile/Tauri 경계 변경 시 Android/iOS mobile smoke가 포함됩니다.

MD Notes는 local-first입니다. 동기화 secret은 vault, Markdown 파일, remote URL, Git config, 로그, `localStorage`에 저장해서는 안 되며 운영체제 credential store에만 보관해야 합니다. 동기화 충돌은 조용히 덮어쓰면 안 됩니다.

Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved. MD Notes는 독점 소프트웨어이며 공개 저장소는 오픈 소스 라이선스를 부여하지 않습니다. 노트는 사용자의 것입니다. [LICENSE.md](../../LICENSE.md)와 [법적 고지](../LEGAL_AND_COPYRIGHT.md)를 참고하세요.
