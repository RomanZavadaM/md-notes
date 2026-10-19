# MD Notes

[🇺🇦 Українська](../../README.md) · [🇬🇧 English](README.en.md) · [🇫🇷 Français](README.fr.md) · **🇩🇪 Deutsch** · [🇪🇸 Español](README.es.md) · [🇰🇷 한국어](README.ko.md) · [🇯🇵 日本語](README.ja.md)

> **Release-Checkpoint: MD Notes v0.3.2**
>
> **Entwicklungsstatus: ACTIVE — Roadmap-Stufe v0.3 „mobile Plattformen und Synchronisierung“.**

## MD Notes v0.3.2 herunterladen

**Schnellinstallation:**

- **Windows 64-bit:** [.exe-Installer](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.2/MD.Notes_0.3.2_x64-setup.exe) · [MSI](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.2/MD.Notes_0.3.2_x64_en-US.msi) · [Portable ZIP](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.2/MD-Notes-0.3.2-Windows-x64-portable.zip)
- **macOS Universal:** [DMG herunterladen](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.2/MD.Notes_0.3.2_universal.dmg)
- **Linux x86_64:** [AppImage](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.2/MD.Notes_0.3.2_amd64.AppImage) · [DEB](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.2/MD.Notes_0.3.2_amd64.deb) · [RPM](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.2/MD.Notes-0.3.2-1.x86_64.rpm)

[**Alle Dateien von v0.3.2 →**](https://github.com/RomanZavadaM/md-notes/releases/tag/v0.3.2)

> Die Builds sind derzeit nicht signiert/notarisiert; das Betriebssystem kann eine normale Sicherheitswarnung anzeigen.

## Über das Produkt

**MD Notes** ist eine plattformübergreifende Local-first-App für Markdown-Wissensdatenbanken. Sie arbeitet mit normalen `.md/.markdown`-Dateien, Anhängen und offenen JSON-Metadaten ohne verstecktes proprietäres Format.

**Ihre Daten gehören Ihnen.** Notizen bleiben im eigenen Dateisystem, können mit jedem Editor geöffnet und mit Git versioniert werden. MD Notes hat keinen eigenen Anwendungsserver, keine Analyse und keine Telemetrie.

Plattformen: **Windows, macOS, Linux**. Android und iOS sind aktive v0.3-Ziele; CI bestätigt die Buildbarkeit, Laufzeit auf physischen Geräten ist noch nicht als validiert ausgewiesen.

## Integrierter Funktionsstand

### v0.2 — abgeschlossene funktionale Basis

- lokaler Vault, Dateibaum, Erstellen/Umbenennen/Papierkorb;
- CodeMirror 6, Vorschau/Split, Autosave und atomare Schreibvorgänge;
- SQLite/FTS5-Index, Suche, Tags und Quick Open;
- Wiki-Links, Aliases, Backlinks und Link-Rewrite bei Rename/Move;
- Mermaid 11, KaTeX, Bilder, Anhänge, Vorlagen und Daily Notes;
- schema-gesteuerte Eigenschaften über `.mdnotes/schema.json`;
- Empty / PARA / Zettelkasten Presets;
- globaler/lokaler Wissensgraph;
- Oberfläche in sieben Sprachen und Hell/Dunkel/System-Theme.

### v0.3 — integriert bis zum Checkpoint v0.3.2

- provider-neutrales `StorageProvider` / `VaultStorage`;
- Mobile-Sandbox-Vault für Android/iOS;
- Android- und iOS-Build-Smoke in CI;
- Local-first Sync-Grundlage und persistenter Sync-State;
- HTTPS-Git-Grundlage mit `gix`/gitoxide;
- Dirty-Worktree-Erkennung und lokale Git-Commits;
- sicherer öffentlicher HTTPS-Fetch;
- In-Memory-HTTPS-Authentifizierung ohne Token-Persistenz;
- systemweiter Git-Credential-Speicher für Windows, macOS, iOS, Android und Linux;
- eigenständiges Öffnen/Bearbeiten/Speichern von `.md/.markdown` ohne Vault;
- Markdown-Desktop-Integration unter Windows/Linux/macOS, Standardzuordnung nur nach ausdrücklicher Benutzeraktion;
- Git-Pull-Planer: `UpToDate / FastForward / LocalAhead / Diverged / Unrelated`;
- Preflight blockiert einen schmutzigen Worktree;
- sicherer Fast-Forward nur bei sauberem `FastForward`, ohne blinden `hard reset`, Merge-Commit oder stilles Überschreiben;
- optionaler verschlüsselter Tresor (age): Notizen liegen als `.age`-Dateien auf der Festplatte, der private Schlüssel liegt nur im System-Geheimnisspeicher des Geräts (Windows Credential Manager, macOS Keychain, Linux Secret Service); Dialoge für das Anlegen mit einmaliger Schlüsselsicherung, den Schlüsselimport auf einem anderen Gerät und einen optionalen Wiederherstellungsschlüssel.

Noch **nicht abgeschlossen**: Git Push, vollständige Git-State-Integration mit `SyncManifest` / ADR-0005, WebDAV, Runtime-Validierung auf physischen Android/iOS-Geräten und optionale externe Ordner über Android SAF / iOS security-scoped access.

Der **verschlüsselte Tresor** ist eine Option, keine Pflicht: normale Tresore funktionieren unverändert, und ein Build ohne das Feature `encrypted-vault` enthält keinen Verschlüsselungscode. Bisher zeigt ein verschlüsselter Tresor keine Bilder und Anhänge an, es gibt keine Oberfläche zum Hinzufügen eines Wiederherstellungsschlüssels zu einem bestehenden Tresor, und der Index `.mdnotes/cache` bleibt unverschlüsselt; Git Push aus einem solchen Tresor kommt mit einer Sperre, die keinen Klartext sendet.

Vollständiger Plan: [Roadmap](../roadmap.md) (Ukrainisch, maßgeblich).

## Installation

Pakete gibt es auf der [Release-Seite](https://github.com/RomanZavadaM/md-notes/releases): Windows (`.exe`, `.msi`, portable ZIP), macOS (`.dmg`, `.app.tar.gz`) und Linux (`.AppImage`, `.deb`, `.rpm`).

Der Checkpoint **v0.3.2** enthält Standalone-Markdown und die sichere Git-Fast-Forward-Pipeline. Builds bleiben unsigniert/nicht notarisiert, sofern ein konkretes Release nichts anderes angibt.

Vollständige Anleitung: **[Benutzerhandbuch](../user-guide/USER_GUIDE.de.md)**.

## Entwicklung, Datenschutz und Rechtliches

Benötigt werden Rust stable, Node.js 20+ und die Tauri-Systemvoraussetzungen. Gates umfassen Rust fmt/clippy/tests, Frontend-Build, Lizenzprüfung, Desktop-CI sowie Android/iOS-Mobile-Smoke für relevante Änderungen.

MD Notes arbeitet local-first. Sync-Secrets dürfen niemals im Vault, in Markdown-Dateien, Remote-URLs, Git config, Logs oder `localStorage` gespeichert werden. Konflikte dürfen niemals stillschweigend überschrieben werden; automatisches Git-Apply ist nur für einen sauberen Fast-Forward erlaubt.

Copyright © 2026 Roman Zavada (Роман Завада). Alle Rechte vorbehalten. MD Notes ist proprietäre Software; das öffentliche Repository gewährt keine Open-Source-Lizenz. Ihre Notizen gehören Ihnen. Siehe [LICENSE.md](../../LICENSE.md) und [rechtliche Hinweise](../LEGAL_AND_COPYRIGHT.md).
