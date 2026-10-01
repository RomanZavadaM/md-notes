# MD Notes — Guide de l'utilisateur

[🇺🇦 Українська](USER_GUIDE.uk.md) · [🇬🇧 English](USER_GUIDE.en.md) · **🇫🇷 Français** · [🇩🇪 Deutsch](USER_GUIDE.de.md) · [🇪🇸 Español](USER_GUIDE.es.md) · [🇰🇷 한국어](USER_GUIDE.ko.md) · [🇯🇵 日本語](USER_GUIDE.ja.md)

S'applique à la **v0.1.0**. La version ukrainienne fait foi.

## 1. Premier lancement

1. Installez MD Notes pour votre système (voir le README).
2. Cliquez sur **« Ouvrir un dossier »** et choisissez un dossier de notes. Tout dossier contenant des fichiers `.md` convient, y compris `sample-vault/` du dépôt.
3. MD Notes mémorise le dossier et l'ouvre au lancement suivant.

Ouvrir un dossier ne le modifie pas. Le dossier de service `.mdnotes/` n'est créé que lorsqu'il est réellement nécessaire (corbeille, réglages, index).

## 2. La fenêtre

- **Barre d'outils** — bouton du panneau latéral ☰, nom du coffre, titre de la note ouverte (un point signale des modifications non enregistrées), modes « Éditeur » / « Côte à côte » / « Aperçu » et choix du thème.
- **Panneau latéral** — l'arborescence des fichiers du coffre. Les dossiers apparaissent en premier ; les dossiers cachés (`.mdnotes`, `.git`) ne sont pas affichés. Les fichiers d'autres formats sont grisés.
- **Zone de travail** — l'éditeur, l'aperçu ou les deux côte à côte.
- **Barre d'état** — chemin de la note, étiquettes, nombre de liens, erreurs de propriétés et état de l'enregistrement.

Sur les écrans étroits, le panneau latéral s'ouvre par-dessus le contenu et se masque après le choix d'une note.

## 3. Notes et dossiers

- **+ Note** crée une note dans le dossier sélectionné (ou à côté du fichier sélectionné). Le titre devient le nom du fichier ; les caractères interdits dans les noms de fichiers sont supprimés.
- Une nouvelle note reçoit les propriétés `id` (identifiant stable), `type: note` et `created`.
- **+ Dossier** crée un dossier.
- **✎** renomme l'élément sélectionné. L'extension `.md` est ajoutée automatiquement.
- **🗑** déplace l'élément dans la corbeille du coffre `.mdnotes/trash/`. Rien n'est supprimé définitivement : le fichier peut être replacé à la main.

## 4. Édition

- Les modifications sont enregistrées automatiquement peu après la fin de la saisie, ainsi qu'avec `Ctrl+S` / `Cmd+S`.
- L'écriture est atomique : un incident pendant l'enregistrement ne laisse jamais de fichier endommagé.
- Le mode **« Côte à côte »** affiche l'éditeur et l'aperçu ensemble. L'aperçu prend en charge GitHub Flavored Markdown : tableaux, listes de tâches, texte barré.

## 5. Liens

- `[[Titre de la note]]` renvoie vers une autre note ; il est cliquable dans l'aperçu.
- `[[Titre|texte]]` affiche un autre texte, `[[Titre#Section]]` renvoie vers une section.
- Si aucune note de ce nom n'existe, un clic la crée.
- Les liens sont résolus par chemin, puis par nom de fichier, sans tenir compte de la casse. Les liens dans le code sont ignorés.
- Les liens ordinaires `https://…` s'ouvrent dans le navigateur du système.

## 6. Étiquettes et propriétés

- `#étiquette` dans le texte ou une liste `tags` dans les propriétés désigne un thème. Les étiquettes imbriquées sont prises en charge : `#projet/design`.
- Les propriétés s'écrivent en YAML front matter en tête du fichier, entre des lignes `---`.
- Dans l'aperçu, les propriétés sont repliées dans un bloc **« Propriétés »**.
- Si le YAML contient une erreur, la barre d'état l'indique et la note s'ouvre comme du texte brut.

## 7. Thèmes

Choisissez **Système**, **Clair** ou **Sombre** dans le coin droit de la barre d'outils. Le choix est mémorisé.

## 8. Où sont stockées les données

- Les notes sont des fichiers ordinaires dans le dossier choisi ; elles restent utilisables sans MD Notes.
- `.mdnotes/` est le dossier de service du coffre : réglages, modèles, corbeille, cache.
- MD Notes n'envoie vos données nulle part : ni serveur, ni analyse d'usage, ni télémétrie.
- Sauvegardez le dossier du coffre. Git est pratique pour conserver les versions.

## 9. Clavier

| Action | Windows / Linux | macOS |
|---|---|---|
| Enregistrer | `Ctrl+S` | `Cmd+S` |
| Annuler / rétablir | `Ctrl+Z` / `Ctrl+Shift+Z` | `Cmd+Z` / `Cmd+Shift+Z` |
| Indenter | `Tab` | `Tab` |

## 10. Problèmes courants

- **Windows affiche SmartScreen** — la version n'est pas signée. Choisissez « Informations complémentaires → Exécuter quand même ».
- **macOS n'ouvre pas l'application** — la version n'est pas notarisée. Utilisez **Réglages Système → Confidentialité et sécurité → Ouvrir quand même**.
- **Le coffre ne s'ouvre pas au lancement** — le dossier a été renommé ou déplacé. Ouvrez-le de nouveau avec **« Autre coffre… »**.
- **Une note ne s'ouvre pas** — MD Notes n'ouvre que les fichiers `.md` et `.markdown`.

## 11. Droits d'auteur

Copyright © 2026 Roman Zavada (Роман Завада). Tous droits réservés. MD Notes est un logiciel propriétaire. Vos notes vous appartiennent. Voir [LICENSE.md](../../LICENSE.md) et [LEGAL_AND_COPYRIGHT.md](../LEGAL_AND_COPYRIGHT.md).
