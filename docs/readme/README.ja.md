# MD Notes

[🇺🇦 Українська](../../README.md) · [🇬🇧 English](README.en.md) · [🇫🇷 Français](README.fr.md) · [🇩🇪 Deutsch](README.de.md) · [🇪🇸 Español](README.es.md) · [🇰🇷 한국어](README.ko.md) · **🇯🇵 日本語**

<!-- x-release-please-start-version -->
> **リリース checkpoint: MD Notes v1.0.2**
>
> **状態: desktop 機能開発完了。stable v1.0.2 を公開済みです。現在は実運用と安定化の段階です。**

## MD Notes v1.0.2 をダウンロード

**クイックインストール:**

- **Windows 64-bit:** [.exe インストーラー](https://github.com/RomanZavadaM/md-notes/releases/download/v1.0.2/MD.Notes_1.0.2_x64-setup.exe) · [MSI](https://github.com/RomanZavadaM/md-notes/releases/download/v1.0.2/MD.Notes_1.0.2_x64_en-US.msi) · [portable ZIP](https://github.com/RomanZavadaM/md-notes/releases/download/v1.0.2/MD-Notes-1.0.2-Windows-x64-portable.zip)
- **macOS Universal:** [DMG をダウンロード](https://github.com/RomanZavadaM/md-notes/releases/download/v1.0.2/MD.Notes_1.0.2_universal.dmg)
- **Linux x86_64:** [AppImage](https://github.com/RomanZavadaM/md-notes/releases/download/v1.0.2/MD.Notes_1.0.2_amd64.AppImage) · [DEB](https://github.com/RomanZavadaM/md-notes/releases/download/v1.0.2/MD.Notes_1.0.2_amd64.deb) · [RPM](https://github.com/RomanZavadaM/md-notes/releases/download/v1.0.2/MD.Notes-1.0.2-1.x86_64.rpm)

[**v1.0.2 のすべてのリリースファイル →**](https://github.com/RomanZavadaM/md-notes/releases/tag/v1.0.2)
<!-- x-release-please-end -->

> 現在のビルドは未署名/未公証のため、OS が通常のセキュリティ警告を表示することがあります。

> **Windows 1.0 パッケージ:** NSIS/MSI は offline WebView2 installer と VC runtime を含みます。軽量 portable ZIP は Windows に既にインストールされている WebView2 Runtime を使用します。

## 製品について

**MD Notes** は Markdown 個人ナレッジベースのためのクロスプラットフォーム local-first アプリです。通常の `.md/.markdown` ファイル、添付ファイル、公開 JSON メタデータを扱い、隠された独自形式は作りません。

**データはユーザーのものです。** ノートは自分のファイルシステムに残り、任意のエディターで開き、Git でバージョン管理できます。MD Notes には独自のアプリケーションサーバー、分析、テレメトリーはありません。

Desktop 1.0 の対象: **Windows、macOS、Linux**。Android/iOS は引き続き CI でビルドしますが、物理端末での runtime 検証と mobile store 公開は 1.0 後に行います。

## 統合済み機能

### v0.2 — 完了した機能ベースライン

- ローカル vault、ファイルツリー、作成/名前変更/ゴミ箱;
- CodeMirror 6、preview/split、自動保存、アトミック書き込み;
- SQLite/FTS5 インデックス、検索、タグ、quick open;
- wiki link、aliases、backlinks、rename/move 時のリンク更新;
- Mermaid 11、KaTeX、画像、添付ファイル、テンプレート、daily notes;
- `.mdnotes/schema.json` による schema-driven properties;
- Empty / PARA / Zettelkasten preset;
- global/local knowledge graph;
- 生きたグラフ物理（「浮遊する」グラフ）、別ウィンドウのグラフ、最も弱いリンクで評価する関係ツリー、2つのノート間の経路；
- 正確なコード（EDRPOU/納税者番号）または名前による対象検索、コードで統合するフォルダーインポート、重複の可能性；
- 7 言語 UI とライト/ダーク/システムテーマ。

### v0.3 — v0.3.18 checkpoint まで統合済み

- provider-neutral `StorageProvider` / `VaultStorage`;
- Android/iOS 用 mobile sandbox vault;
- CI の Android + iOS build smoke;
- local-first sync foundation と persistent sync state;
- `gix`/gitoxide による HTTPS Git foundation;
- dirty worktree 検出とローカル Git commit pipeline;
- 安全な public HTTPS fetch;
- token を保存しない in-memory HTTPS authentication;
- Windows/macOS/iOS/Android/Linux の system Git credential storage;
- vault なしで `.md/.markdown` を開く・編集する・保存する機能;
- Windows/Linux/macOS の Markdown desktop integration。既定アプリ変更はユーザーの明示操作後のみ;
- Git pull planner: `UpToDate / FastForward / LocalAhead / Diverged / Unrelated`;
- dirty worktree をブロックする pull safety preflight;
- clean `FastForward` のみを適用する安全な fast-forward。blind `hard reset`、merge commit、silent overwrite は行いません;
- オプションの暗号化保管庫（age）: ノートは `.age` ファイルとしてディスクに保存され、秘密鍵は端末のシステムのシークレットストア（Windows Credential Manager、macOS Keychain、Linux Secret Service）にのみ保管されます。ダイアログで、作成時の鍵の一度きりのバックアップ、別端末への鍵のインポート、任意の復旧鍵に対応します;
- システムの `git` 経由の Git push（Git のインストールが必要）と暗号化保管庫用ガード（平文は送信されません）、アプリ内 Git パネル（状態、fetch、pull、commit、push）。

まだ **未完了**: `SyncManifest` / ADR-0005 との完全な Git state 統合、WebDAV、物理 Android/iOS 端末での runtime 検証、optional Android SAF / iOS security-scoped 外部フォルダー。

**暗号化保管庫**は引き続きオプションです。通常の保管庫は従来どおり動作し、`encrypted-vault` feature なしのビルドには暗号化コードが含まれません。1.0 安定化では、画像と PDF はメモリ内だけで復号してプレビューし、既存の保管庫にもファイルメニューから公開 recovery recipient を追加できます。暗号化添付の外部表示は平文のディスクコピーを避けるため無効で、`.mdnotes/cache` インデックスは引き続き暗号化されず、Git push の平文防止ガードも維持されます。

全体計画: [roadmap](../roadmap.md)（ウクライナ語、正本）。

> この README、ユーザーガイド、UI 文字列の FR / DE / ES / KO / JA 翻訳は自動生成であり、ネイティブによる校閲を受けていません。基準はウクライナ語の文面です。上記は自動テストと GitHub ビルドで確認済みで、実際のアプリと物理 Android/iOS 端末での手動確認はオーナーがまだ確認していません。

## インストール

[リリースページ](https://github.com/RomanZavadaM/md-notes/releases) から Windows (`.exe`, `.msi`, portable ZIP)、macOS (`.dmg`, `.app.tar.gz`)、Linux (`.AppImage`, `.deb`, `.rpm`) のパッケージを取得できます。

**v0.3.18** checkpoint には standalone Markdown と安全な Git fast-forward pipeline が含まれます。個別リリースで別途明記されない限り、ビルドは未署名/未公証です。

詳しい手順: **[ユーザーガイド](../user-guide/USER_GUIDE.ja.md)**。

## 開発、プライバシー、法的情報

Rust stable、Node.js 20+、Tauri のシステム要件が必要です。統合 gate には Rust fmt/clippy/tests、frontend build、dependency license check、desktop CI、関連変更に対する Android/iOS mobile smoke が含まれます。

MD Notes は local-first です。同期 secret は vault、Markdown ファイル、remote URL、Git config、ログ、`localStorage` に保存してはいけません。同期競合は黙って上書きせず、自動 Git apply は clean fast-forward 状態にのみ許可されます。

Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved. MD Notes はプロプライエタリソフトウェアであり、公開リポジトリはオープンソースライセンスを付与しません。ノートはユーザーのものです。[LICENSE.md](../../LICENSE.md) と [法的情報](../LEGAL_AND_COPYRIGHT.md) を参照してください。


## マルチウィンドウエディター（現在の `main`）

**v0.3.21** では ADR-0010 M1–M3 が公開されています。複数の保管庫ノートをタブで同時に開き、それぞれ独立した下書きを保持できます。デスクトップでは、アクティブなノートを **別ウィンドウで開く** で独立したエディターウィンドウへ移動できます。移動前に変更内容を保存し、メインウィンドウは前のタブまたは空の作業領域に戻ります。

M1–M3 は公開済み **v0.3.21** に含まれます。競合ガード、分離エディターの安全な終了、全保存/安全な終了を実装し、Windows/macOS/Linux CI と Android/iOS build smoke は通過しています。マルチウィンドウ動作は stable **v1.0.0** で受け入れ済みです。実 Windows での確認は stable 公開前に完了し、CI では引き続き Windows/macOS/Linux と Android/iOS build smoke を確認します。


## バックアップと復元

**ファイル** メニューから **バックアップを作成…** と **バックアップを復元…** を利用できます。

- バックアップは `manifest.json` と各ファイルの SHA-256 を含む移動可能な ZIP です。
- バックアップ作成前に、開いているノートを保存します。
- ノート、添付ファイル、設定、テンプレート、ゴミ箱を含み、`.git`、`.mdnotes/cache`、一時的な OS ファイルは除外します。
- 暗号化保管庫は **復号せずに** コピーされ、秘密鍵/復旧キーはアーカイブに保存されません。
- 復元はパス、サイズ、チェックサムを先に検証し、その後 **新しい保管庫フォルダー**を作成して既存の保管庫を上書きしません。

## 詳細な原則と手順

- [知識リンクとグラフ](../concepts/KNOWLEDGE_GRAPH.ja.md)
- [暗号化 Vault](../security/ENCRYPTED_VAULT.ja.md)
- [Backup と復旧](../security/BACKUP_AND_RECOVERY.ja.md)
- [AI/自動化向け正本 contract](../ai/README.md)

