# MD Notes

[🇺🇦 Українська](../../README.md) · [🇬🇧 English](README.en.md) · **🇫🇷 Français** · [🇩🇪 Deutsch](README.de.md) · [🇪🇸 Español](README.es.md) · [🇰🇷 한국어](README.ko.md) · [🇯🇵 日本語](README.ja.md)

> **Checkpoint de version : MD Notes v0.3.2**
>
> **État du développement : ACTIVE — étape roadmap v0.3 « plateformes mobiles et synchronisation ».**

## Télécharger MD Notes v0.3.2

**Installation rapide :**

- **Windows 64 bits :** [installateur .exe](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.2/MD.Notes_0.3.2_x64-setup.exe) · [MSI](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.2/MD.Notes_0.3.2_x64_en-US.msi) · [ZIP portable](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.2/MD-Notes-0.3.2-Windows-x64-portable.zip)
- **macOS Universal :** [télécharger le DMG](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.2/MD.Notes_0.3.2_universal.dmg)
- **Linux x86_64 :** [AppImage](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.2/MD.Notes_0.3.2_amd64.AppImage) · [DEB](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.2/MD.Notes_0.3.2_amd64.deb) · [RPM](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.2/MD.Notes-0.3.2-1.x86_64.rpm)

[**Tous les fichiers de la version v0.3.2 →**](https://github.com/RomanZavadaM/md-notes/releases/tag/v0.3.2)

> Les builds ne sont pas encore signés/notarisés ; le système peut afficher un avertissement de sécurité standard.

## Présentation

**MD Notes** est une application multiplateforme local-first pour une base de connaissances Markdown. Elle utilise des fichiers `.md/.markdown`, des pièces jointes et des métadonnées JSON ouvertes, sans format propriétaire caché.

**Vos données vous appartiennent.** Les notes restent dans votre système de fichiers, peuvent être ouvertes avec n’importe quel éditeur et versionnées avec Git. MD Notes n’a ni serveur applicatif, ni analyse d’usage, ni télémétrie.

Plateformes : **Windows, macOS, Linux**. Android et iOS sont des cibles actives de la v0.3 ; la CI valide la compilation, mais le runtime sur appareils physiques n’est pas encore déclaré validé.

## Fonctionnalités intégrées

### v0.2 — base fonctionnelle terminée

- coffre local, arborescence, création/renommage/corbeille ;
- CodeMirror 6, aperçu/split, autosave et écritures atomiques ;
- index SQLite/FTS5, recherche, tags et ouverture rapide ;
- liens wiki, aliases, backlinks et réécriture lors des déplacements/renommages ;
- Mermaid 11, KaTeX, images, pièces jointes, modèles et notes quotidiennes ;
- propriétés pilotées par `.mdnotes/schema.json` ;
- presets Empty / PARA / Zettelkasten ;
- graphe de connaissances global/local ;
- interface en sept langues et thèmes clair/sombre/système.

### v0.3 — intégré jusqu’au checkpoint v0.3.2

- `StorageProvider` / `VaultStorage` indépendants du backend ;
- coffre sandbox mobile Android/iOS ;
- build smoke Android + iOS dans la CI ;
- fondation local-first de synchronisation et état persistant ;
- fondation Git HTTPS avec `gix`/gitoxide ;
- détection du worktree modifié et commits Git locaux ;
- fetch HTTPS public sûr ;
- authentification HTTPS en mémoire sans persistance du token ;
- stockage système des identifiants Git sur Windows, macOS, iOS, Android et Linux ;
- ouverture/édition/enregistrement d’un fichier `.md/.markdown` sans vault ;
- intégration Markdown Windows/Linux/macOS, sans changer l’application par défaut sans action explicite de l’utilisateur ;
- planner Git pull : `UpToDate / FastForward / LocalAhead / Diverged / Unrelated` ;
- preflight bloquant tout worktree modifié ;
- fast-forward sûr uniquement pour un `FastForward` propre, sans `hard reset` aveugle, commit de merge ni écrasement silencieux;
- coffre chiffré optionnel (age) : les notes sont stockées sur le disque en fichiers `.age`, la clé privée reste uniquement dans le magasin de secrets du système de l’appareil (Windows Credential Manager, macOS Keychain, Linux Secret Service) ; des dialogues gèrent la création avec sauvegarde unique de la clé, l’import de la clé sur un autre appareil et une clé de récupération facultative .

Encore **non terminé** : Git push, intégration complète avec `SyncManifest` / ADR-0005, WebDAV, validation sur appareils Android/iOS physiques et dossiers externes optionnels Android SAF / iOS security-scoped.

Le **coffre chiffré** est une option, pas une obligation : les coffres ordinaires fonctionnent sans changement, et une build sans la feature `encrypted-vault` ne contient aucun code de chiffrement. Pour l’instant, un coffre chiffré n’affiche pas les images ni les pièces jointes, il n’existe pas d’interface pour ajouter une clé de récupération à un coffre existant, et l’index `.mdnotes/cache` reste non chiffré ; le Git push depuis un tel coffre arrivera avec une garde qui refuse d’envoyer du texte en clair.

Plan complet : [roadmap](../roadmap.md) (ukrainien, référence).

## Installation

Téléchargez les paquets depuis la [page des versions](https://github.com/RomanZavadaM/md-notes/releases) : Windows (`.exe`, `.msi`, ZIP portable), macOS (`.dmg`, `.app.tar.gz`) et Linux (`.AppImage`, `.deb`, `.rpm`).

Le checkpoint **v0.3.2** inclut le Markdown autonome et le pipeline Git fast-forward sûr. Les builds restent non signés/non notarisés sauf indication contraire dans une version précise.

Guide complet : **[Guide de l’utilisateur](../user-guide/USER_GUIDE.fr.md)**.

## Développement, confidentialité et mentions légales

Rust stable, Node.js 20+ et les prérequis Tauri sont nécessaires. Les gates incluent fmt/clippy/tests Rust, build frontend, contrôle des licences, CI desktop et mobile smoke Android/iOS pour les changements concernés.

MD Notes est local-first. Les secrets de synchronisation ne doivent jamais être stockés dans le vault, les fichiers Markdown, les URL distantes, Git config, les logs ou `localStorage`. Les conflits ne doivent jamais être écrasés silencieusement et l’application Git automatique n’est autorisée que pour un fast-forward propre.

Copyright © 2026 Roman Zavada (Роман Завада). Tous droits réservés. MD Notes est un logiciel propriétaire ; le dépôt public n’accorde aucune licence open source. Vos notes vous appartiennent. Voir [LICENSE.md](../../LICENSE.md) et les [mentions légales](../LEGAL_AND_COPYRIGHT.md).
