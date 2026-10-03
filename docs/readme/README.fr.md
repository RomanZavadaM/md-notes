# MD Notes

[🇺🇦 Українська](../../README.md) · [🇬🇧 English](README.en.md) · **🇫🇷 Français** · [🇩🇪 Deutsch](README.de.md) · [🇪🇸 Español](README.es.md) · [🇰🇷 한국어](README.ko.md) · [🇯🇵 日本語](README.ja.md)

> **Dernier checkpoint publié : [MD Notes v0.2.2](https://github.com/RomanZavadaM/md-notes/releases/tag/v0.2.2)**
>
> **État du développement : ACTIVE — étape roadmap v0.3 « plateformes mobiles et synchronisation ».**

## Présentation

**MD Notes** est une application multiplateforme local-first pour une base de connaissances en Markdown. Elle travaille avec des fichiers `.md` ordinaires et leurs pièces jointes, sans format de données propriétaire caché.

**Vos données vous appartiennent.** Les notes peuvent être ouvertes dans n’importe quel éditeur, stockées dans votre propre système de fichiers et versionnées avec Git. MD Notes n’a ni serveur applicatif, ni analyse d’usage, ni télémétrie.

Plateformes : **Windows, macOS, Linux**. Android et iOS sont en développement actif dans la v0.3 ; la CI valide leur compilation, mais le runtime sur appareils physiques n’est pas encore déclaré validé.

## Ce qui est déjà disponible

### v0.2 — base fonctionnelle terminée

- coffre local, arborescence, création/renommage/corbeille ;
- CodeMirror 6, aperçu/split, autosave et écritures atomiques ;
- index SQLite/FTS5, recherche, tags et ouverture rapide ;
- liens wiki, aliases, backlinks et réécriture des liens lors des déplacements/renommages ;
- Mermaid 11, KaTeX, images, pièces jointes, modèles et notes quotidiennes ;
- propriétés pilotées par le schéma ouvert `.mdnotes/schema.json` ;
- presets Empty / PARA / Zettelkasten ;
- graphe de connaissances global/local ;
- interface en sept langues et thèmes clair/sombre/système.

### v0.3 — développement actif

Déjà intégré dans `main` :

- `StorageProvider` / `VaultStorage` indépendants du backend ;
- coffre sandbox mobile pour Android/iOS ;
- build smoke Android + iOS dans la CI ;
- fondation de décision de synchronisation local-first et état de sync persistant ;
- fondation Git HTTPS via `gix`/gitoxide ;
- détection du worktree modifié ;
- pipeline de commit Git local sans dépendre de la configuration Git utilisateur/système ;
- fetch HTTPS public sûr ;
- authentification HTTPS en mémoire sans persister le token dans Git config, le coffre ou l’URL distante.

Le checkpoint de sécurité actuel ajoute le **stockage système des identifiants Git** dans la couche Tauri : Windows Credential Manager, macOS Keychain, iOS Protected Data, stockage Android adossé au Keystore et Linux Secret Service. Le frontend peut enregistrer/vérifier/supprimer les identifiants, mais ne peut pas relire le token ; le fetch authentifié lit le secret uniquement dans Rust juste avant l’appel réseau.

Encore **non terminé** : politique pull/merge Git, push, intégration complète avec la politique de conflits, WebDAV, validation runtime sur appareils Android/iOS physiques et dossiers externes optionnels Android SAF / iOS security-scoped.

Plan complet : [roadmap](../roadmap.md) (ukrainien, référence).

## Installation

Téléchargez les paquets depuis la [page des versions](https://github.com/RomanZavadaM/md-notes/releases) : Windows (`.exe`, `.msi`, ZIP portable), macOS (`.dmg`, `.app.tar.gz`) et Linux (`.AppImage`, `.deb`, `.rpm`).

Le dernier prerelease publié est **v0.2.2**. Les builds ne sont pas encore signés/notarisés ; le système peut afficher des avertissements de sécurité standards.

Guide complet : **[Guide de l’utilisateur](../user-guide/USER_GUIDE.fr.md)**.

## Développement, confidentialité et mentions légales

Rust stable, Node.js 20+ et les prérequis système Tauri sont nécessaires. Les gates d’intégration incluent format/clippy/tests Rust, build frontend, contrôle des licences, CI desktop et mobile smoke Android/iOS lorsque la frontière mobile/Tauri change.

MD Notes est local-first. Les secrets de synchronisation ne doivent jamais être stockés dans le coffre, les fichiers Markdown, les URL distantes, Git config, les logs ou `localStorage` ; ils appartiennent au gestionnaire d’identifiants du système d’exploitation. Les conflits ne doivent jamais être écrasés silencieusement.

Copyright © 2026 Roman Zavada (Роман Завада). Tous droits réservés. MD Notes est un logiciel propriétaire ; le dépôt public n’accorde aucune licence open source. Vos notes vous appartiennent. Voir [LICENSE.md](../../LICENSE.md) et les [mentions légales](../LEGAL_AND_COPYRIGHT.md).
