# 암호화 저장소 — 동작 원리, 키, 보호 범위

[🇺🇦 Українська](ENCRYPTED_VAULT.uk.md) · [🇬🇧 English](ENCRYPTED_VAULT.en.md) · [🇫🇷 Français](ENCRYPTED_VAULT.fr.md) · [🇩🇪 Deutsch](ENCRYPTED_VAULT.de.md) · [🇪🇸 Español](ENCRYPTED_VAULT.es.md) · **🇰🇷 한국어** · [🇯🇵 日本語](ENCRYPTED_VAULT.ja.md)

> MD Notes v0.3.23 기준입니다. 우크라이나어 문서가 기준본입니다. ADR-0007의 설계 의도만이 아니라 실제 제품 동작을 설명합니다.

## 1. 목적

일반 vault는 열린 Markdown 파일입니다. 선택 기능인 **암호화 vault**는 외장 디스크, Git, 동기화된 클라우드 폴더, 백업, 분실 장치 등에 저장소 사본이 노출될 때 내용을 보호합니다.

개방형 **age** 형식과 X25519 키를 사용합니다. MD Notes는 자체 암호 알고리즘을 만들지 않습니다.

## 2. 핵심 원리

앱은 논리적으로 `projects/secret-note.md`를 다루지만, 물리 저장소에 쓰기 전에 메모리에서 데이터를 암호화합니다.

```mermaid
flowchart LR
  A[메모리의 Markdown] --> B[MD Notes]
  B --> C[메모리에서 age 암호화]
  C --> D[디스크 / Git / backup의 암호문]
  D --> E[메모리에서 복호화]
  E --> B
```

하위 storage provider는 노트나 첨부파일의 평문을 받아서는 안 됩니다.

## 3. 새 vault의 무작위 이름

v0.3.23에서 새 암호화 vault는 **random names**를 사용합니다. 예: `k8r2p6m1q4x7z9ab.age`. 논리 경로는 암호화된 envelope 안에 저장됩니다.

따라서 새 vault에서는 파일 시스템과 Git에 노트/폴더 이름이 드러나지 않습니다. 이전 vault는 `named` 모드일 수 있으며, 이 경우 이름은 보이지만 내용은 암호화됩니다.

무작위 이름도 파일 개수, 대략적인 크기, 수정 시간, 열린 서비스 메타데이터까지 숨기지는 않습니다.

## 4. 무엇이 암호화되는가

일반 vault 파일과 `.mdnotes/trash/`는 암호화됩니다. 나머지 `.mdnotes/`는 대체로 열린 서비스 메타데이터입니다.

`.mdnotes/encryption.json`은 비밀 파일이 아닙니다. 형식/버전, `vaultId`, 공개 recipient `age1...`, 이름 모드를 포함하며 개인 키는 없습니다.

## 5. 개인 키

개인 키는 `AGE-SECRET-KEY-...` 형태입니다.

vault, `localStorage`, Git, 로그, vault backup ZIP에 저장하면 안 됩니다.

MD Notes는 Windows Credential Manager, macOS Keychain, Linux Secret Service 같은 OS 비밀 저장소에 보관합니다.

새 vault를 만들 때 사용자는 독립적인 복구용 키 사본을 보관해야 합니다. 사용 가능한 모든 개인/recovery 키를 잃으면 암호문을 복구할 수 없습니다.

## 6. 다른 장치에서 열기

vault를 복사/복원하고 엽니다. 새 장치에 키가 없으면 `AGE-SECRET-KEY-...`를 별도로 가져옵니다. 이후 그 장치의 OS secret store에 저장됩니다.

독립적인 보안을 원한다면 vault와 개인 키를 같은 채널로 전송하지 마십시오.

## 7. Recovery recipient

age는 여러 공개 recipient에 동시에 암호화할 수 있습니다. 권장 모델: 작업용 개인 키는 장치 secret store, recovery 개인 키는 별도/offline, vault에는 공개키만 저장.

1.0 안정화에서는 **이미 만든** 암호화 vault에도 **파일 → “복구 키 관리…”**에서 recovery recipient를 추가할 수 있습니다. 공개 `age1…` 키만 입력하며 MD Notes가 모든 recipient용으로 vault 파일을 다시 암호화합니다. 개인 recovery 키는 vault 밖에 보관합니다.

## 8. 캐시/인덱스 경계

`.mdnotes/cache/index.db`는 암호화 vault에서도 열린 로컬 캐시로 남습니다. 검색과 그래프에 필요한 파생 평문 데이터가 들어갈 수 있습니다.

캐시는 동기화되지 않고 Git에 올라가지 않으며 내장 backup에서도 제외됩니다. 보호는 OS 계정과 디스크 암호화에 의존합니다.

따라서 “encrypted vault”는 MD Notes의 모든 바이트를 완전히 암호화한다는 뜻이 아닙니다.

## 9. 첨부파일과 이미지

1.0 안정화에서는 암호화 vault의 이미지와 PDF를 active storage provider를 통해 읽고 **메모리에서만** 복호화한 뒤 in-memory Blob URL로 표시합니다. 디스크에 평문 임시 파일을 만들지 않습니다.

암호화 첨부파일의 외부 **“파일 열기”**는 의도적으로 비활성화됩니다. 다른 프로그램에 파일을 넘기려면 디스크에 평문 복사본이 필요하기 때문입니다. inline preview가 없는 다른 첨부 형식도 vault 안에서는 계속 올바르게 암호화됩니다.

## 10. Git

암호화 vault의 Git push에는 평문 전송을 막기 위한 guard가 있습니다. 새 random-name vault는 Git에 무작위 `*.age` 파일을 보여 주며, 이전 `named` vault는 경로 이름이 노출될 수 있습니다.

Git은 개인 키 백업이 아닙니다.

## 11. Backup

내장 backup은 물리 디스크 상태를 복사하므로 암호화 vault에서는 암호문을 복사합니다. `.git`, `.mdnotes/cache`, 개인 키, recovery 개인 키는 포함하지 않습니다.

backup 파일과 사용 가능한 복구 키를 **서로 독립된 두 요소**로 보관하십시오. [Backup 및 복구](BACKUP_AND_RECOVERY.ko.md)를 참고하십시오.

## 12. 보호하는 것

키가 없는 vault 사본 — 도난당한 ZIP, Git 사본, 동기화 폴더, 별도 저장매체 — 의 내용을 읽기 어렵게 합니다.

## 13. 보호하지 않는 것

이미 잠금 해제된 OS 계정에서 실행되는 악성코드, secret store 침해, 화면 캡처, 모든 키 분실, 로컬 캐시의 평문 파생 데이터, 파일 수/크기/시간 분석은 막지 못합니다.

민감 데이터는 BitLocker/FileVault/LUKS 같은 디스크 암호화와 함께 사용하십시오.

## 14. 안전한 절차

**생성**: 신뢰 환경 → 키를 별도 보관 → 다시 열기 테스트 → 첫 backup.

**이동**: 키 없이 vault 복사 → 새 장치에 MD Notes 설치 → vault 열기 → 다른 채널로 키 가져오기 → 여러 노트와 그래프 확인.

**장치 분실**: backup/Git/sync에서 암호문을 복원하고 키는 별도로 복원합니다. MD Notes에는 age를 우회하는 master key가 없습니다.

## 15. 자동화와 AI

규범 규칙:
- [ENCRYPTED_VAULT_CONTRACT.md](../ai/ENCRYPTED_VAULT_CONTRACT.md)
- [encrypted-vault-contract.yaml](../ai/encrypted-vault-contract.yaml)

AI는 개인 키를 vault/Git/log에 기록하거나, 편의를 위해 지속적인 평문 복사본을 만들거나, 캐시가 암호화되었다고 주장하거나, 잃어버린 키를 MD Notes가 복구할 수 있다고 말해서는 안 됩니다.

> 자동 생성 번역이며 원어민 검수를 거치지 않았습니다. 우크라이나어 문서가 기준입니다.
