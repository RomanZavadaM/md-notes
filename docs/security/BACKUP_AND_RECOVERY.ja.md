# Backup と復旧 — 仕組みと安全な手順

[🇺🇦 Українська](BACKUP_AND_RECOVERY.uk.md) · [🇬🇧 English](BACKUP_AND_RECOVERY.en.md) · [🇫🇷 Français](BACKUP_AND_RECOVERY.fr.md) · [🇩🇪 Deutsch](BACKUP_AND_RECOVERY.de.md) · [🇪🇸 Español](BACKUP_AND_RECOVERY.es.md) · [🇰🇷 한국어](BACKUP_AND_RECOVERY.ko.md) · **🇯🇵 日本語**

> MD Notes v0.3.23 を対象とします。ウクライナ語版が正本です。Backup/restore は v0.3.22 から実装され自動 desktop テストがありますが、手順全体の独立した owner runtime 試験はまだ記録されていません。

## 1. Backup、同期、Git は別物

**Backup** は復旧用の自己完結した ZIP snapshot です。**同期**はコピーを最新に保ちますが、誤削除や破損も伝播する可能性があります。**Git** は履歴を持ちますが、すべてのローカルデータを自動的に含むわけではなく、暗号化 vault の鍵バックアップにはなりません。

作業 vault と同期経路から独立した backup を少なくとも1つ持ってください。

## 2. 「Backup を作成…」の処理

ファイルメニューのコマンドは:
1. 開いている文書の保存を試みる;
2. vault の物理ファイルを走査する;
3. 非移植データを除外する;
4. 各対象ファイルの SHA-256 を計算する;
5. ZIP を書く;
6. 開かれた `manifest.json` を追加する;
7. 処理中に元データが変わっていないか再確認する;
8. 成功時だけ最終 backup 名に確定する。

snapshot 作成中に vault が変わった場合、混合状態を静かに保存せず失敗するべきです。

## 3. 含まれるもの

Markdown ノート、添付、移植可能な config/schema/templates/views、`.mdnotes/trash/`、必要な暗号化 vault 設定。

## 4. 除外されるもの

- `.git/`
- `.mdnotes/cache/`
- MD Notes 一時ファイル
- `.DS_Store`, `Thumbs.db`, `desktop.ini`
- age 秘密鍵/recovery 秘密鍵

シンボリックリンクは vault 外への予期しない参照を防ぐため拒否されます。

## 5. Manifest

`manifest.json` には `md-notes-backup`、形式バージョン、アプリバージョン、作成時刻、vault 名、encrypted フラグ、各ファイルの相対パス・サイズ・SHA-256 が入ります。

目的は **完全性と整合性の検証**であり、暗号化ではありません。manifest は開かれたメタデータです。

## 6. v1 の安全上限

最大100,000ファイル、1ファイル4 GiB、合計16 GiB。絶対パス、`..`、path traversal、重複パス、大文字小文字だけ異なる衝突は許可しません。

## 7. Restore 前の検証

MD Notes はまず manifest、形式/バージョン、相対パス、ZIP エントリ、サイズ、SHA-256 を検証し、不足・余分なファイルがないか確認します。1つでも失敗すれば作業 vault を作る前に停止します。

## 8. 復旧先

検証後、いったん一時フォルダーに展開し、成功結果のみ **新しい** vault にします。既存フォルダーを黙って上書きしません: `My Vault-restored`, `My Vault-restored-2` など。

## 9. 通常 vault

通常 vault の ZIP は読める Markdown と添付を含むため、元フォルダーと同じように保護してください。

## 10. 暗号化 vault

Backup は物理ディスク状態を読むため、age ファイルは暗号文のまま ZIP に入ります。`.mdnotes/encryption.json` など一部のサービス情報は開かれ、cache と秘密鍵は除外されます。

完全復旧には:
1. vault backup;
2. recipient に対応する秘密 age 鍵が最低1つ必要です。

[暗号化 vault](ENCRYPTED_VAULT.ja.md)を参照してください。

## 11. 推奨モデル

作業コピー + 別媒体/別場所への定期 backup + 暗号化 vault の recovery key を別管理。重要データには 3-2-1 原則が有効な目標です。

## 12. Backup 手順

重要変更を保存 → backup 作成 → vault 外に ZIP 保存 → 成功確認 → 独立ストレージへコピー → 暗号化 vault なら recovery key が別保管されていることを確認。

## 13. Restore 手順

旧 vault を先に削除しない → 新しい場所へ restore → manifest/checksum 検証完了 → `*-restored` を開く → 重要ノート/添付/グラフを確認 → 必要なら鍵 import → その後で旧コピーを処理。

## 14. 復旧訓練

定期的に実際の backup を別フォルダーへ restore して開いてください。暗号化 vault では使用可能な recovery key の存在も確認します。

## 15. よくある失敗

同じ物理ディスクだけに backup、鍵のない ciphertext、最新データのない鍵、sync を backup と誤認、通常 vault ZIP の公開、検証前に旧 vault 削除、外部大量書換中の backup。

## 16. Checksum エラー

通すために manifest を編集しないでください。不一致は archive が記録 snapshot と一致しないことを意味します。分析用に保持し、別 backup を使ってください。

## 17. AI と自動化

AI は、検証済み restore 前に最後の正常コピーを削除したり、sync/Git を backup と同等扱いしたり、秘密鍵を ZIP に入れたり、checksum/path 検証を回避したり、既存 vault に上書き展開したり、restored vault を確認せず成功を宣言してはいけません。

暗号化データ規則: [ENCRYPTED_VAULT_CONTRACT.md](../ai/ENCRYPTED_VAULT_CONTRACT.md)。

> 自動生成翻訳であり、ネイティブ校閲は未実施です。ウクライナ語版が正本です。
