# MD Notes — Benutzerhandbuch

[🇺🇦 Українська](USER_GUIDE.uk.md) · [🇬🇧 English](USER_GUIDE.en.md) · [🇫🇷 Français](USER_GUIDE.fr.md) · **🇩🇪 Deutsch** · [🇪🇸 Español](USER_GUIDE.es.md) · [🇰🇷 한국어](USER_GUIDE.ko.md) · [🇯🇵 日本語](USER_GUIDE.ja.md)

Gilt für **v0.1.0**. Maßgeblich ist die ukrainische Fassung.

## 1. Erster Start

1. Installieren Sie MD Notes für Ihr Betriebssystem (siehe README).
2. Klicken Sie auf **„Ordner öffnen“** und wählen Sie einen Ordner mit Notizen. Jeder Ordner mit `.md`-Dateien eignet sich, auch `sample-vault/` aus dem Repository.
3. MD Notes merkt sich den Ordner und öffnet ihn beim nächsten Start.

Das Öffnen eines Ordners verändert ihn nicht. Der Dienstordner `.mdnotes/` wird erst angelegt, wenn er wirklich gebraucht wird (Papierkorb, Einstellungen, Index).

## 2. Das Fenster

- **Werkzeugleiste** — Seitenleisten-Schaltfläche ☰, Name des Tresors, Titel der geöffneten Notiz (ein Punkt bedeutet ungespeicherte Änderungen), die Modi „Editor“ / „Nebeneinander“ / „Vorschau“ und die Design-Auswahl.
- **Seitenleiste** — der Dateibaum des Tresors. Ordner stehen zuerst; versteckte Ordner (`.mdnotes`, `.git`) werden nicht angezeigt. Dateien anderer Formate sind ausgegraut.
- **Arbeitsbereich** — Editor, Vorschau oder beides nebeneinander.
- **Statusleiste** — Pfad der Notiz, Tags, Anzahl der Links, Fehler in Eigenschaften und Speicherstatus.

Auf schmalen Bildschirmen öffnet sich die Seitenleiste über dem Inhalt und schließt sich nach der Auswahl einer Notiz.

## 3. Notizen und Ordner

- **+ Notiz** legt eine Notiz im ausgewählten Ordner an (oder neben der ausgewählten Datei). Der Titel wird zum Dateinamen; in Dateinamen unzulässige Zeichen werden entfernt.
- Eine neue Notiz erhält die Eigenschaften `id` (stabile Kennung), `type: note` und `created`.
- **+ Ordner** legt einen Ordner an.
- **✎** benennt das ausgewählte Element um. Die Endung `.md` wird automatisch ergänzt.
- **🗑** verschiebt das Element in den Papierkorb des Tresors `.mdnotes/trash/`. Nichts wird endgültig gelöscht: Die Datei lässt sich von Hand zurückholen.

## 4. Bearbeiten

- Änderungen werden kurz nach dem Tippen automatisch gespeichert, außerdem mit `Ctrl+S` / `Cmd+S`.
- Das Schreiben ist atomar: Ein Fehler mitten im Speichern hinterlässt nie eine beschädigte Datei.
- **„Nebeneinander“** zeigt Editor und Vorschau gleichzeitig. Die Vorschau unterstützt GitHub Flavored Markdown: Tabellen, Aufgabenlisten, Durchstreichen.

## 5. Links

- `[[Titel der Notiz]]` verweist auf eine andere Notiz und ist in der Vorschau anklickbar.
- `[[Titel|Text]]` zeigt einen anderen Text, `[[Titel#Abschnitt]]` verweist auf einen Abschnitt.
- Gibt es keine Notiz mit diesem Namen, wird sie per Klick angelegt.
- Links werden über den Pfad und dann über den Dateinamen aufgelöst, ohne Groß-/Kleinschreibung zu beachten. Links innerhalb von Code werden ignoriert.
- Normale `https://…`-Links öffnen sich im Systembrowser.

## 6. Tags und Eigenschaften

- `#Tag` im Text oder eine `tags`-Liste in den Eigenschaften kennzeichnet ein Thema. Verschachtelte Tags werden unterstützt: `#projekt/design`.
- Eigenschaften stehen als YAML-Front-Matter am Anfang der Datei zwischen `---`-Zeilen.
- In der Vorschau sind die Eigenschaften im Block **„Eigenschaften“** eingeklappt.
- Enthält das YAML einen Fehler, zeigt die Statusleiste ihn an, und die Notiz öffnet sich als normaler Text.

## 7. Designs

Wählen Sie **System**, **Hell** oder **Dunkel** in der rechten Ecke der Werkzeugleiste. Die Auswahl wird gespeichert.

## 8. Wo die Daten liegen

- Notizen sind gewöhnliche Dateien im gewählten Ordner und bleiben ohne MD Notes nutzbar.
- `.mdnotes/` ist der Dienstordner des Tresors: Einstellungen, Vorlagen, Papierkorb, Cache.
- MD Notes sendet Ihre Daten nirgendwohin: keine Server, keine Analyse, keine Telemetrie.
- Sichern Sie den Tresor-Ordner. Git eignet sich gut zur Versionierung.

## 9. Tastatur

| Aktion | Windows / Linux | macOS |
|---|---|---|
| Speichern | `Ctrl+S` | `Cmd+S` |
| Rückgängig / Wiederholen | `Ctrl+Z` / `Ctrl+Shift+Z` | `Cmd+Z` / `Cmd+Shift+Z` |
| Einrücken | `Tab` | `Tab` |

## 10. Häufige Probleme

- **Windows zeigt SmartScreen** — der Build ist nicht signiert. Wählen Sie „Weitere Informationen → Trotzdem ausführen“.
- **macOS öffnet die App nicht** — der Build ist nicht notarisiert. Nutzen Sie **Systemeinstellungen → Datenschutz & Sicherheit → Dennoch öffnen**.
- **Der Tresor öffnet sich beim Start nicht** — der Ordner wurde umbenannt oder verschoben. Öffnen Sie ihn erneut über **„Anderer Tresor…“**.
- **Eine Notiz öffnet sich nicht** — MD Notes öffnet nur `.md`- und `.markdown`-Dateien.

## 11. Urheberrecht

Copyright © 2026 Roman Zavada (Роман Завада). Alle Rechte vorbehalten. MD Notes ist proprietäre Software. Ihre Notizen gehören Ihnen. Siehe [LICENSE.md](../../LICENSE.md) und [LEGAL_AND_COPYRIGHT.md](../LEGAL_AND_COPYRIGHT.md).
