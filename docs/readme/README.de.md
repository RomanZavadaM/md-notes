# MD Notes

[🇺🇦 Українська](../../README.md) · [🇬🇧 English](README.en.md) · [🇫🇷 Français](README.fr.md) · **🇩🇪 Deutsch** · [🇪🇸 Español](README.es.md) · [🇰🇷 한국어](README.ko.md) · [🇯🇵 日本語](README.ja.md)

> **Letzter veröffentlichter Checkpoint: [MD Notes v0.2.2](https://github.com/RomanZavadaM/md-notes/releases/tag/v0.2.2)**
>
> **Entwicklungsstatus: ACTIVE — Roadmap-Stufe v0.3 „mobile Plattformen und Synchronisierung“.**

## Über das Produkt

**MD Notes** ist eine plattformübergreifende Local-first-App für eine Markdown-Wissensdatenbank. Sie arbeitet mit normalen `.md`-Dateien und Anhängen ohne verstecktes proprietäres Datenformat.

**Ihre Daten gehören Ihnen.** Notizen können mit jedem Editor geöffnet, im eigenen Dateisystem gespeichert und mit Git versioniert werden. MD Notes hat keinen eigenen Anwendungsserver, keine Analyse und keine Telemetrie.

Plattformen: **Windows, macOS, Linux**. Android und iOS werden in v0.3 aktiv entwickelt; CI bestätigt die Buildbarkeit, die Laufzeit auf physischen Geräten ist jedoch noch nicht als validiert ausgewiesen.

## Bereits umgesetzt

### v0.2 — abgeschlossene funktionale Basis

- lokaler Vault, Dateibaum, Erstellen/Umbenennen/Papierkorb;
- CodeMirror 6, Vorschau/Split, Autosave und atomare Schreibvorgänge;
- SQLite/FTS5-Index, Suche, Tags und Quick Open;
- Wiki-Links, Aliases, Backlinks und Link-Rewrite bei Rename/Move;
- Mermaid 11, KaTeX, Bilder, Anhänge, Vorlagen und Daily Notes;
- schema-gesteuerte Eigenschaften über `.mdnotes/schema.json`;
- Empty / PARA / Zettelkasten Presets;
- globaler/lokaler Wissensgraph;
- Oberfläche in sieben Sprachen sowie Hell/Dunkel/System-Theme.

### v0.3 — aktive Entwicklung

Bereits in `main` integriert:

- provider-neutrales `StorageProvider` / `VaultStorage`;
- Mobile-Sandbox-Vault für Android/iOS;
- Android- und iOS-Build-Smoke in CI;
- Local-first Sync-Entscheidungsgrundlage und persistenter Sync-State;
- HTTPS-Git-Grundlage mit `gix`/gitoxide;
- Erkennung eines veränderten Worktrees;
- lokaler Git-Commit-Pipeline ohne Abhängigkeit von User/System-Git-Konfiguration;
- sicherer öffentlicher HTTPS-Fetch;
- In-Memory-HTTPS-Authentifizierung ohne Token in Git config, Vault oder Remote-URL.

Der aktuelle Security-Checkpoint ergänzt **systemweiten Git-Credential-Speicher** in der Tauri-Schicht: Windows Credential Manager, macOS Keychain, iOS Protected Data, Android Keystore-backed storage und Linux Secret Service. Das Frontend kann Credentials speichern/prüfen/löschen, aber den Token nicht zurücklesen; authentifizierter Fetch liest das Secret nur intern in Rust direkt vor dem Netzwerkaufruf.

Noch **nicht abgeschlossen**: Git Pull/Merge-Policy, Push, vollständige Integration mit der Konfliktpolitik, WebDAV, Runtime-Validierung auf physischen Android/iOS-Geräten sowie optionale Android-SAF-/iOS-security-scoped externe Ordner.

Vollständiger Plan: [Roadmap](../roadmap.md) (Ukrainisch, maßgeblich).

## Installation

Pakete gibt es auf der [Release-Seite](https://github.com/RomanZavadaM/md-notes/releases): Windows (`.exe`, `.msi`, portable ZIP), macOS (`.dmg`, `.app.tar.gz`) und Linux (`.AppImage`, `.deb`, `.rpm`).

Der letzte veröffentlichte Prerelease ist **v0.2.2**. Die Builds sind derzeit nicht signiert/notarisiert, daher kann das Betriebssystem normale Sicherheitswarnungen anzeigen.

Vollständige Anleitung: **[Benutzerhandbuch](../user-guide/USER_GUIDE.de.md)**.

## Entwicklung, Datenschutz und Rechtliches

Benötigt werden Rust stable, Node.js 20+ und die Tauri-Systemvoraussetzungen. Integrations-Gates umfassen Rust format/clippy/tests, Frontend-Build, Lizenzprüfung, Desktop-CI sowie Android/iOS-Mobile-Smoke bei Änderungen an der Mobile/Tauri-Grenze.

MD Notes arbeitet local-first. Sync-Secrets dürfen niemals im Vault, in Markdown-Dateien, Remote-URLs, Git config, Logs oder `localStorage` gespeichert werden; sie gehören in den Credential Store des Betriebssystems. Synchronisationskonflikte dürfen niemals stillschweigend überschrieben werden.

Copyright © 2026 Roman Zavada (Роман Завада). Alle Rechte vorbehalten. MD Notes ist proprietäre Software; das öffentliche Repository gewährt keine Open-Source-Lizenz. Ihre Notizen gehören Ihnen. Siehe [LICENSE.md](../../LICENSE.md) und [rechtliche Hinweise](../LEGAL_AND_COPYRIGHT.md).
