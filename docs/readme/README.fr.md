# MD Notes

[🇺🇦 Українська](../../README.md) · [🇬🇧 English](README.en.md) · **🇫🇷 Français** · [🇩🇪 Deutsch](README.de.md) · [🇪🇸 Español](README.es.md) · [🇰🇷 한국어](README.ko.md) · [🇯🇵 日本語](README.ja.md)

> **Checkpoint actuel : [MD Notes v0.1.0](https://github.com/RomanZavadaM/md-notes/releases/tag/v0.1.0)**
>
> **État du développement : ACTIVE — étape v0.2 « structure et liens ».**

## Présentation

**MD Notes** est une application multiplateforme et local-first pour une base de connaissances personnelle en Markdown : consulter, modifier, structurer et visualiser de simples fichiers `.md` avec leurs pièces jointes.

**Vos données vous appartiennent.** Les notes sont des fichiers texte ouverts. Vous pouvez les ouvrir dans n'importe quel éditeur, les consulter sur GitHub et les versionner avec Git. L'application ne crée jamais de formats cachés.

Plateformes : **Windows, macOS, Linux** ; Android et iOS sont prévus pour la v0.3.

## Nouveautés de la v0.1.0

- un dossier local comme coffre (vault) ; arborescence de fichiers avec création, renommage et corbeille ;
- éditeur CodeMirror 6, aperçu, mode côte à côte, enregistrement automatique, écriture atomique ;
- `[[liens wiki]]` qui ouvrent la note cible ou la créent si elle n'existe pas ;
- `#étiquettes` et propriétés YAML front matter ;
- thèmes clair, sombre et système ; mise en page pour écrans étroits ;
- la base d'exemple `sample-vault/`.

En cours (v0.2) : index et recherche plein texte, rétroliens, mise à jour des liens lors d'un renommage, Mermaid et KaTeX, modèles et notes quotidiennes, interface en sept langues. Plan complet : [roadmap](../roadmap.md) (en ukrainien).

## Installation

Téléchargez le paquet de votre système depuis la [page de publication](https://github.com/RomanZavadaM/md-notes/releases) :

- **Windows** — `MD.Notes_<version>_x64-setup.exe` ou `.msi`. La version n'est pas signée ; Windows peut afficher SmartScreen.
- **macOS** — `.dmg` / `.app.tar.gz` (universel). La version n'est pas notarisée ; il peut être nécessaire d'utiliser **Réglages Système → Confidentialité et sécurité → Ouvrir quand même**.
- **Linux** — `.AppImage`, `.deb` ou `.rpm`.

Au démarrage, cliquez sur **« Ouvrir un dossier »** et choisissez un dossier de notes ou `sample-vault/` de ce dépôt.

Guide complet : **[Guide de l'utilisateur](../user-guide/USER_GUIDE.fr.md)**.

## Pour les développeurs

Prérequis : [Rust](https://rustup.rs/) (stable), [Node.js](https://nodejs.org/) 20+ et les [dépendances système de Tauri](https://v2.tauri.app/start/prerequisites/). Lancez `npm ci` puis `npm run tauri dev` dans `app/`. Règles de développement : [PROJECT_RULES.md](../../PROJECT_RULES.md) (en ukrainien, version de référence).

## Confidentialité et mentions légales

MD Notes est local-first : les notes, les pièces jointes et l'index restent sur votre appareil ou dans le stockage que vous choisissez. Aucun serveur, aucune analyse d'usage, aucune télémétrie. Les conflits de synchronisation ne sont jamais écrasés en silence. Conservez des sauvegardes de vos coffres.

Copyright © 2026 Roman Zavada (Роман Завада). Tous droits réservés. MD Notes est un logiciel propriétaire ; le dépôt public n'accorde aucune licence open source. Vos notes vous appartiennent. Voir [LICENSE.md](../../LICENSE.md) et les [mentions légales](../LEGAL_AND_COPYRIGHT.md).
