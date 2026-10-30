# Sauvegarde et récupération — fonctionnement et procédure sûre

[🇺🇦 Українська](BACKUP_AND_RECOVERY.uk.md) · [🇬🇧 English](BACKUP_AND_RECOVERY.en.md) · **🇫🇷 Français** · [🇩🇪 Deutsch](BACKUP_AND_RECOVERY.de.md) · [🇪🇸 Español](BACKUP_AND_RECOVERY.es.md) · [🇰🇷 한국어](BACKUP_AND_RECOVERY.ko.md) · [🇯🇵 日本語](BACKUP_AND_RECOVERY.ja.md)

> Valable pour MD Notes v0.3.23. L’ukrainien est canonique. Backup/restore est livré depuis v0.3.22 ; des tests desktop automatisés existent, mais aucun test runtime complet séparé par le propriétaire n’a été enregistré.

## 1. Backup, synchronisation et Git sont différents

Un **backup** est un snapshot ZIP autonome pour la récupération. La **synchronisation** peut aussi propager une erreur ou une suppression. **Git** conserve un historique mais n’inclut pas nécessairement toutes les données locales et ne sauvegarde jamais automatiquement les clés d’un coffre chiffré.

Gardez au moins une sauvegarde indépendante du dossier de travail et de son canal de synchronisation.

## 2. Création

« Fichier → Créer une sauvegarde… » tente d’enregistrer les documents ouverts, parcourt les fichiers physiques, exclut les données non portables, calcule SHA-256, écrit le ZIP et `manifest.json`, puis vérifie que la source n’a pas changé avant de finaliser l’archive.

Si le vault change pendant l’opération, la sauvegarde doit échouer plutôt que produire un état mélangé.

## 3. Inclus

Notes Markdown, pièces jointes, configuration/schema/templates/views portables, `.mdnotes/trash/` et configuration du coffre chiffré.

## 4. Exclus

- `.git/`;
- `.mdnotes/cache/`;
- fichiers temporaires MD Notes ;
- `.DS_Store`, `Thumbs.db`, `desktop.ini` ;
- clés privées age/recovery.

Les liens symboliques sont refusés pour éviter une sortie du périmètre du vault.

## 5. Manifest

`manifest.json` contient format/version, version de l’application, date, nom du vault, indicateur encrypted et, pour chaque fichier, chemin relatif, taille et SHA-256.

Il sert à l’**intégrité**, pas au chiffrement. Il reste ouvert et peut révéler certaines métadonnées.

## 6. Limites v1

Jusqu’à 100 000 fichiers, 4 GiB par fichier, 16 GiB au total ; pas de chemin absolu, `..`, traversal, doublons ni collisions ne différant que par la casse.

## 7. Validation avant restauration

MD Notes lit et valide d’abord le manifest, les chemins, entrées ZIP, tailles et SHA-256, et vérifie l’absence de fichiers manquants ou supplémentaires. Une erreur arrête la procédure avant création d’un vault exploitable.

## 8. Destination

Après validation, extraction dans un dossier temporaire puis création d’un **nouveau** vault. Aucun dossier existant n’est écrasé silencieusement : `My Vault-restored`, `My Vault-restored-2`, etc.

## 9. Vault normal

Le ZIP contient du Markdown et des pièces jointes lisibles. Protégez-le comme le dossier original.

## 10. Vault chiffré

Le backup copie l’état physique ; les fichiers age restent donc chiffrés. `.mdnotes/encryption.json` reste ouvert, cache et clés privées sont exclus.

La récupération complète exige :
1. le backup du vault ;
2. au moins une clé privée age valide.

Voir [Coffre chiffré](ENCRYPTED_VAULT.fr.md).

## 11. Modèle recommandé

Copie de travail + sauvegardes régulières sur autre support/emplacement + clé de récupération séparée pour un vault encrypted. Le principe 3-2-1 est une bonne cible pour les données importantes.

## 12. Procédure de sauvegarde

Enregistrer les modifications → créer le backup → placer le ZIP hors du vault → vérifier le succès → copier vers un stockage indépendant → vérifier que la recovery key est ailleurs.

## 13. Procédure de restauration

Ne pas supprimer l’ancien vault → restaurer vers un nouveau dossier → laisser checksums/manifest être validés → ouvrir `*-restored` → tester notes, pièces jointes et graphe → importer la clé si nécessaire → seulement ensuite décider du sort de l’ancienne copie.

## 14. Exercice de récupération

Périodiquement, restaurez un backup dans un dossier distinct et vérifiez réellement son contenu. Pour un vault chiffré, vérifiez aussi que la clé de récupération est utilisable.

## 15. Erreurs courantes

Même disque physique pour travail et backup ; ciphertext sans clé ; clé sans copie récente ; sync confondu avec backup ; ZIP normal rendu public ; ancien vault supprimé trop tôt ; backup pendant une réécriture massive externe.

## 16. Erreur checksum

Ne modifiez pas le manifest. Une discordance signifie que l’archive ne correspond plus au snapshot enregistré. Conservez-la pour analyse et utilisez une autre copie.

## 17. IA et automatisation

Une IA ne doit pas supprimer la dernière bonne copie avant validation, assimiler sync/Git à un backup, ajouter les clés privées au ZIP, contourner les validations, restaurer par-dessus un vault existant ou annoncer un succès sans vérifier le nouveau vault.

Règles encrypted : [ENCRYPTED_VAULT_CONTRACT.md](../ai/ENCRYPTED_VAULT_CONTRACT.md).

> Traduction automatique non relue par un locuteur natif ; l’ukrainien est canonique.
