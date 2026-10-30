# MD Notes — Benutzerhandbuch

[🇺🇦 Українська](USER_GUIDE.uk.md) · [🇬🇧 English](USER_GUIDE.en.md) · [🇫🇷 Français](USER_GUIDE.fr.md) · **🇩🇪 Deutsch** · [🇪🇸 Español](USER_GUIDE.es.md) · [🇰🇷 한국어](USER_GUIDE.ko.md) · [🇯🇵 日本語](USER_GUIDE.ja.md)

Gilt für **v1.0.0**. Maßgeblich ist die ukrainische Fassung.

## 1. Erster Start

1. Installieren Sie MD Notes für Ihr Betriebssystem (siehe README). Unter Windows enthalten die NSIS/MSI-Installer von v1.0 Offline-WebView2 und die VC-Runtime; das Portable-ZIP verwendet die bereits in Windows vorhandene WebView2 Runtime.
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
- **Mehrfachauswahl:** `Ctrl` (Windows/Linux) bzw. `Cmd` (macOS) fügt ein Element zur Auswahl hinzu oder entfernt es daraus; `Shift` wählt einen sichtbaren Bereich im Baum. Ein normaler Klick kehrt zur Einzelauswahl und zum gewohnten Öffnen zurück.
- Sind mehrere Elemente ausgewählt, verschiebt **🗑** sie gemeinsam in den Papierkorb. Sind ein Ordner und zugleich ein darin enthaltenes Element ausgewählt, wird das untergeordnete Element nicht doppelt verarbeitet. Umbenennen bleibt vorerst auf ein einzelnes ausgewähltes Element beschränkt.

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

### Verschlüsselter Tresor (optional)

- Aktivieren Sie beim Anlegen des Tresors **„Tresor verschlüsseln“**. Notizen liegen als `.age`-Dateien auf der Festplatte und sind ohne Schlüssel nicht lesbar.
- Nach dem Anlegen wird der private Schlüssel (`AGE-SECRET-KEY-…`) **einmalig** angezeigt. Bewahren Sie ihn in einem Passwortmanager oder offline auf: Er lässt sich nicht wiederherstellen. Dauerhaft liegt der Schlüssel nur im System-Geheimnisspeicher dieses Geräts.
- Sie können einen **öffentlichen Wiederherstellungsschlüssel** (`age1…`) angeben: Er öffnet den Tresor ebenfalls, falls der Hauptschlüssel verloren geht.
- Auf einem anderen Computer kopieren Sie den Tresorordner und öffnen ihn: Die App fragt nach dem Schlüssel. Fügen Sie den privaten Schlüssel ein; er wird dann im System-Geheimnisspeicher dieses Computers abgelegt.
- In der 1.0-Stabilisierung werden Bilder und PDFs eines verschlüsselten Tresors angezeigt, ohne eine Klartextkopie auf den Datenträger zu schreiben: Die Daten werden nur im Speicher entschlüsselt. Für einen bestehenden Vault kann über **Datei → „Wiederherstellungsschlüssel verwalten…“** ein weiterer öffentlicher `age1…`-Empfänger hinzugefügt werden. Externes „Datei öffnen“ ist für verschlüsselte Anhänge deaktiviert; der Index `.mdnotes/cache` bleibt unverschlüsselt.

### Git (nur Desktop)

Die Schaltfläche **Git** (oder „Git…“ im Menü) öffnet ein Panel: Repository anlegen, Remote-Adresse festlegen, Zugang speichern (das Token liegt nur im System-Geheimnisspeicher) sowie **Fetch**, **Pull**, **Commit** und **Push**. Pull läuft nur, wenn es keine ungespeicherten Änderungen gibt und das Remote einfach Neues hat; bei auseinandergelaufenen Verläufen ist eine manuelle Entscheidung nötig. Für Push muss Git auf dem Computer installiert sein. In einem verschlüsselten Tresor prüft Push zuvor, dass alle Dateien verschlüsselt sind, und wird sonst blockiert.

## 12. Graph, Beziehungsbaum und Subjektsuche

### Graph

Das Panel **„Verknüpfungen“** hat die Reiter „Verknüpfungen“, „Lokaler Graph“ und „Globaler Graph“. Ein Knoten ist eine Notiz, seine Farbe der `type` der Notiz, eine Linie ein `[[…]]`-Link oder eine typisierte Beziehung. Ein Klick auf einen Knoten öffnet die Notiz.

- **Lebendige Physik („schwebender“ Graph).** Ziehen Sie einen Knoten: Die Nachbarn folgen ihm und weichen ihm aus, nach dem Loslassen beruhigt sich der Graph sanft. Die Schaltfläche **„Schweben“** fügt eine leichte Drift hinzu (bis 400 Knoten standardmäßig an), **„Pause“** stoppt die Bewegung. Unter **„Kräfte“** gibt es Regler für Zentrum, Abstoßung, Linkstärke und Linklänge; die Auswahl wird gespeichert.
- **„Formen“ und „Legende“.** Die Schaltfläche **„Formen“** schaltet einen Modus ein, in dem eine **Form** den Notiztyp zeigt (Person — Kopf und Schultern, Firma — Quadrat, Institution — Sechseck, Fall — Raute, Material — Dreieck, Gruppe — drei Punkte) und die **Knotenfarbe** die **Verlässlichkeit** der besten Bewertung seiner Beziehungen (grün 1–2, bernstein 3–4, rot 5–6, grau — ohne Bewertung). Öffnen Sie **„Legende“**, um alle Formen, Farben und Linienstile zu sehen; ein gestrichelter Rand und das Zeichen „Ч“ bedeuten ein Gerücht, Farbe ist nie das einzige Signal.
- **Filter „Beziehungen zeigen“.** Die Liste im Graph-Bereich zeigt nur Beziehungen mit mindestens der gewählten Bewertung („verlässlich“ — 1–2, „verlässlich und mittel“ — 1–4); unbewertete Beziehungen und einfache Links werden ausgeblendet, die Knoten bleiben.
- **Typisierte Beziehungen** stehen in der Eigenschaft `relations` (`to`, dazu `type`, `grade`, `origin`, `status`, `evidence` und `dir: in` für die Gegenrichtung). Ein `grade` besteht aus Buchstabe und Ziffer (A1 am verlässlichsten, F6 am schwächsten): Der Buchstabe bewertet die Quelle, die Ziffer die Aussage. Linie und Knotenrand folgen der Bewertung; Bewertungen 5–6 tragen das Wort **„GERÜCHT“**. Farbe ist nie das einzige Signal.
- **„Woher das kommt“** hebt die Eigentümerkette oberhalb des gewählten Knotens und die Materialien darum hervor.
- **Eigenes Fenster.** Die Schaltfläche **↗** in den Graph-Reitern oder „Datei → Graph in eigenem Fenster“ (nur Desktop) öffnet den Graphen in einem eigenen Fenster mit mehr Platz, etwa auf einem zweiten Monitor. Es folgt der aktiven Notiz, aktualisiert sich nach Änderungen, und ein Klick auf einen Knoten öffnet die Notiz im Hauptfenster.
- **Schaltfläche „Zurück“.** Im Graphfenster führt sie zur vorherigen Notiz zurück.

### Beziehungsbaum

Der Reiter **„Beziehungsbaum“** zeigt die Kette typisierter Beziehungen ab der gewählten Notiz: **„Eigentümer und Quellen“** (aufwärts) oder **„Was sie besitzt“** (abwärts), Tiefe 1–8.

- Jeder Knoten zeigt die Beziehungsart und die Linienbewertung; daneben die **„Ketten“**-Bewertung: das schwächste Glied auf dem Weg von der Wurzel (B2 zusammen mit C1 ergibt C2). Eine Kette ist so verlässlich wie ihr schwächstes Glied.
- **„−/+“** klappt einen Zweig, **◎** baut den Baum ab einem anderen Knoten neu, **„Alles aufklappen“ / „Alles zuklappen“** steuern alle Zweige. Ein wiederholter Knoten wird markiert und nicht zweimal entfaltet; der Baum ist auf 400 Knoten begrenzt.
- **„Pfad zwischen zwei Notizen“** findet die kürzeste Kette typisierter Beziehungen (Linienrichtung egal) und zeigt ihr schwächstes Glied. Das Kontrollkästchen **„Verlässlichster statt kürzester“** sucht die Kette, deren schwächstes Glied am besten ist (bei Gleichstand die mit den wenigsten Schritten).
- Der Baum nutzt nur ausdrückliche typisierte Beziehungen, keine einfachen Links. Die Kettenbewertung ist eine Hilfe, keine rechtliche Schlussfolgerung.

### Subjekt nach Code oder Name suchen

Im Reiter **„Suche“** wird eine Zahl mit 8 Ziffern als EDRPOU-Code, mit 10 Ziffern als Steuernummer gesucht (ein Code mit verlorenen führenden Nullen, 6–7 Ziffern, wird ergänzt). Die Ergebnisse stehen getrennt: Bei einem 8-stelligen Code erscheint darunter der Block **„Datenbank „З-під Ляди“ nach Code“**: Wählen Sie den Ordner mit den offenen Daten der Seite (lokale Kopie des Zweigs data); er zeigt die Fakten zum genauen Code — Registrierungsdatum, Status und ausländische Länder der Gründer und wirtschaftlich Berechtigten (Register), Entscheidungen der Wettbewerbsbehörde, Prozorro-Vergaben als Auftraggeber und Gewinner. Nur lesend, nichts wird über das Netzwerk gesendet; es sind Fakten, keine Bewertungen.

- **„Exakter Treffer“**: eine Notiz mit demselben Code in der Eigenschaft `edrpou` oder `ipn` (oder demselben Namen nach Normalisierung);
- **„Kandidaten nach Name“**: ähnliche Namen; ein Namenstreffer ist **kein Beweis**, bestätigen Sie ihn mit Code oder Dokument;
- **„Im Text“**: gewöhnliche Volltextsuche.

Namen werden ohne Rücksicht auf Groß-/Kleinschreibung, Anführungszeichen, Bindestriche und Rechtsformen (GmbH u. Ä.) verglichen; lateinische Buchstaben, die kyrillischen gleichen, werden auf Kyrillisch abgebildet.

### Ordnerimport, Zusammenführen nach Code und mögliche Duplikate

- **„Datei → Ordner importieren…“** fügt Notizen aus einem Ordner hinzu. Eine Notiz, deren Code (`edrpou`/`ipn`) schon im Tresor steht, wird mit der vorhandenen **zusammengeführt**, statt dupliziert zu werden: fehlende Eigenschaften werden ergänzt, Tags, Aliase und Beziehungen vereint, der Titel der importierten Notiz wird zum Alias, abweichende Werte werden nicht überschrieben, sondern zur Prüfung aufgelistet. Erneuter Import ändert nichts.
- **„Datei → Mögliche Duplikate…“** listet Gruppen von Notizen mit gleichem normalisiertem Titel oder Alias. Das ist nur ein Hinweis: Nichts wird geändert, und Notizen mit verschiedenen Codes werden nie gruppiert.

## 13. Tastatur

| Aktion | Windows / Linux | macOS |
|---|---|---|
| Speichern | `Ctrl+S` | `Cmd+S` |
| Schnellwechsler | `Ctrl+O` / `Ctrl+P` | `Cmd+O` / `Cmd+P` |
| Suche | `Ctrl+Shift+F` | `Cmd+Shift+F` |
| Rückgängig / Wiederholen | `Ctrl+Z` / `Ctrl+Shift+Z` | `Cmd+Z` / `Cmd+Shift+Z` |
| Einrücken | `Tab` | `Tab` |

## 14. Häufige Probleme

- **Windows zeigt SmartScreen** — der Build ist nicht signiert. Wählen Sie „Weitere Informationen → Trotzdem ausführen“.
- **macOS öffnet die App nicht** — der Build ist nicht notarisiert. Nutzen Sie **Systemeinstellungen → Datenschutz & Sicherheit → Dennoch öffnen**.
- **Der Tresor öffnet sich beim Start nicht** — der Ordner wurde umbenannt oder verschoben. Öffnen Sie ihn erneut über **„Anderer Tresor…“**.
- **Die Suche findet eine gerade geänderte Notiz nicht** — schließen und öffnen Sie den Tresor erneut. Hilft das nicht, löschen Sie `.mdnotes/cache/index.db`; der Index wird neu aufgebaut.
- **Ein Bild wird nicht angezeigt** — prüfen Sie, ob die Datei im Tresor liegt und der Pfad relativ ist.

## 15. Urheberrecht

Copyright © 2026 Roman Zavada (Роман Завада). Alle Rechte vorbehalten. MD Notes ist proprietäre Software. Ihre Notizen gehören Ihnen. Siehe [LICENSE.md](../../LICENSE.md) und [LEGAL_AND_COPYRIGHT.md](../LEGAL_AND_COPYRIGHT.md).


## Tabs und separate Editorfenster

In **v0.3.21** können mehrere Vault-Notizen gleichzeitig in Tabs geöffnet bleiben. Jeder Tab behält seinen eigenen Entwurf, sodass beim Wechseln kein ungespeicherter Text verloren gehen soll.

Auf dem Desktop bietet die aktive Vault-Notiz **In separatem Fenster öffnen**. MD Notes speichert den aktuellen Entwurf vor dem Abtrennen. Nach erfolgreichem Erstellen des unabhängigen Fensters wird die Notiz aus den Tabs des Haupteditors entfernt; das Hauptfenster kehrt zum vorherigen Tab zurück oder zeigt, wenn keiner vorhanden ist, einen leeren Editor bei weiterhin verfügbarem Vault-Baum. Im separaten Fenster funktionieren Editor / Nebeneinander / Vorschau sowie Autosave.

Eine externe `.md/.markdown`-Datei bleibt ein separates Dokument und wird nicht automatisch in den Vault importiert. M3 ist jetzt integriert: Ändert ein anderes Fenster oder ein anderer Prozess die Datei nach dem Öffnen, blockiert MD Notes stilles Überschreiben; ein geändertes separates Fenster bleibt bis zu einem erfolgreichen geschützten Speichern geöffnet; vollständiges Beenden verarbeitet zuerst alle offenen Dokumente sicher. Diese Funktionalität ist in der stabilen **v1.0.0** enthalten; die praktische Windows-Abnahme des Mehrfenster-Workflows wurde vor 1.0 abgeschlossen, während die CI weiterhin Desktop- und Mobile-Build-Smoke-Tests abdeckt.


## Tresor-Sicherung

Im Menü **Datei** gibt es **Sicherung erstellen…**. Vor der Sicherung versucht MD Notes, alle geöffneten Tabs zu speichern. Das Ergebnis ist ein portables ZIP mit `manifest.json`; für jede Datei werden Pfad, Größe und SHA-256 festgehalten.

Die Sicherung enthält Notizen, Anhänge, Dienstkonfiguration, Vorlagen und `.mdnotes/trash`. `.git`, `.mdnotes/cache` und temporäre Systemdateien werden ausgeschlossen. Bei einem verschlüsselten Tresor werden die verschlüsselten Dateien direkt vom Datenträger kopiert; private/Wiederherstellungsschlüssel werden nie in das ZIP aufgenommen.

**Sicherung wiederherstellen…** prüft Manifest, Pfade, Größen und SHA-256 aller Dateien, bevor etwas geschrieben wird. Erst nach vollständiger Prüfung wird in einen temporären Ordner entpackt und danach ein neuer Tresorordner erstellt. Bestehende Ordner werden niemals überschrieben.
