# MD Notes

[🇺🇦 Українська](../../README.md) · [🇬🇧 English](README.en.md) · [🇫🇷 Français](README.fr.md) · **🇩🇪 Deutsch** · [🇪🇸 Español](README.es.md) · [🇰🇷 한국어](README.ko.md) · [🇯🇵 日本語](README.ja.md)

> **Aktueller Checkpoint: [MD Notes v0.1.0](https://github.com/RomanZavadaM/md-notes/releases/tag/v0.1.0)**
>
> **Entwicklungsstatus: ACTIVE — Stufe v0.2 „Struktur und Verknüpfungen“.**

## Über das Produkt

**MD Notes** ist eine plattformübergreifende Local-first-App für eine persönliche Wissensdatenbank in Markdown: einfache `.md`-Dateien samt Anhängen ansehen, bearbeiten, strukturieren und visualisieren.

**Ihre Daten gehören Ihnen.** Notizen sind offene Textdateien. Sie lassen sich in jedem Editor öffnen, auf GitHub ansehen und mit Git versionieren. Die App erzeugt keine versteckten Formate.

Plattformen: **Windows, macOS, Linux**; Android und iOS sind für v0.3 geplant.

## Neu in v0.1.0

- ein lokaler Ordner als Tresor (Vault); Dateibaum mit Anlegen, Umbenennen und Papierkorb;
- CodeMirror-6-Editor, Vorschau, Nebeneinander-Modus, automatisches Speichern, atomares Schreiben;
- `[[Wiki-Links]]`, die die Zielnotiz öffnen oder sie anlegen, falls sie fehlt;
- `#Tags` und Eigenschaften im YAML-Front-Matter;
- helles, dunkles und System-Design; Layout für schmale Bildschirme;
- die Beispiel-Wissensdatenbank `sample-vault/`.

In Arbeit (v0.2): Index und Volltextsuche, Rückverweise, Aktualisierung von Links beim Umbenennen, Mermaid und KaTeX, Vorlagen und Tagesnotizen, Oberfläche in sieben Sprachen. Vollständiger Plan: [Roadmap](../roadmap.md) (Ukrainisch).

## Installation

Laden Sie das Paket für Ihr Betriebssystem von der [Release-Seite](https://github.com/RomanZavadaM/md-notes/releases/latest):

- **Windows** — `MD.Notes_<Version>_x64-setup.exe` oder `.msi`. Der Build ist nicht signiert, daher kann Windows SmartScreen anzeigen.
- **macOS** — `.dmg` / `.app.tar.gz` (Universal). Der Build ist nicht notarisiert; eventuell ist **Systemeinstellungen → Datenschutz & Sicherheit → Dennoch öffnen** nötig.
- **Linux** — `.AppImage`, `.deb` oder `.rpm`.

Klicken Sie nach dem Start auf **„Ordner öffnen“** und wählen Sie einen Ordner mit Notizen oder `sample-vault/` aus diesem Repository.

Vollständige Anleitung: **[Benutzerhandbuch](../user-guide/USER_GUIDE.de.md)**.

## Für Entwickler

Benötigt werden [Rust](https://rustup.rs/) (stable), [Node.js](https://nodejs.org/) 20+ und die [Tauri-Systemvoraussetzungen](https://v2.tauri.app/start/prerequisites/). In `app/` `npm ci` und `npm run tauri dev` ausführen. Entwicklungsregeln: [PROJECT_RULES.md](../../PROJECT_RULES.md) (Ukrainisch, maßgeblich).

## Datenschutz und Rechtliches

MD Notes arbeitet local-first: Notizen, Anhänge und der Index bleiben auf Ihrem Gerät oder in dem von Ihnen gewählten Speicher. Es gibt keine Server, keine Analyse und keine Telemetrie. Synchronisationskonflikte werden nie stillschweigend überschrieben. Erstellen Sie Sicherungen Ihrer Tresore.

Copyright © 2026 Roman Zavada (Роман Завада). Alle Rechte vorbehalten. MD Notes ist proprietäre Software; das öffentliche Repository gewährt keine Open-Source-Lizenz. Ihre Notizen gehören Ihnen. Siehe [LICENSE.md](../../LICENSE.md) und die [rechtlichen Hinweise](../LEGAL_AND_COPYRIGHT.md).
