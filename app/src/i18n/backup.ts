import type { LanguageCode } from "./index";

export interface BackupStrings {
  create: string;
  restore: string;
  createTitle: string;
  restoreArchiveTitle: string;
  restoreFolderTitle: string;
  archiveFilter: string;
  created: (files: number, path: string) => string;
  createdEncrypted: string;
  restored: (files: number, path: string) => string;
  openRestored: string;
  openRestoredTitle: string;
}

export const BACKUP_STRINGS: Record<LanguageCode, BackupStrings> = {
  uk: {
    create: "Створити резервну копію…",
    restore: "Відновити резервну копію…",
    createTitle: "Зберегти резервну копію сховища",
    restoreArchiveTitle: "Виберіть резервну копію MD Notes",
    restoreFolderTitle: "Виберіть папку, де створити відновлене сховище",
    archiveFilter: "Резервна копія MD Notes",
    created: (files, path) => "Резервну копію створено: " + files + " файлів.\n" + path,
    createdEncrypted: "Зашифроване сховище скопійовано без розшифрування. Приватний ключ не входить до архіву — зберігайте recovery key окремо.",
    restored: (files, path) => "Сховище відновлено в нову папку: " + files + " файлів.\n" + path,
    openRestored: "Відкрити відновлене сховище зараз?",
    openRestoredTitle: "Відновлення завершено",
  },
  en: {
    create: "Create backup…",
    restore: "Restore backup…",
    createTitle: "Save vault backup",
    restoreArchiveTitle: "Choose an MD Notes backup",
    restoreFolderTitle: "Choose the folder where the restored vault will be created",
    archiveFilter: "MD Notes backup",
    created: (files, path) => "Backup created: " + files + " files.\n" + path,
    createdEncrypted: "The encrypted vault was copied without decryption. The private key is not in the archive; keep the recovery key separately.",
    restored: (files, path) => "Vault restored into a new folder: " + files + " files.\n" + path,
    openRestored: "Open the restored vault now?",
    openRestoredTitle: "Restore complete",
  },
  fr: {
    create: "Créer une sauvegarde…",
    restore: "Restaurer une sauvegarde…",
    createTitle: "Enregistrer la sauvegarde du coffre",
    restoreArchiveTitle: "Choisissez une sauvegarde MD Notes",
    restoreFolderTitle: "Choisissez le dossier où créer le coffre restauré",
    archiveFilter: "Sauvegarde MD Notes",
    created: (files, path) => "Sauvegarde créée : " + files + " fichiers.\n" + path,
    createdEncrypted: "Le coffre chiffré a été copié sans déchiffrement. La clé privée n’est pas dans l’archive ; conservez la clé de récupération séparément.",
    restored: (files, path) => "Coffre restauré dans un nouveau dossier : " + files + " fichiers.\n" + path,
    openRestored: "Ouvrir maintenant le coffre restauré ?",
    openRestoredTitle: "Restauration terminée",
  },
  de: {
    create: "Sicherung erstellen…",
    restore: "Sicherung wiederherstellen…",
    createTitle: "Tresor-Sicherung speichern",
    restoreArchiveTitle: "MD-Notes-Sicherung auswählen",
    restoreFolderTitle: "Ordner für den wiederhergestellten Tresor auswählen",
    archiveFilter: "MD-Notes-Sicherung",
    created: (files, path) => "Sicherung erstellt: " + files + " Dateien.\n" + path,
    createdEncrypted: "Der verschlüsselte Tresor wurde ohne Entschlüsselung kopiert. Der private Schlüssel ist nicht im Archiv; bewahren Sie den Wiederherstellungsschlüssel separat auf.",
    restored: (files, path) => "Tresor in einem neuen Ordner wiederhergestellt: " + files + " Dateien.\n" + path,
    openRestored: "Wiederhergestellten Tresor jetzt öffnen?",
    openRestoredTitle: "Wiederherstellung abgeschlossen",
  },
  es: {
    create: "Crear copia de seguridad…",
    restore: "Restaurar copia de seguridad…",
    createTitle: "Guardar copia de la bóveda",
    restoreArchiveTitle: "Elige una copia de MD Notes",
    restoreFolderTitle: "Elige la carpeta donde crear la bóveda restaurada",
    archiveFilter: "Copia de MD Notes",
    created: (files, path) => "Copia creada: " + files + " archivos.\n" + path,
    createdEncrypted: "La bóveda cifrada se copió sin descifrar. La clave privada no está en el archivo; guarda la clave de recuperación por separado.",
    restored: (files, path) => "Bóveda restaurada en una carpeta nueva: " + files + " archivos.\n" + path,
    openRestored: "¿Abrir ahora la bóveda restaurada?",
    openRestoredTitle: "Restauración completada",
  },
  ko: {
    create: "백업 만들기…",
    restore: "백업 복원…",
    createTitle: "보관함 백업 저장",
    restoreArchiveTitle: "MD Notes 백업 선택",
    restoreFolderTitle: "복원된 보관함을 만들 폴더 선택",
    archiveFilter: "MD Notes 백업",
    created: (files, path) => "백업을 만들었습니다: " + files + "개 파일.\n" + path,
    createdEncrypted: "암호화 보관함은 복호화하지 않고 복사했습니다. 개인 키는 압축 파일에 포함되지 않으므로 복구 키를 별도로 보관하세요.",
    restored: (files, path) => "새 폴더에 보관함을 복원했습니다: " + files + "개 파일.\n" + path,
    openRestored: "복원된 보관함을 지금 열까요?",
    openRestoredTitle: "복원 완료",
  },
  ja: {
    create: "バックアップを作成…",
    restore: "バックアップを復元…",
    createTitle: "保管庫バックアップを保存",
    restoreArchiveTitle: "MD Notes バックアップを選択",
    restoreFolderTitle: "復元した保管庫を作成するフォルダーを選択",
    archiveFilter: "MD Notes バックアップ",
    created: (files, path) => "バックアップを作成しました: " + files + " ファイル。\n" + path,
    createdEncrypted: "暗号化保管庫は復号せずにコピーされました。秘密鍵はアーカイブに含まれません。復旧キーは別に保管してください。",
    restored: (files, path) => "新しいフォルダーに保管庫を復元しました: " + files + " ファイル。\n" + path,
    openRestored: "復元した保管庫を今開きますか？",
    openRestoredTitle: "復元完了",
  },
};
