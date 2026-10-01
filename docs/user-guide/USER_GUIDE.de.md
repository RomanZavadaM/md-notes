# MD Notes — Benutzerhandbuch

[🇺🇦 Українська](USER_GUIDE.uk.md) · [🇬🇧 English](USER_GUIDE.en.md) · [🇫🇷 Français](USER_GUIDE.fr.md) · **🇩🇪 Deutsch** · [🇪🇸 Español](USER_GUIDE.es.md) · [🇰🇷 한국어](USER_GUIDE.ko.md) · [🇯🇵 日本語](USER_GUIDE.ja.md)

Gilt für **v0.2.0**. Maßgeblich ist die ukrainische Fassung.

## 1. Erster Start

1. Installieren Sie MD Notes für Ihr Betriebssystem (siehe README).
2. Wählen Sie bei Bedarf die Sprache der Oberfläche auf dem Startbildschirm.
3. Klicken Sie auf **„Ordner öffnen“** und wählen Sie einen Ordner mit Notizen. Jeder Ordner mit `.md`-Dateien eignet sich, auch `sample-vault/` aus dem Repository.
4. MD Notes merkt sich den Ordner und öffnet ihn beim nächsten Start.

Das Öffnen eines Ordners verändert Ihre Notizen nicht. MD Notes legt den Dienstordner `.mdnotes/` für den Suchindex, den Papierkorb und die Einstellungen an.

## 2. Das Fenster

- **Werkzeugleiste**:
  - Seitenleisten-Schaltfläche ☰ und Name des Tresors;
  - Titel der geöffneten Notiz: Ein Klick öffnet den Schnellwechsler, ein Punkt bedeutet ungespeicherte Änderungen;
  - die Modi „Editor“ / „Nebeneinander“ / „Vorschau“;
  - Schaltfläche für die Verknüpfungsleiste ⇆;
  - Design- und Sprachauswahl, Schaltfläche „Über“ ⓘ.
- **Seitenleiste** — die Registerkarten **Dateien**, **Suche** und **Tags**.
  - Im Dateibaum stehen Ordner zuerst, versteckte Ordner (`.mdnotes`, `.git`) werden nicht angezeigt.
  - Dateien anderer Formate sind ausgegraut.
- **Arbeitsbereich** — Editor, Vorschau oder beides nebeneinander.
- **Leiste „Verknüpfungen“** rechts — Rückverweise, ausgehende Links und Tags der geöffneten Notiz.
- **Statusleiste** — Pfad der Notiz, Tags, Anzahl der Links, Fehler in Eigenschaften und Speicherstatus.

Auf schmalen Bildschirmen öffnen sich Seitenleiste und Verknüpfungsleiste über dem Inhalt.

## 3. Notizen und Ordner

- **+ Notiz** legt eine Notiz im ausgewählten Ordner an (oder neben der ausgewählten Datei). Im Dialog lässt sich eine **Vorlage** wählen (Abschnitt 9).
- Der Titel wird zum Dateinamen; in Dateinamen unzulässige Zeichen werden entfernt. Eine neue Notiz erhält die Eigenschaften `id`, `type` und `created`.
- **Heute** öffnet die Tagesnotiz (Abschnitt 9).
- **+ Ordner** legt einen Ordner an.
- **✎** benennt das ausgewählte Element um. Die Endung `.md` wird automatisch ergänzt. **Links auf die umbenannte Notiz oder auf Notizen in einem umbenannten Ordner werden in allen anderen Notizen aktualisiert**, auch in Eigenschaften. Die App zeigt an, in wie vielen Notizen Links aktualisiert wurden.
- **🗑** verschiebt das Element in den Papierkorb des Tresors `.mdnotes/trash/`. Nichts wird endgültig gelöscht: Die Datei lässt sich von Hand zurückholen.

## 4. Bearbeiten

- Änderungen werden kurz nach dem Tippen automatisch gespeichert, außerdem mit `Ctrl+S` / `Cmd+S`.
- Das Schreiben ist atomar: Ein Fehler mitten im Speichern hinterlässt nie eine beschädigte Datei.
- Ändert ein anderes Programm (Editor, Git, Cloud-Client) eine Datei, aktualisiert MD Notes den Baum und lädt die geöffnete Notiz neu. Haben Sie ungespeicherte Änderungen, warnt die App, dass das Speichern die externen Änderungen überschreibt.

## 5. Vorschau

- GitHub Flavored Markdown: Tabellen, Aufgabenlisten, Durchstreichen.
- **Mermaid-Diagramme** — ein Codeblock mit der Sprache `mermaid`.
- **KaTeX-Formeln** — `$…$` im Text und `$$…$$` als Block.
- **Bilder** aus dem Tresor: relativer Pfad `![](../attachments/2026/10/diagramm.png)` oder Einbettung `![[diagramm.png]]`. Es werden nur Dateien aus dem geöffneten Tresor angezeigt.
- Relative Links auf `.md`-Dateien öffnen die Notiz in der App; `https://…`-Links öffnen sich im Systembrowser.

## 6. Links

- `[[Titel der Notiz]]` verweist auf eine andere Notiz und ist in der Vorschau anklickbar.
- `[[Titel|Text]]` zeigt einen anderen Text, `[[Titel#Abschnitt]]` verweist auf einen Abschnitt.
- Gibt es keine Notiz mit diesem Namen, wird sie per Klick angelegt.
- Links werden über Pfad, Dateinamen oder die Eigenschaft `aliases` aufgelöst, ohne Groß-/Kleinschreibung zu beachten. Links innerhalb von Code werden ignoriert.
- Links in Eigenschaften (`project: "[[MD Notes]]"`) zählen ebenfalls.
- Die Leiste **„Verknüpfungen“** (⇆) zeigt, welche Notizen auf die geöffnete Notiz verweisen, samt Kontextzeile.

## 7. Suche, Tags und Schnellwechsler

- **Suche** (`Ctrl+Shift+F` / `Cmd+Shift+F`) durchsucht Text und Titel aller Notizen. Jedes Wort der Anfrage passt auf den Anfang eines Wortes. Treffer im Titel stehen weiter oben, gefundene Wörter werden hervorgehoben.
- **Tags** — alle Tags mit der Anzahl der Notizen. Die Auswahl eines Tags zeigt seine Notizen, einschließlich verschachtelter Tags (`#projekt` findet auch `#projekt/design`).
- **Schnellwechsler** (`Ctrl+O` / `Cmd+O`, auch `Ctrl+P`) — tippen Sie einen Teil von Titel, Alias oder Pfad. `↑`/`↓` wählen aus, `Enter` öffnet. Gibt es die Notiz nicht, legt `Enter` sie an.

## 8. Tags und Eigenschaften

- `#Tag` im Text oder eine `tags`-Liste in den Eigenschaften kennzeichnet ein Thema. Tags am Ende der Überschrift werden nicht Teil des Notiztitels.
- Eigenschaften stehen als YAML-Front-Matter am Anfang der Datei zwischen `---`-Zeilen.
- In der Vorschau sind die Eigenschaften im Block **„Eigenschaften“** eingeklappt.
- Enthält das YAML einen Fehler, zeigt die Statusleiste ihn an, und die Notiz öffnet sich als normaler Text.

## 9. Vorlagen und Tagesnotizen

- Vorlagen sind gewöhnliche `.md`-Dateien in `.mdnotes/templates/`. Die Typnamen für die Liste stammen aus `.mdnotes/schema.json`.
- Platzhalter: `{{title}}` — Titel, `{{date}}` — Datum `JJJJ-MM-TT`, `{{time}}` — Uhrzeit `HH:MM`, `{{id}}` — neue Kennung.
- **Heute** öffnet die Notiz `JJJJ-MM-TT.md` im Ordner für Tagesnotizen (`dailyNotesDir` in `.mdnotes/config.json`, standardmäßig `daily`). Fehlt sie, wird sie aus der Vorlage `daily` angelegt.

## 10. Designs und Sprachen

- Design: **System**, **Hell** oder **Dunkel**.
- Sprache der Oberfläche: 🇺🇦 Українська (Hauptsprache) · 🇬🇧 English · 🇫🇷 Français · 🇩🇪 Deutsch · 🇪🇸 Español · 🇰🇷 한국어 · 🇯🇵 日本語.
- Beide Einstellungen werden gespeichert. Das Fenster **„Über“** (ⓘ) zeigt Version, Rechteinhaber und die Liste der Sprachen.

## 11. Wo die Daten liegen

- Notizen sind gewöhnliche Dateien im gewählten Ordner und bleiben ohne MD Notes nutzbar.
- `.mdnotes/` ist der Dienstordner des Tresors: Einstellungen, Vorlagen, Papierkorb, Cache.
- `.mdnotes/cache/index.db` ist der Such- und Link-Index. Er ist nur ein Cache: Sie können ihn löschen, er wird neu aufgebaut. Cache und Papierkorb bleiben außerhalb von Git (`.mdnotes/.gitignore`).
- MD Notes sendet Ihre Daten nirgendwohin: keine Server, keine Analyse, keine Telemetrie.
- Sichern Sie den Tresor-Ordner. Git eignet sich gut zur Versionierung.

## 12. Tastatur

| Aktion | Windows / Linux | macOS |
|---|---|---|
| Speichern | `Ctrl+S` | `Cmd+S` |
| Schnellwechsler | `Ctrl+O` / `Ctrl+P` | `Cmd+O` / `Cmd+P` |
| Suche | `Ctrl+Shift+F` | `Cmd+Shift+F` |
| Rückgängig / Wiederholen | `Ctrl+Z` / `Ctrl+Shift+Z` | `Cmd+Z` / `Cmd+Shift+Z` |
| Einrücken | `Tab` | `Tab` |

## 13. Häufige Probleme

- **Windows zeigt SmartScreen** — der Build ist nicht signiert. Wählen Sie „Weitere Informationen → Trotzdem ausführen“.
- **macOS öffnet die App nicht** — der Build ist nicht notarisiert. Nutzen Sie **Systemeinstellungen → Datenschutz & Sicherheit → Dennoch öffnen**.
- **Der Tresor öffnet sich beim Start nicht** — der Ordner wurde umbenannt oder verschoben. Öffnen Sie ihn erneut über **„Anderer Tresor…“**.
- **Die Suche findet eine gerade geänderte Notiz nicht** — schließen und öffnen Sie den Tresor erneut. Hilft das nicht, löschen Sie `.mdnotes/cache/index.db`; der Index wird neu aufgebaut.
- **Ein Bild wird nicht angezeigt** — prüfen Sie, ob die Datei im Tresor liegt und der Pfad relativ ist.

## 14. Urheberrecht

Copyright © 2026 Roman Zavada (Роман Завада). Alle Rechte vorbehalten. MD Notes ist proprietäre Software. Ihre Notizen gehören Ihnen. Siehe [LICENSE.md](../../LICENSE.md) und [LEGAL_AND_COPYRIGHT.md](../LEGAL_AND_COPYRIGHT.md).
