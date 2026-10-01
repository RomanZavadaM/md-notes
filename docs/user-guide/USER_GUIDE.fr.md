# MD Notes — Guide de l'utilisateur

[🇺🇦 Українська](USER_GUIDE.uk.md) · [🇬🇧 English](USER_GUIDE.en.md) · **🇫🇷 Français** · [🇩🇪 Deutsch](USER_GUIDE.de.md) · [🇪🇸 Español](USER_GUIDE.es.md) · [🇰🇷 한국어](USER_GUIDE.ko.md) · [🇯🇵 日本語](USER_GUIDE.ja.md)

S'applique à la **v0.2.0**. La version ukrainienne fait foi.

## 1. Premier lancement

1. Installez MD Notes pour votre système (voir le README).
2. Si besoin, choisissez la langue de l'interface sur l'écran d'accueil.
3. Cliquez sur **« Ouvrir un dossier »** et choisissez un dossier de notes. Tout dossier contenant des fichiers `.md` convient, y compris `sample-vault/` du dépôt.
4. MD Notes mémorise le dossier et l'ouvre au lancement suivant.

Ouvrir un dossier ne modifie pas vos notes. MD Notes crée le dossier de service `.mdnotes/` pour l'index de recherche, la corbeille et les réglages.

## 2. La fenêtre

- **Barre d'outils** :
  - bouton du panneau latéral ☰ et nom du coffre ;
  - titre de la note ouverte : un clic ouvre la navigation rapide, un point signale des modifications non enregistrées ;
  - modes « Éditeur » / « Côte à côte » / « Aperçu » ;
  - bouton du panneau des liens ⇆ ;
  - choix du thème et de la langue, bouton « À propos » ⓘ.
- **Panneau latéral** — onglets **Fichiers**, **Recherche** et **Étiquettes**.
  - Dans l'arborescence, les dossiers apparaissent en premier et les dossiers cachés (`.mdnotes`, `.git`) ne sont pas affichés.
  - Les fichiers d'autres formats sont grisés.
- **Zone de travail** — l'éditeur, l'aperçu ou les deux côte à côte.
- **Panneau « Liens »** à droite — rétroliens, liens sortants et étiquettes de la note ouverte.
- **Barre d'état** — chemin de la note, étiquettes, nombre de liens, erreurs de propriétés et état de l'enregistrement.

Sur les écrans étroits, le panneau latéral et le panneau des liens s'ouvrent par-dessus le contenu.

## 3. Notes et dossiers

- **+ Note** crée une note dans le dossier sélectionné (ou à côté du fichier sélectionné). La boîte de dialogue permet de choisir un **modèle** (section 9).
- Le titre devient le nom du fichier ; les caractères interdits dans les noms de fichiers sont supprimés. Une nouvelle note reçoit les propriétés `id`, `type` et `created`.
- **Aujourd'hui** ouvre la note quotidienne (section 9).
- **+ Dossier** crée un dossier.
- **✎** renomme l'élément sélectionné. L'extension `.md` est ajoutée automatiquement. **Les liens vers la note renommée, ou vers les notes d'un dossier renommé, sont mis à jour dans toutes les autres notes**, propriétés comprises. L'application indique combien de notes ont été mises à jour.
- **🗑** déplace l'élément dans la corbeille du coffre `.mdnotes/trash/`. Rien n'est supprimé définitivement : le fichier peut être replacé à la main.

## 4. Édition

- Les modifications sont enregistrées automatiquement peu après la fin de la saisie, ainsi qu'avec `Ctrl+S` / `Cmd+S`.
- L'écriture est atomique : un incident pendant l'enregistrement ne laisse jamais de fichier endommagé.
- Si un autre programme (éditeur, Git, client cloud) modifie un fichier, MD Notes actualise l'arborescence et recharge la note ouverte. Si vous avez des modifications non enregistrées, l'application vous avertit que l'enregistrement écrasera les modifications externes.

## 5. Aperçu

- GitHub Flavored Markdown : tableaux, listes de tâches, texte barré.
- **Diagrammes Mermaid** — un bloc de code avec le langage `mermaid`.
- **Formules KaTeX** — `$…$` en ligne et `$$…$$` en bloc.
- **Images** du coffre : chemin relatif `![](../attachments/2026/10/schema.png)` ou incorporation `![[schema.png]]`. Seuls les fichiers du coffre ouvert sont affichés.
- Les liens relatifs vers des fichiers `.md` ouvrent la note dans l'application ; les liens `https://…` s'ouvrent dans le navigateur du système.

## 6. Liens

- `[[Titre de la note]]` renvoie vers une autre note ; il est cliquable dans l'aperçu.
- `[[Titre|texte]]` affiche un autre texte, `[[Titre#Section]]` renvoie vers une section.
- Si aucune note de ce nom n'existe, un clic la crée.
- Les liens sont résolus par chemin, nom de fichier ou propriété `aliases`, sans tenir compte de la casse. Les liens dans le code sont ignorés.
- Les liens dans les propriétés (`project: "[[MD Notes]]"`) sont aussi pris en compte.
- Le panneau **« Liens »** (⇆) montre quelles notes renvoient vers la note ouverte, avec la ligne de contexte.

## 7. Recherche, étiquettes et navigation rapide

- **Recherche** (`Ctrl+Shift+F` / `Cmd+Shift+F`) parcourt le texte et les titres de toutes les notes. Chaque mot de la requête correspond au début d'un mot. Les correspondances dans le titre sont classées plus haut et les mots trouvés sont surlignés.
- **Étiquettes** — toutes les étiquettes avec le nombre de notes. Choisir une étiquette affiche ses notes, étiquettes imbriquées comprises (`#projet` trouve aussi `#projet/design`).
- **Navigation rapide** (`Ctrl+O` / `Cmd+O`, ou `Ctrl+P`) — tapez une partie du titre, d'un alias ou du chemin. `↑`/`↓` sélectionnent, `Entrée` ouvre. Si la note n'existe pas, `Entrée` la crée.

## 8. Étiquettes et propriétés

- `#étiquette` dans le texte ou une liste `tags` dans les propriétés désigne un thème. Les étiquettes en fin de titre ne font pas partie du titre de la note.
- Les propriétés s'écrivent en YAML front matter en tête du fichier, entre des lignes `---`.
- Dans l'aperçu, les propriétés sont repliées dans un bloc **« Propriétés »**.
- Si le YAML contient une erreur, la barre d'état l'indique et la note s'ouvre comme du texte brut.

## 9. Modèles et notes quotidiennes

- Les modèles sont de simples fichiers `.md` dans `.mdnotes/templates/`. Les noms de types de la liste proviennent de `.mdnotes/schema.json`.
- Variables : `{{title}}` — titre, `{{date}}` — date `AAAA-MM-JJ`, `{{time}}` — heure `HH:MM`, `{{id}}` — nouvel identifiant.
- **Aujourd'hui** ouvre la note `AAAA-MM-JJ.md` dans le dossier des notes quotidiennes (`dailyNotesDir` dans `.mdnotes/config.json`, `daily` par défaut). Si elle n'existe pas, elle est créée à partir du modèle `daily`.

## 10. Thèmes et langues

- Thème : **Système**, **Clair** ou **Sombre**.
- Langue de l'interface : 🇺🇦 Українська (principale) · 🇬🇧 English · 🇫🇷 Français · 🇩🇪 Deutsch · 🇪🇸 Español · 🇰🇷 한국어 · 🇯🇵 日本語.
- Les deux choix sont mémorisés. La fenêtre **« À propos »** (ⓘ) affiche la version, le titulaire des droits et la liste des langues.

## 11. Où sont stockées les données

- Les notes sont des fichiers ordinaires dans le dossier choisi ; elles restent utilisables sans MD Notes.
- `.mdnotes/` est le dossier de service du coffre : réglages, modèles, corbeille, cache.
- `.mdnotes/cache/index.db` est l'index de recherche et de liens. Ce n'est qu'un cache : vous pouvez le supprimer, il sera reconstruit. Le cache et la corbeille sont exclus de Git (`.mdnotes/.gitignore`).
- MD Notes n'envoie vos données nulle part : ni serveur, ni analyse d'usage, ni télémétrie.
- Sauvegardez le dossier du coffre. Git est pratique pour conserver les versions.

## 12. Clavier

| Action | Windows / Linux | macOS |
|---|---|---|
| Enregistrer | `Ctrl+S` | `Cmd+S` |
| Navigation rapide | `Ctrl+O` / `Ctrl+P` | `Cmd+O` / `Cmd+P` |
| Recherche | `Ctrl+Shift+F` | `Cmd+Shift+F` |
| Annuler / rétablir | `Ctrl+Z` / `Ctrl+Shift+Z` | `Cmd+Z` / `Cmd+Shift+Z` |
| Indenter | `Tab` | `Tab` |

## 13. Problèmes courants

- **Windows affiche SmartScreen** — la version n'est pas signée. Choisissez « Informations complémentaires → Exécuter quand même ».
- **macOS n'ouvre pas l'application** — la version n'est pas notarisée. Utilisez **Réglages Système → Confidentialité et sécurité → Ouvrir quand même**.
- **Le coffre ne s'ouvre pas au lancement** — le dossier a été renommé ou déplacé. Ouvrez-le de nouveau avec **« Autre coffre… »**.
- **La recherche ne trouve pas une note que vous venez de modifier** — fermez puis rouvrez le coffre. Si cela ne suffit pas, supprimez `.mdnotes/cache/index.db` et l'index sera reconstruit.
- **Une image ne s'affiche pas** — vérifiez que le fichier se trouve dans le coffre et que le chemin est relatif.

## 14. Droits d'auteur

Copyright © 2026 Roman Zavada (Роман Завада). Tous droits réservés. MD Notes est un logiciel propriétaire. Vos notes vous appartiennent. Voir [LICENSE.md](../../LICENSE.md) et [LEGAL_AND_COPYRIGHT.md](../LEGAL_AND_COPYRIGHT.md).
