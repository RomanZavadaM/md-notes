# Backup 및 복구 — 동작 원리와 안전 절차

[🇺🇦 Українська](BACKUP_AND_RECOVERY.uk.md) · [🇬🇧 English](BACKUP_AND_RECOVERY.en.md) · [🇫🇷 Français](BACKUP_AND_RECOVERY.fr.md) · [🇩🇪 Deutsch](BACKUP_AND_RECOVERY.de.md) · [🇪🇸 Español](BACKUP_AND_RECOVERY.es.md) · **🇰🇷 한국어** · [🇯🇵 日本語](BACKUP_AND_RECOVERY.ja.md)

> MD Notes v1.0.0 기준입니다. 우크라이나어가 기준본입니다. Backup/restore는 v0.3.22부터 구현되어 있고 자동 desktop 테스트가 있지만, 전체 절차에 대한 별도의 owner runtime 테스트는 아직 기록되지 않았습니다.

## 1. Backup, 동기화, Git은 다릅니다

**Backup**은 복구용 독립 ZIP snapshot입니다. **동기화**는 최신 상태를 복제하지만 실수나 삭제도 함께 전파할 수 있습니다. **Git**은 버전 이력을 제공하지만 모든 로컬 데이터를 자동 포함하지 않으며 암호화 vault의 키 백업을 대신하지 않습니다.

작업 vault와 동기화 경로에서 독립된 backup을 최소 하나 보관하십시오.

## 2. “Backup 만들기…”가 하는 일

파일 메뉴 명령은:
1. 열린 문서를 저장하려고 시도하고;
2. 현재 vault의 물리 파일을 순회하고;
3. 비이식성 데이터를 제외하고;
4. 포함 파일마다 SHA-256을 계산하고;
5. ZIP을 만들고;
6. 열린 `manifest.json`을 추가하고;
7. 작업 중 원본이 바뀌지 않았는지 다시 확인하고;
8. 성공한 경우에만 최종 backup 이름으로 확정합니다.

snapshot 생성 중 vault가 바뀌면 혼합 상태의 backup을 조용히 만들지 않고 실패해야 합니다.

## 3. 포함

Markdown 노트, 첨부파일, 이식 가능한 config/schema/templates/views, `.mdnotes/trash/`, 해당되는 경우 암호화 vault 설정.

## 4. 제외

- `.git/`
- `.mdnotes/cache/`
- MD Notes 임시 파일
- `.DS_Store`, `Thumbs.db`, `desktop.ini`
- age 개인 키/recovery 개인 키

심볼릭 링크는 vault 외부로 빠져나가는 위험을 막기 위해 거부됩니다.

## 5. Manifest

`manifest.json`에는 `md-notes-backup` 형식, 형식 버전, 앱 버전, 생성 시간, vault 이름, encrypted 여부와 각 파일의 상대 경로, 크기, SHA-256이 기록됩니다.

목적은 **무결성과 완전성 검증**이지 암호화가 아닙니다. manifest 자체는 열린 메타데이터입니다.

## 6. v1 안전 한계

최대 100,000개 파일, 파일당 4 GiB, 총 16 GiB. 절대 경로, `..`, path traversal, 중복 경로, 대소문자만 다른 충돌 경로는 허용하지 않습니다.

## 7. Restore 전 검증

MD Notes는 먼저 manifest 형식/버전, 상대 경로, ZIP 항목, 크기, SHA-256을 검증하고 빠진 파일이나 예상치 못한 파일이 없는지 확인합니다. 하나라도 실패하면 작동 가능한 vault를 만들기 전에 중단합니다.

## 8. 복구 위치

검증 후 임시 폴더에 풀고, 성공한 결과만 **새로운** vault로 승격합니다. 기존 폴더를 조용히 덮어쓰지 않습니다: `My Vault-restored`, `My Vault-restored-2` 등.

## 9. 일반 vault

일반 vault의 ZIP에는 읽을 수 있는 Markdown과 첨부파일이 들어갑니다. 원본 폴더와 같은 수준으로 보호해야 합니다.

## 10. 암호화 vault

Backup은 물리 디스크 상태를 읽기 때문에 age 파일은 암호문 그대로 ZIP에 들어갑니다. `.mdnotes/encryption.json` 같은 일부 서비스 메타데이터는 열려 있고, cache와 개인 키는 제외됩니다.

완전 복구에는:
1. vault backup;
2. recipients에 맞는 age 개인 키 최소 하나가 필요합니다.

[암호화 vault](ENCRYPTED_VAULT.ko.md) 참고.

## 11. 권장 모델

작업 사본 + 다른 매체/위치의 정기 backup + 암호화 vault용 별도 recovery key. 중요한 데이터는 3-2-1 원칙을 목표로 삼을 수 있습니다.

## 12. Backup 절차

중요 변경 저장 → backup 생성 → vault 밖에 ZIP 저장 → 성공 확인 → 독립 저장소에 복사 → 암호화 vault라면 recovery key가 별도 위치에 있는지 확인.

## 13. Restore 절차

기존 vault를 먼저 삭제하지 않기 → 새 폴더로 restore → manifest/checksum 검증 완료 → `*-restored` 열기 → 중요 노트/첨부/그래프 확인 → 필요 시 키 import → 그 뒤에만 이전 복사본 처리.

## 14. 복구 연습

주기적으로 실제 backup을 별도 폴더에 복원해 열어 보십시오. 암호화 vault에서는 사용 가능한 recovery key도 확인해야 합니다.

## 15. 흔한 실수

같은 물리 디스크에만 backup 보관, 키 없는 ciphertext, 최신 데이터 없는 키, sync를 backup으로 착각, 일반 vault ZIP 공개 업로드, 검증 전 기존 vault 삭제, 외부 대량 변경 중 backup 생성.

## 16. Checksum 오류

통과시키려고 manifest를 수정하지 마십시오. checksum 불일치는 archive가 기록된 snapshot과 다르다는 뜻입니다. 분석용으로 보존하고 다른 backup을 사용하십시오.

## 17. AI와 자동화

AI는 검증된 restore 전에 마지막 정상 복사본을 삭제하거나, sync/Git을 backup과 동일시하거나, 개인 키를 ZIP에 넣거나, checksum/path 검증을 우회하거나, 기존 vault 위에 풀거나, restored vault를 확인하지 않고 성공했다고 말해서는 안 됩니다.

암호화 데이터 규칙: [ENCRYPTED_VAULT_CONTRACT.md](../ai/ENCRYPTED_VAULT_CONTRACT.md).

> 자동 생성 번역이며 원어민 검수는 아직 없습니다. 우크라이나어가 기준입니다.

## 1.0 recovery 검증

Desktop 1.0 전에 암호화 vault용 end-to-end recovery drill을 자동화했습니다.

1. 별도의 recovery recipient를 가진 실제 암호화 vault를 생성합니다.
2. Markdown 노트를 쓰고 바이너리 첨부파일을 가져옵니다.
3. portable ZIP backup을 생성합니다.
4. backup을 새 폴더에 복원합니다.
5. 복원된 vault를 **recovery private key**로 다시 엽니다.
6. 암호화 storage provider를 통해 복원된 노트와 첨부파일 바이트를 읽습니다.

이 시나리오는 1.0 전에 Windows, macOS, Linux core gate에서 CI를 통과했습니다. 이는 자동화된 recovery 근거이며 실제 데이터로 수행하는 별도의 owner 수동 테스트를 대체하지 않습니다.

