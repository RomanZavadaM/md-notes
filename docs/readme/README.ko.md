# MD Notes

[🇺🇦 Українська](../../README.md) · [🇬🇧 English](README.en.md) · [🇫🇷 Français](README.fr.md) · [🇩🇪 Deutsch](README.de.md) · [🇪🇸 Español](README.es.md) · **🇰🇷 한국어** · [🇯🇵 日本語](README.ja.md)

> **현재 checkpoint: [MD Notes v0.2.0](https://github.com/RomanZavadaM/md-notes/releases/tag/v0.2.0)**
>
> **개발 상태: ACTIVE — 다음 단계 v0.3 「모바일 플랫폼과 동기화」.**

## 제품 소개

**MD Notes**는 Markdown 기반 개인 지식 베이스를 위한 크로스 플랫폼 local-first 앱입니다. 일반 `.md` 파일과 첨부 파일을 보고, 편집하고, 구조화하고, 시각화할 수 있습니다.

**데이터는 사용자의 것입니다.** 노트는 열린 텍스트 파일입니다. 어떤 편집기에서도 열 수 있고, GitHub에서 볼 수 있으며, Git으로 버전 관리할 수 있습니다. 앱은 숨겨진 형식을 만들지 않습니다.

플랫폼: **Windows, macOS, Linux**; Android와 iOS는 v0.3에서 지원할 예정입니다.

## v0.2.0 주요 기능

- 로컬 폴더를 보관함으로 사용; 파일 트리에서 생성, 이름 변경, 휴지통;
- CodeMirror 6 편집기, 미리보기, 나란히 보기, 자동 저장, 원자적 쓰기;
- `[[위키 링크]]`, `aliases`, 문맥이 표시되는 백링크 패널;
- **노트와 폴더의 이름 변경·이동 시 링크 자동 갱신**;
- 전문 검색, 노트 수가 표시되는 태그, `Ctrl+O` 빠른 이동;
- Mermaid, KaTeX, 보관함의 이미지;
- 노트 템플릿과 데일리 노트;
- 앱 밖에서 바뀐 파일 감지(데스크톱);
- 7개 언어 인터페이스와 「정보」 창;
- 라이트, 다크, 시스템 테마; 예제 지식 베이스 `sample-vault/`.

다음 단계 — v0.3: Android와 iOS, Git·WebDAV 동기화, 표와 쿼리 언어. 전체 계획: [roadmap](../roadmap.md) (우크라이나어).

## 설치

[릴리스 페이지](https://github.com/RomanZavadaM/md-notes/releases)에서 OS에 맞는 패키지를 내려받으세요.

- **Windows** — `MD.Notes_<버전>_x64-setup.exe` 또는 `.msi`. 코드 서명이 없으므로 Windows가 SmartScreen을 표시할 수 있습니다.
- **macOS** — `.dmg` / `.app.tar.gz` (universal). 공증(notarization)되지 않았으므로 **시스템 설정 → 개인정보 보호 및 보안 → 그래도 열기**가 필요할 수 있습니다.
- **Linux** — `.AppImage`, `.deb` 또는 `.rpm`.

실행 후 **「폴더 열기」**를 누르고 노트 폴더나 이 저장소의 `sample-vault/`를 선택하세요.

전체 안내: **[사용자 가이드](../user-guide/USER_GUIDE.ko.md)**.

## 개발자용

[Rust](https://rustup.rs/) (stable), [Node.js](https://nodejs.org/) 20+, [Tauri 시스템 요구 사항](https://v2.tauri.app/start/prerequisites/)이 필요합니다. `app/`에서 `npm ci`와 `npm run tauri dev`를 실행하세요. 개발 규칙: [PROJECT_RULES.md](../../PROJECT_RULES.md) (우크라이나어, 기준 문서).

## 개인정보 및 법적 고지

MD Notes는 local-first로 동작합니다. 노트, 첨부 파일, 인덱스는 사용자의 기기 또는 사용자가 선택한 저장소에 남습니다. 서버, 분석, 원격 측정이 없습니다. 동기화 충돌은 절대 조용히 덮어쓰지 않습니다. 보관함을 백업해 두세요.

Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved. MD Notes는 독점 소프트웨어이며, 공개 저장소는 오픈 소스 라이선스를 부여하지 않습니다. 노트는 사용자의 것입니다. [LICENSE.md](../../LICENSE.md)와 [법적 고지](../LEGAL_AND_COPYRIGHT.md)를 참고하세요.
