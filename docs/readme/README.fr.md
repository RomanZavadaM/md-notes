# MD Notes

[🇺🇦 Українська](../../README.md) · [🇬🇧 English](README.en.md) · **🇫🇷 Français** · [🇩🇪 Deutsch](README.de.md) · [🇪🇸 Español](README.es.md) · [🇰🇷 한국어](README.ko.md) · [🇯🇵 日本語](README.ja.md)

<!-- x-release-please-start-version -->
> **Checkpoint de version : MD Notes v1.0.0**
>
> **État : version desktop fonctionnellement terminée ; stable v1.0.0 publiée. Le projet passe en mode utilisation et stabilisation.**

## Télécharger MD Notes v1.0.0

**Installation rapide :**

- **Windows 64 bits :** [installateur .exe](https://github.com/RomanZavadaM/md-notes/releases/download/v1.0.0/MD.Notes_1.0.0_x64-setup.exe) · [MSI](https://github.com/RomanZavadaM/md-notes/releases/download/v1.0.0/MD.Notes_1.0.0_x64_en-US.msi) · [ZIP portable](https://github.com/RomanZavadaM/md-notes/releases/download/v1.0.0/MD-Notes-1.0.0-Windows-x64-portable.zip)
- **macOS Universal :** [télécharger le DMG](https://github.com/RomanZavadaM/md-notes/releases/download/v1.0.0/MD.Notes_1.0.0_universal.dmg)
- **Linux x86_64 :** [AppImage](https://github.com/RomanZavadaM/md-notes/releases/download/v1.0.0/MD.Notes_1.0.0_amd64.AppImage) · [DEB](https://github.com/RomanZavadaM/md-notes/releases/download/v1.0.0/MD.Notes_1.0.0_amd64.deb) · [RPM](https://github.com/RomanZavadaM/md-notes/releases/download/v1.0.0/MD.Notes-1.0.0-1.x86_64.rpm)

[**Tous les fichiers de la version v1.0.0 →**](https://github.com/RomanZavadaM/md-notes/releases/tag/v1.0.0)
<!-- x-release-please-end -->

> Les builds ne sont pas encore signés/notarisés ; le système peut afficher un avertissement de sécurité standard.

> **Packaging Windows 1.0 :** NSIS/MSI incluent l’installateur WebView2 hors ligne et le runtime VC ; le ZIP portable léger utilise le WebView2 Runtime déjà installé dans Windows.

## Présentation

**MD Notes** est une application multiplateforme local-first pour une base de connaissances Markdown. Elle utilise des fichiers `.md/.markdown`, des pièces jointes et des métadonnées JSON ouvertes, sans format propriétaire caché.

**Vos données vous appartiennent.** Les notes restent dans votre système de fichiers, peuvent être ouvertes avec n’importe quel éditeur et versionnées avec Git. MD Notes n’a ni serveur applicatif, ni analyse d’usage, ni télémétrie.

Plateformes desktop 1.0 : **Windows, macOS, Linux**. Android/iOS continuent à être compilés en CI, mais la validation sur appareils physiques et la publication dans les stores mobiles sont postérieures à la 1.0 desktop.

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
- physique de graphe vivante (graphe « flottant »), graphe dans une fenêtre séparée, arbre des relations noté par son maillon le plus faible, chemin entre deux notes ;
- recherche d’un sujet par code exact (EDRPOU/numéro fiscal) ou par nom, import de dossier avec fusion par code, doublons possibles ;
- interface en sept langues et thèmes clair/sombre/système.

### v0.3 — intégré jusqu’au checkpoint v0.3.18

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
- coffre chiffré optionnel (age) : les notes sont stockées sur le disque en fichiers `.age`, la clé privée reste uniquement dans le magasin de secrets du système de l’appareil (Windows Credential Manager, macOS Keychain, Linux Secret Service) ; des dialogues gèrent la création avec sauvegarde unique de la clé, l’import de la clé sur un autre appareil et une clé de récupération facultative ;
- Git push via le `git` du système (Git doit être installé) avec une garde pour les coffres chiffrés : aucun texte en clair n’est envoyé ; panneau Git dans l’application (état, fetch, pull, commit, push) .

Encore **non terminé** : intégration complète avec `SyncManifest` / ADR-0005, WebDAV, validation sur appareils Android/iOS physiques et dossiers externes optionnels Android SAF / iOS security-scoped.

Le **coffre chiffré** reste optionnel : les coffres ordinaires fonctionnent sans changement et une build sans la feature `encrypted-vault` ne contient aucun code de chiffrement. Dans la stabilisation 1.0, les images et PDF sont prévisualisés par déchiffrement uniquement en mémoire et un coffre existant peut recevoir un destinataire public de récupération depuis le menu Fichier. L’ouverture externe des pièces jointes chiffrées est désactivée afin d’éviter une copie en clair sur le disque ; l’index `.mdnotes/cache` reste non chiffré et le Git push conserve sa garde anti-texte-clair.

Plan complet : [roadmap](../roadmap.md) (ukrainien, référence).

> Les traductions FR / DE / ES / KO / JA de ce README, du guide et de l’interface sont automatiques et non relues par des locuteurs natifs ; le texte ukrainien fait foi. Tout ce qui précède est couvert par les tests automatiques et la build GitHub ; les vérifications manuelles dans l’application et sur de vrais appareils Android/iOS ne sont pas encore confirmées par le propriétaire.

## Installation

Téléchargez les paquets depuis la [page des versions](https://github.com/RomanZavadaM/md-notes/releases) : Windows (`.exe`, `.msi`, ZIP portable), macOS (`.dmg`, `.app.tar.gz`) et Linux (`.AppImage`, `.deb`, `.rpm`).

Le checkpoint **v0.3.18** inclut le Markdown autonome et le pipeline Git fast-forward sûr. Les builds restent non signés/non notarisés sauf indication contraire dans une version précise.

Guide complet : **[Guide de l’utilisateur](../user-guide/USER_GUIDE.fr.md)**.

## Développement, confidentialité et mentions légales

Rust stable, Node.js 20+ et les prérequis Tauri sont nécessaires. Les gates incluent fmt/clippy/tests Rust, build frontend, contrôle des licences, CI desktop et mobile smoke Android/iOS pour les changements concernés.

MD Notes est local-first. Les secrets de synchronisation ne doivent jamais être stockés dans le vault, les fichiers Markdown, les URL distantes, Git config, les logs ou `localStorage`. Les conflits ne doivent jamais être écrasés silencieusement et l’application Git automatique n’est autorisée que pour un fast-forward propre.

Copyright © 2026 Roman Zavada (Роман Завада). Tous droits réservés. MD Notes est un logiciel propriétaire ; le dépôt public n’accorde aucune licence open source. Vos notes vous appartiennent. Voir [LICENSE.md](../../LICENSE.md) et les [mentions légales](../LEGAL_AND_COPYRIGHT.md).


## Éditeur multi-fenêtres (`main` actuel)

**v0.3.21** publie les étapes M1–M3 d’ADR-0010. Plusieurs notes du coffre peuvent rester ouvertes dans des onglets, chacune avec son propre brouillon. Sur ordinateur, la note active peut être déplacée avec **Ouvrir dans une fenêtre séparée** vers une fenêtre d’édition indépendante. Les modifications sont enregistrées avant le déplacement, puis la fenêtre principale revient à l’onglet précédent ou à un espace vide.

M1–M3 font partie de la version publiée **v0.3.21** : protection contre les conflits, fermeture sûre des éditeurs détachés et sauvegarde globale/arrêt sûr. La CI Windows/macOS/Linux et les builds de contrôle Android/iOS passent. **M4 désigne désormais la validation manuelle en exécution** ; la prerelease de test est déjà publiée.


## Sauvegarde et restauration

Le menu **Fichier** propose **Créer une sauvegarde…** et **Restaurer une sauvegarde…**.

- la sauvegarde est une archive ZIP portable avec `manifest.json` et un SHA-256 pour chaque fichier ;
- MD Notes enregistre les notes ouvertes avant la sauvegarde ;
- notes, pièces jointes, configuration, modèles et corbeille sont inclus ; `.git`, `.mdnotes/cache` et les fichiers temporaires du système sont exclus ;
- un coffre chiffré est copié **sans déchiffrement** ; la clé privée/de récupération n’est jamais ajoutée à l’archive ;
- la restauration vérifie d’abord les chemins, tailles et sommes de contrôle, puis crée un **nouveau** dossier de coffre sans écraser l’existant.

## Principes et procédures détaillés

- [Liens et graphe de connaissances](../concepts/KNOWLEDGE_GRAPH.fr.md)
- [Coffre chiffré](../security/ENCRYPTED_VAULT.fr.md)
- [Sauvegarde et récupération](../security/BACKUP_AND_RECOVERY.fr.md)
- [Contrats canoniques pour IA/automatisation](../ai/README.md)

