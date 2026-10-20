# MD Notes

[🇺🇦 Українська](../../README.md) · [🇬🇧 English](README.en.md) · [🇫🇷 Français](README.fr.md) · [🇩🇪 Deutsch](README.de.md) · [🇪🇸 Español](README.es.md) · [🇰🇷 한국어](README.ko.md) · **🇯🇵 日本語**

<!-- x-release-please-start-version -->
> **リリース checkpoint: MD Notes v0.3.13**
>
> **開発状態: ACTIVE — roadmap 段階 v0.3「モバイルプラットフォームと同期」。**

## MD Notes v0.3.13 をダウンロード

**クイックインストール:**

- **Windows 64-bit:** [.exe インストーラー](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.13/MD.Notes_0.3.13_x64-setup.exe) · [MSI](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.13/MD.Notes_0.3.13_x64_en-US.msi) · [portable ZIP](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.13/MD-Notes-0.3.13-Windows-x64-portable.zip)
- **macOS Universal:** [DMG をダウンロード](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.13/MD.Notes_0.3.13_universal.dmg)
- **Linux x86_64:** [AppImage](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.13/MD.Notes_0.3.13_amd64.AppImage) · [DEB](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.13/MD.Notes_0.3.13_amd64.deb) · [RPM](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.13/MD.Notes-0.3.13-1.x86_64.rpm)

[**v0.3.13 のすべてのリリースファイル →**](https://github.com/RomanZavadaM/md-notes/releases/tag/v0.3.13)
<!-- x-release-please-end -->

> 現在のビルドは未署名/未公証のため、OS が通常のセキュリティ警告を表示することがあります。

## 製品について

**MD Notes** は Markdown 個人ナレッジベースのためのクロスプラットフォーム local-first アプリです。通常の `.md/.markdown` ファイル、添付ファイル、公開 JSON メタデータを扱い、隠された独自形式は作りません。

**データはユーザーのものです。** ノートは自分のファイルシステムに残り、任意のエディターで開き、Git でバージョン管理できます。MD Notes には独自のアプリケーションサーバー、分析、テレメトリーはありません。

プラットフォーム: **Windows、macOS、Linux**。Android と iOS は v0.3 の対象で、CI はビルド可能性を検証していますが、物理端末での runtime 検証はまだ完了扱いではありません。

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
- 7 言語 UI とライト/ダーク/システムテーマ。

### v0.3 — v0.3.4 checkpoint まで統合済み

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
- オプションの暗号化保管庫（age）: ノートは `.age` ファイルとしてディスクに保存され、秘密鍵は端末のシステムのシークレットストア（Windows Credential Manager、macOS Keychain、Linux Secret Service）にのみ保管されます。ダイアログで、作成時の鍵の一度きりのバックアップ、別端末への鍵のインポート、任意の復旧鍵に対応します.

まだ **未完了**: Git push、`SyncManifest` / ADR-0005 との完全な Git state 統合、WebDAV、物理 Android/iOS 端末での runtime 検証、optional Android SAF / iOS security-scoped 外部フォルダー。

**暗号化保管庫**は必須ではなくオプションです。通常の保管庫は従来どおり動作し、`encrypted-vault` feature なしのビルドには暗号化コードが含まれません。現時点では、暗号化保管庫では画像と添付ファイルが表示されず、既存の保管庫に復旧鍵を追加する UI がなく、`.mdnotes/cache` インデックスは暗号化されません。この保管庫からの Git push は、平文を送信しないガード付きで追加される予定です。

全体計画: [roadmap](../roadmap.md)（ウクライナ語、正本）。

## インストール

[リリースページ](https://github.com/RomanZavadaM/md-notes/releases) から Windows (`.exe`, `.msi`, portable ZIP)、macOS (`.dmg`, `.app.tar.gz`)、Linux (`.AppImage`, `.deb`, `.rpm`) のパッケージを取得できます。

**v0.3.4** checkpoint には standalone Markdown と安全な Git fast-forward pipeline が含まれます。個別リリースで別途明記されない限り、ビルドは未署名/未公証です。

詳しい手順: **[ユーザーガイド](../user-guide/USER_GUIDE.ja.md)**。

## 開発、プライバシー、法的情報

Rust stable、Node.js 20+、Tauri のシステム要件が必要です。統合 gate には Rust fmt/clippy/tests、frontend build、dependency license check、desktop CI、関連変更に対する Android/iOS mobile smoke が含まれます。

MD Notes は local-first です。同期 secret は vault、Markdown ファイル、remote URL、Git config、ログ、`localStorage` に保存してはいけません。同期競合は黙って上書きせず、自動 Git apply は clean fast-forward 状態にのみ許可されます。

Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved. MD Notes はプロプライエタリソフトウェアであり、公開リポジトリはオープンソースライセンスを付与しません。ノートはユーザーのものです。[LICENSE.md](../../LICENSE.md) と [法的情報](../LEGAL_AND_COPYRIGHT.md) を参照してください。
