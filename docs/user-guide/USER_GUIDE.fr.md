# MD Notes — Guide de l'utilisateur

[🇺🇦 Українська](USER_GUIDE.uk.md) · [🇬🇧 English](USER_GUIDE.en.md) · **🇫🇷 Français** · [🇩🇪 Deutsch](USER_GUIDE.de.md) · [🇪🇸 Español](USER_GUIDE.es.md) · [🇰🇷 한국어](USER_GUIDE.ko.md) · [🇯🇵 日本語](USER_GUIDE.ja.md)

S'applique à la **v0.3.19**. La version ukrainienne fait foi.

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

### Coffre chiffré (facultatif)

- À la création du coffre, cochez **« Chiffrer ce coffre »**. Les notes sont stockées sur le disque en fichiers `.age`, illisibles sans la clé.
- Après la création, la clé privée (`AGE-SECRET-KEY-…`) est affichée **une seule fois**. Conservez-la dans un gestionnaire de mots de passe ou hors ligne : elle ne peut pas être récupérée. Durablement, la clé ne reste que dans le magasin de secrets système de cet appareil.
- Vous pouvez indiquer une **clé publique de récupération** (`age1…`) : elle permet aussi d’ouvrir le coffre si la clé principale est perdue.
- Sur un autre ordinateur, copiez le dossier du coffre et ouvrez-le : l’application demande la clé. Collez la clé privée ; elle est ensuite conservée dans le magasin de secrets de cet ordinateur.
- Pour l’instant, un coffre chiffré n’affiche pas les images ni les pièces jointes, et l’index `.mdnotes/cache` n’est pas chiffré.

### Git (ordinateurs uniquement)

Le bouton **Git** (ou « Git… » dans le menu) ouvre un panneau : créer un dépôt, définir l’adresse distante, enregistrer l’accès (le jeton reste uniquement dans le coffre de secrets du système), puis **fetch**, **pull**, **commit** et **push**. Le pull ne s’exécute que sans modifications non enregistrées et lorsque le distant a simplement du nouveau ; si les historiques ont divergé, une décision manuelle est nécessaire. Le push exige que Git soit installé. Dans un coffre chiffré, le push vérifie d’abord que tous les fichiers sont chiffrés et se bloque sinon.

## 12. Graphe, arbre des relations et recherche d’un sujet

### Graphe

Le panneau **« Liens »** propose les onglets « Liens », « Graphe local » et « Graphe global ». Un nœud est une note, sa couleur est le `type` de la note, une ligne est un lien `[[…]]` ou une relation typée. Cliquez sur un nœud pour ouvrir la note.

- **Physique vivante (graphe « flottant »).** Faites glisser un nœud : ses voisins le suivent et le contournent, puis le graphe se stabilise en douceur. Le bouton **« Flottement »** ajoute une légère dérive (activé par défaut jusqu’à 400 nœuds), **« Pause »** arrête le mouvement. **« Forces »** propose des curseurs pour le centre, la répulsion, la force et la longueur des liens ; votre choix est mémorisé.
- **« Formes » et « Légende ».** Le bouton **« Formes »** active un mode où une **forme** indique le type de la note (personne — tête et épaules, société — carré, institution — hexagone, affaire — losange, document — triangle, groupe — trois points) et la **couleur du nœud** indique la **fiabilité** de la meilleure note de ses relations (vert 1–2, ambre 3–4, rouge 5–6, gris — sans note). Ouvrez **« Légende »** pour voir toutes les formes, couleurs et styles de trait ; un contour en pointillés et le badge « Ч » signalent une rumeur, la couleur n’est jamais le seul signal.
- **Filtre « Afficher les relations ».** La liste du panneau du graphe ne garde que les relations notées au moins aussi bien que le choix (« fiables » — 1–2, « fiables et moyens » — 1–4) ; les relations sans note et les simples liens sont masqués, les nœuds restent.
- **Relations typées** : propriété `relations` (`to`, plus `type`, `grade`, `origin`, `status`, `evidence`, et `dir: in` pour le sens inverse). Un `grade` est une lettre et un chiffre (A1 le plus fiable, F6 le plus faible) : la lettre note la source, le chiffre l’affirmation. Le trait et le contour du nœud suivent la note ; les notes 5–6 portent le mot **« RUMEUR »**. La couleur n’est jamais le seul signal.
- **« D’où cela vient »** met en évidence la chaîne des propriétaires au-dessus du nœud choisi et les documents autour.
- **Fenêtre séparée.** Le bouton **↗** des onglets du graphe, ou « Fichier → Graphe dans une fenêtre séparée » (bureau uniquement), ouvre le graphe dans sa propre fenêtre, par exemple sur un second écran. Elle suit la note active, se met à jour après les modifications, et un clic sur un nœud ouvre la note dans la fenêtre principale.
- **Bouton « Retour ».** Dans la fenêtre du graphe, il revient à la note précédente.

### Arbre des relations

L’onglet **« Arbre des relations »** montre la chaîne de relations typées à partir de la note choisie : **« Propriétaires et sources »** (vers le haut) ou **« Ce qu’elle possède »** (vers le bas), profondeur 1–8.

- Chaque nœud indique le type de relation et la note du trait ; à côté, la note de **« chaîne »** : le maillon le plus faible depuis la racine (B2 avec C1 donne C2). Une chaîne vaut son maillon le plus faible.
- **« −/+ »** replie une branche, **◎** reconstruit l’arbre depuis un autre nœud, **« Tout déplier » / « Tout replier »** agissent sur toutes les branches. Un nœud répété est signalé et non redéployé ; l’arbre est limité à 400 nœuds.
- **« Chemin entre deux notes »** trouve la plus courte chaîne de relations typées (sens des traits ignoré) et indique son maillon le plus faible. La case **« Le plus fiable, pas le plus court »** cherche la chaîne dont le maillon le plus faible est le meilleur (le moins d’étapes à fiabilité égale).
- L’arbre n’utilise que des relations typées explicites, pas les liens simples. La note de chaîne est une aide, pas une conclusion juridique.

### Rechercher un sujet par code ou par nom

Dans l’onglet **« Recherche »**, un nombre de 8 chiffres est cherché comme code EDRPOU, de 10 chiffres comme numéro fiscal (un code ayant perdu ses zéros initiaux, 6–7 chiffres, est restitué). Les résultats sont séparés : Pour un code à 8 chiffres, un bloc **« Base « З-під Ляди » par code »** apparaît en dessous : choisissez le dossier des données ouvertes du site (copie locale de la branche data) et il affiche les faits pour le code exact — date d’enregistrement, état et pays étrangers des fondateurs et bénéficiaires (registre), décisions de l’autorité de la concurrence, marchés Prozorro comme acheteur et gagnant. Lecture seule, rien n’est envoyé sur le réseau ; ce sont des faits, pas des évaluations.

- **« Correspondance exacte »** : une note avec le même code dans la propriété `edrpou` ou `ipn` (ou le même nom après normalisation) ;
- **« Candidats par nom »** : noms proches ; une correspondance de nom **ne prouve rien**, confirmez par un code ou un document ;
- **« Dans le texte »** : recherche plein texte ordinaire.

Les noms sont comparés sans tenir compte de la casse, des guillemets, des traits d’union et des formes juridiques (SARL, etc.) ; les lettres latines ressemblant à des cyrilliques sont ramenées au cyrillique.

### Import d’un dossier, fusion par code et doublons possibles

- **« Fichier → Importer un dossier… »** ajoute les notes d’un dossier. Une note dont le code (`edrpou`/`ipn`) existe déjà dans le coffre est **fusionnée** avec la note existante au lieu d’être dupliquée : les propriétés manquantes sont ajoutées, les étiquettes, alias et relations réunis, le titre de la note importée devient un alias, les valeurs divergentes ne sont pas écrasées mais listées pour relecture. Réimporter le même dossier ne change rien.
- **« Fichier → Doublons possibles… »** liste les groupes de notes ayant le même titre ou alias normalisé. Ce n’est qu’une indication : rien n’est modifié, et des notes de codes différents ne sont jamais regroupées.

## 13. Clavier

| Action | Windows / Linux | macOS |
|---|---|---|
| Enregistrer | `Ctrl+S` | `Cmd+S` |
| Navigation rapide | `Ctrl+O` / `Ctrl+P` | `Cmd+O` / `Cmd+P` |
| Recherche | `Ctrl+Shift+F` | `Cmd+Shift+F` |
| Annuler / rétablir | `Ctrl+Z` / `Ctrl+Shift+Z` | `Cmd+Z` / `Cmd+Shift+Z` |
| Indenter | `Tab` | `Tab` |

## 14. Problèmes courants

- **Windows affiche SmartScreen** — la version n'est pas signée. Choisissez « Informations complémentaires → Exécuter quand même ».
- **macOS n'ouvre pas l'application** — la version n'est pas notarisée. Utilisez **Réglages Système → Confidentialité et sécurité → Ouvrir quand même**.
- **Le coffre ne s'ouvre pas au lancement** — le dossier a été renommé ou déplacé. Ouvrez-le de nouveau avec **« Autre coffre… »**.
- **La recherche ne trouve pas une note que vous venez de modifier** — fermez puis rouvrez le coffre. Si cela ne suffit pas, supprimez `.mdnotes/cache/index.db` et l'index sera reconstruit.
- **Une image ne s'affiche pas** — vérifiez que le fichier se trouve dans le coffre et que le chemin est relatif.

## 15. Droits d'auteur

Copyright © 2026 Roman Zavada (Роман Завада). Tous droits réservés. MD Notes est un logiciel propriétaire. Vos notes vous appartiennent. Voir [LICENSE.md](../../LICENSE.md) et [LEGAL_AND_COPYRIGHT.md](../LEGAL_AND_COPYRIGHT.md).


## Onglets et fenêtres d’édition séparées

Dans le `main` actuel après v0.3.20, plusieurs notes du coffre peuvent rester ouvertes dans des onglets. Chaque onglet conserve son propre brouillon afin qu’un changement d’onglet ne supprime pas le texte non enregistré.

Sur ordinateur, la note active du coffre propose **Ouvrir dans une fenêtre séparée**. MD Notes enregistre le brouillon avant de le détacher. Une fois la fenêtre indépendante créée, la note est retirée des onglets de l’éditeur principal et la fenêtre principale revient à l’onglet précédent ; s’il n’en reste aucun, l’éditeur devient vide tout en gardant l’arbre du coffre disponible. La fenêtre séparée prend en charge Éditeur / côte à côte / aperçu et l’enregistrement automatique.

Un fichier externe `.md/.markdown` reste un document séparé et n’est pas importé automatiquement dans le coffre. Le mode multi-fenêtres doit encore être validé manuellement en exécution ; la protection contre les conflits d’édition simultanée entre fenêtres est prévue pour M3.
