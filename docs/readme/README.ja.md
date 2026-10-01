# MD Notes

[🇺🇦 Українська](../../README.md) · [🇬🇧 English](README.en.md) · [🇫🇷 Français](README.fr.md) · [🇩🇪 Deutsch](README.de.md) · [🇪🇸 Español](README.es.md) · [🇰🇷 한국어](README.ko.md) · **🇯🇵 日本語**

> **現在の checkpoint: [MD Notes v0.2.0](https://github.com/RomanZavadaM/md-notes/releases/tag/v0.2.0)**
>
> **開発状態: ACTIVE — 次の段階 v0.3「モバイルプラットフォームと同期」。**

## 製品

**MD Notes** は、Markdown による個人ナレッジベースのためのクロスプラットフォーム local-first アプリです。通常の `.md` ファイルと添付ファイルを表示、編集、構造化、可視化できます。

**データはユーザーのものです。** ノートはオープンなテキストファイルです。任意のエディターで開き、GitHub で閲覧し、Git でバージョン管理できます。アプリが隠し形式を作ることはありません。

プラットフォーム: **Windows、macOS、Linux**。Android と iOS は v0.3 で対応予定です。

## v0.2.0 の内容

- ローカルフォルダーを保管庫として使用。ファイルツリーで作成、名前変更、ゴミ箱;
- CodeMirror 6 エディター、プレビュー、並列表示、自動保存、アトミックな書き込み;
- `[[ウィキリンク]]`、`aliases`、前後の文脈付きバックリンクパネル;
- **ノートやフォルダーの名前変更・移動時のリンク自動更新**;
- 全文検索、ノート数付きのタグ、`Ctrl+O` クイック移動;
- Mermaid、KaTeX、保管庫の画像;
- ノートテンプレートとデイリーノート;
- アプリ外で変更されたファイルの監視 (デスクトップ);
- 7 言語のインターフェースと「このアプリについて」ウィンドウ;
- ライト、ダーク、システムのテーマ。サンプルのナレッジベース `sample-vault/`。

次の段階 — v0.3: Android と iOS、Git と WebDAV による同期、表とクエリ言語。全体計画: [roadmap](../roadmap.md)(ウクライナ語)。

## インストール

[リリースページ](https://github.com/RomanZavadaM/md-notes/releases) から OS 用のパッケージをダウンロードしてください。

- **Windows** — `MD.Notes_<バージョン>_x64-setup.exe` または `.msi`。コード署名がないため、Windows が SmartScreen を表示することがあります。
- **macOS** — `.dmg` / `.app.tar.gz` (universal)。公証 (notarization) されていないため、**システム設定 → プライバシーとセキュリティ → このまま開く** が必要な場合があります。
- **Linux** — `.AppImage`、`.deb` または `.rpm`。

起動後、**「フォルダーを開く」** を押し、ノートのフォルダーまたはこのリポジトリの `sample-vault/` を選択してください。

詳しい手順: **[ユーザーガイド](../user-guide/USER_GUIDE.ja.md)**。

## 開発者向け

[Rust](https://rustup.rs/) (stable)、[Node.js](https://nodejs.org/) 20 以上、[Tauri のシステム要件](https://v2.tauri.app/start/prerequisites/) が必要です。`app/` で `npm ci` と `npm run tauri dev` を実行します。開発ルール: [PROJECT_RULES.md](../../PROJECT_RULES.md)(ウクライナ語、正本)。

## プライバシーと法的情報

MD Notes は local-first で動作します。ノート、添付ファイル、インデックスはお使いのデバイスまたは選択したストレージに保存されます。サーバー、分析、テレメトリーはありません。同期の競合が黙って上書きされることはありません。保管庫のバックアップを取ってください。

Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved. MD Notes はプロプライエタリソフトウェアであり、公開リポジトリはオープンソースライセンスを付与しません。ノートはユーザーのものです。[LICENSE.md](../../LICENSE.md) と[法的情報](../LEGAL_AND_COPYRIGHT.md)を参照してください。
