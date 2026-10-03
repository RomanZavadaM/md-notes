# MD Notes

[🇺🇦 Українська](../../README.md) · [🇬🇧 English](README.en.md) · [🇫🇷 Français](README.fr.md) · [🇩🇪 Deutsch](README.de.md) · [🇪🇸 Español](README.es.md) · [🇰🇷 한국어](README.ko.md) · **🇯🇵 日本語**

> **最新の公開 checkpoint: [MD Notes v0.2.2](https://github.com/RomanZavadaM/md-notes/releases/tag/v0.2.2)**
>
> **開発状態: ACTIVE — roadmap 段階 v0.3「モバイルプラットフォームと同期」。**

## 製品について

**MD Notes** は Markdown 個人ナレッジベースのためのクロスプラットフォーム local-first アプリです。通常の `.md` ファイルと添付ファイルを扱い、隠された独自データ形式は作りません。

**データはユーザーのものです。** ノートは任意のエディターで開き、自分のファイルシステムに保存し、Git でバージョン管理できます。MD Notes には独自のアプリケーションサーバー、分析、テレメトリーはありません。

プラットフォーム: **Windows、macOS、Linux**。Android と iOS は v0.3 で積極的に開発中です。CI はビルド可能性を確認していますが、物理端末での runtime 検証が完了したとはまだ表明していません。

## 実装済み

### v0.2 — 完了した機能ベースライン

- ローカル vault、ファイルツリー、作成/名前変更/ゴミ箱;
- CodeMirror 6、preview/split、自動保存、アトミック書き込み;
- SQLite/FTS5 インデックス、検索、タグ、quick open;
- wiki link、aliases、backlinks、rename/move 時のリンク更新;
- Mermaid 11、KaTeX、画像、添付ファイル、テンプレート、daily notes;
- オープンな `.mdnotes/schema.json` による schema-driven properties;
- Empty / PARA / Zettelkasten preset;
- global/local knowledge graph;
- 7 言語 UI とライト/ダーク/システムテーマ。

### v0.3 — 開発中

すでに `main` に統合済み:

- provider-neutral `StorageProvider` / `VaultStorage`;
- Android/iOS 用 mobile sandbox vault;
- CI の Android + iOS build smoke;
- local-first sync decision foundation と persistent sync state;
- `gix`/gitoxide による HTTPS Git foundation;
- dirty worktree 検出;
- user/system Git config に依存しないローカル Git commit pipeline;
- 安全な public HTTPS fetch;
- token を Git config、vault、remote URL に保存しない in-memory HTTPS authentication。

現在の security checkpoint では Tauri レイヤーに **システム Git credential storage** を追加します。対象は Windows Credential Manager、macOS Keychain、iOS Protected Data、Android Keystore-backed storage、Linux Secret Service です。フロントエンドは credential の保存/確認/削除はできますが token を読み戻せません。認証 fetch はネットワーク呼び出し直前に Rust 内部だけで secret を取得します。

まだ **未完了**: Git pull/merge policy、push、conflict policy との完全統合、WebDAV、物理 Android/iOS 端末での runtime 検証、optional Android SAF / iOS security-scoped 外部フォルダー。

全体計画: [roadmap](../roadmap.md)（ウクライナ語、正本）。

## インストール

[リリースページ](https://github.com/RomanZavadaM/md-notes/releases) から Windows (`.exe`, `.msi`, portable ZIP)、macOS (`.dmg`, `.app.tar.gz`)、Linux (`.AppImage`, `.deb`, `.rpm`) のパッケージを取得できます。

最新の公開 prerelease は **v0.2.2** です。現在のビルドは未署名/未公証のため、OS が通常のセキュリティ警告を表示することがあります。

詳しい手順: **[ユーザーガイド](../user-guide/USER_GUIDE.ja.md)**。

## 開発、プライバシー、法的情報

Rust stable、Node.js 20+、Tauri のシステム要件が必要です。統合 gate には Rust format/clippy/tests、frontend build、dependency license check、desktop CI、mobile/Tauri 境界変更時の Android/iOS mobile smoke が含まれます。

MD Notes は local-first です。同期 secret は vault、Markdown ファイル、remote URL、Git config、ログ、`localStorage` に保存してはならず、OS の credential store に保存します。同期競合は黙って上書きしてはいけません。

Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved. MD Notes はプロプライエタリソフトウェアであり、公開リポジトリはオープンソースライセンスを付与しません。ノートはユーザーのものです。[LICENSE.md](../../LICENSE.md) と [法的情報](../LEGAL_AND_COPYRIGHT.md) を参照してください。
