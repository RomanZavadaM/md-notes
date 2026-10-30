# Backup und Wiederherstellung — Funktionsweise und sichere Vorgehensweise

[🇺🇦 Українська](BACKUP_AND_RECOVERY.uk.md) · [🇬🇧 English](BACKUP_AND_RECOVERY.en.md) · [🇫🇷 Français](BACKUP_AND_RECOVERY.fr.md) · **🇩🇪 Deutsch** · [🇪🇸 Español](BACKUP_AND_RECOVERY.es.md) · [🇰🇷 한국어](BACKUP_AND_RECOVERY.ko.md) · [🇯🇵 日本語](BACKUP_AND_RECOVERY.ja.md)

> Gilt für MD Notes v1.0.0. Ukrainisch ist kanonisch. Backup/Restore ist seit v0.3.22 implementiert; automatisierte Desktop-Tests existieren, ein separater vollständiger Owner-Runtime-Test wurde noch nicht dokumentiert.

## 1. Backup, Sync und Git sind verschieden

Ein **Backup** ist ein eigenständiger ZIP-Snapshot zur Wiederherstellung. **Sync** hält Kopien aktuell und kann auch Fehler/Löschungen verteilen. **Git** bietet Historie, enthält aber nicht automatisch alle lokalen Daten und ersetzt niemals die Sicherung von Schlüsseln eines encrypted vault.

Mindestens ein Backup sollte unabhängig vom Arbeitsordner und dessen Synchronisation liegen.

## 2. Erstellung

„Datei → Backup erstellen…“ versucht offene Dokumente zu speichern, liest die physischen Vault-Dateien, filtert nicht portable Daten, berechnet SHA-256, schreibt ZIP plus `manifest.json` und prüft vor Abschluss erneut, ob die Quelle unverändert blieb.

Ändert sich der Vault währenddessen, soll die Operation fehlschlagen statt einen Mischzustand zu speichern.

## 3. Enthalten

Markdown-Notizen, Anhänge, portable Config/Schema/Templates/Views, `.mdnotes/trash/` und gegebenenfalls Encryption-Konfiguration.

## 4. Ausgeschlossen

`.git/`, `.mdnotes/cache/`, MD-Notes-Temporärdateien, typische Systemdateien und private age-/Recovery-Schlüssel. Symbolische Links werden aus Sicherheitsgründen abgelehnt.

## 5. Manifest

`manifest.json` enthält Format/Version, App-Version, Zeitpunkt, Vault-Namen, encrypted-Flag sowie für jede Datei relativen Pfad, Größe und SHA-256.

Es dient **Integrität/Vollständigkeit**, nicht der Verschlüsselung, und ist offene Metadaten.

## 6. Grenzen v1

Max. 100.000 Dateien, 4 GiB pro Datei, 16 GiB insgesamt; keine absoluten Pfade, `..`, Traversal, Duplikate oder nur in Groß/Kleinschreibung kollidierende Pfade.

## 7. Prüfung vor Restore

MD Notes validiert Manifest, Pfade, ZIP-Einträge, Größen und SHA-256 vollständig. Fehlende oder unerwartete Dateien führen zum Abbruch, bevor ein Arbeits-Vault entsteht.

## 8. Ziel

Nach erfolgreicher Prüfung wird zunächst temporär entpackt und dann ein **neuer** Vault erzeugt. Vorhandene Ordner werden nicht still überschrieben: `My Vault-restored`, `My Vault-restored-2` usw.

## 9. Normaler Vault

Der ZIP enthält lesbare Markdown-Dateien und Anhänge und muss entsprechend geschützt werden.

## 10. Verschlüsselter Vault

Backup liest den physischen Zustand, daher bleiben age-Dateien Ciphertext. Offene Servicemetadaten bleiben offen; Cache und private Schlüssel werden ausgeschlossen.

Für vollständige Wiederherstellung braucht man:
1. Vault-Backup;
2. mindestens einen passenden privaten age-Schlüssel.

Siehe [Verschlüsselter Vault](ENCRYPTED_VAULT.de.md).

## 11. Empfohlenes Modell

Arbeitskopie + regelmäßige Backups auf anderem Medium/Ort + separat aufbewahrter Recovery Key. Für wichtige Daten ist 3-2-1 ein sinnvolles Ziel.

## 12. Backup-Ablauf

Änderungen sichern → Backup erstellen → ZIP außerhalb des Vault speichern → Erfolg prüfen → auf unabhängigen Speicher kopieren → bei encrypted vault Recovery Key separat prüfen.

## 13. Restore-Ablauf

Alten Vault nicht vorher löschen → Backup in neuen Ordner wiederherstellen → Manifest/Checksums prüfen lassen → `*-restored` öffnen → wichtige Notizen, Anhänge und Graph testen → ggf. Schlüssel importieren → erst danach alte Kopie behandeln.

## 14. Recovery Drill

Regelmäßig einen aktuellen Backup in einen separaten Ordner restaurieren und wirklich öffnen. Bei encrypted vault zusätzlich die Verfügbarkeit eines Recovery Keys prüfen.

## 15. Häufige Fehler

Backup auf demselben ausfallenden Datenträger; Ciphertext ohne Schlüssel; Schlüssel ohne aktuelle Datenkopie; Sync als Backup betrachten; normales ZIP öffentlich hochladen; alten Vault zu früh löschen; Backup während externer Massenänderungen.

## 16. Checksum-Fehler

Manifest nicht „reparieren“. Eine Abweichung bedeutet, dass das Archiv nicht mehr zum Snapshot passt. Archiv aufbewahren und eine andere Sicherung verwenden.

## 17. KI und Automatisierung

Eine KI darf die letzte bekannte gute Kopie nicht vor verifiziertem Restore löschen, Sync/Git nicht als gleichwertiges Backup behandeln, private Schlüssel nicht in ZIP legen, Validierungen nicht umgehen, nicht über einen bestehenden Vault entpacken und keinen Erfolg ohne Prüfung des restored vault behaupten.

Encrypted-Regeln: [ENCRYPTED_VAULT_CONTRACT.md](../ai/ENCRYPTED_VAULT_CONTRACT.md).

> Automatisch erzeugte, nicht muttersprachlich geprüfte Übersetzung; Ukrainisch ist kanonisch.

## Recovery-Prüfung für 1.0

Vor Desktop 1.0 wurde ein automatisierter End-to-End-Recovery-Drill für verschlüsselte Vaults ergänzt:

1. ein echter verschlüsselter Vault mit separatem Recovery-Empfänger wird erstellt;
2. eine Markdown-Notiz und ein binärer Anhang werden geschrieben/importiert;
3. ein portables ZIP-Backup wird erstellt;
4. das Backup wird in einen neuen Ordner wiederhergestellt;
5. der wiederhergestellte Vault wird mit dem **Recovery Private Key** geöffnet;
6. Notiz und Anhangsdaten werden über den verschlüsselten Storage Provider gelesen.

Dieses Szenario lief vor 1.0 in den Core-Gates unter Windows, macOS und Linux erfolgreich. Es ist ein automatisierter Recovery-Nachweis und ersetzt keinen separaten manuellen Owner-Test mit echten Daten.

