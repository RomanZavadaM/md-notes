import type { LanguageCode } from "./index";

export interface EncryptionStrings {
  createEncrypted: string;
  createEncryptedHelp: string;
  recoveryKey: string;
  recoveryKeyPlaceholder: string;
  recoveryKeyHelp: string;
  backupTitle: string;
  backupText: string;
  copy: string;
  copied: string;
  backupConfirm: string;
  backupContinue: string;
  importTitle: string;
  importText: string;
  importPlaceholder: string;
  importButton: string;
  importing: string;
  badge: string;
  badgeTitle: string;
  manageRecovery: string;
  manageRecoveryTitle: string;
  manageRecoveryText: string;
  addRecovery: string;
  addingRecovery: string;
  recoveryAdded: (files: number) => string;
  attachmentExternalOpenUnavailable: string;
}

export const ENCRYPTION_STRINGS: Record<LanguageCode, EncryptionStrings> = {
  uk: {
    createEncrypted: "Зашифрувати сховище",
    createEncryptedHelp:
      "Нотатки лежать на диску зашифрованими (age). Приватний ключ зберігається лише в системному сховищі секретів цього пристрою.",
    recoveryKey: "Публічний ключ відновлення (необов’язково)",
    recoveryKeyPlaceholder: "age1…",
    recoveryKeyHelp: "Другий ключ, яким теж можна відкрити сховище, якщо основний втрачено.",
    backupTitle: "Збережіть ключ сховища",
    backupText:
      "Це приватний ключ. Його показано один раз, відновити його неможливо. Без нього зашифроване сховище не відкрити на іншому пристрої. Збережіть ключ у менеджері паролів або офлайн.",
    copy: "Копіювати",
    copied: "Скопійовано",
    backupConfirm: "Ключ збережено",
    backupContinue: "Продовжити",
    importTitle: "Потрібен ключ сховища",
    importText:
      "Це зашифроване сховище, а на цьому пристрої немає його ключа. Вставте приватний ключ (AGE-SECRET-KEY-…).",
    importPlaceholder: "AGE-SECRET-KEY-…",
    importButton: "Імпортувати й відкрити",
    importing: "Перевірка ключа…",
    badge: "Зашифровано",
    badgeTitle: "Нотатки зберігаються на диску зашифрованими (age)",
    manageRecovery: "Керувати ключем відновлення…",
    manageRecoveryTitle: "Додати ключ відновлення",
    manageRecoveryText: "Додайте ще один публічний age-ключ. MD Notes повторно зашифрує файли сховища для всіх одержувачів; приватний recovery key сюди не вставляйте.",
    addRecovery: "Додати ключ",
    addingRecovery: "Перешифрування…",
    recoveryAdded: (files) => `Ключ відновлення додано. Перешифровано файлів: ${files}.`,
    attachmentExternalOpenUnavailable: "У зашифрованому сховищі зовнішнє відкриття вимкнене, щоб не створювати plaintext-копію на диску.",
  },
  en: {
    createEncrypted: "Encrypt this vault",
    createEncryptedHelp:
      "Notes are stored on disk encrypted (age). The private key is kept only in this device's system secret store.",
    recoveryKey: "Recovery public key (optional)",
    recoveryKeyPlaceholder: "age1…",
    recoveryKeyHelp: "A second key that can also open the vault if the main key is lost.",
    backupTitle: "Back up the vault key",
    backupText:
      "This is the private key. It is shown once and cannot be recovered. Without it the encrypted vault cannot be opened on another device. Keep it in a password manager or offline.",
    copy: "Copy",
    copied: "Copied",
    backupConfirm: "I have saved the key",
    backupContinue: "Continue",
    importTitle: "Vault key required",
    importText:
      "This vault is encrypted and this device does not have its key. Paste the private key (AGE-SECRET-KEY-…).",
    importPlaceholder: "AGE-SECRET-KEY-…",
    importButton: "Import and open",
    importing: "Checking key…",
    badge: "Encrypted",
    badgeTitle: "Notes are stored on disk encrypted (age)",
    manageRecovery: "Manage recovery key…",
    manageRecoveryTitle: "Add a recovery key",
    manageRecoveryText: "Add another public age key. MD Notes will re-encrypt vault files for all recipients; do not paste the private recovery key here.",
    addRecovery: "Add key",
    addingRecovery: "Re-encrypting…",
    recoveryAdded: (files) => `Recovery key added. Files re-encrypted: ${files}.`,
    attachmentExternalOpenUnavailable: "External opening is disabled for encrypted vaults so MD Notes does not create a plaintext copy on disk.",
  },
  fr: {
    createEncrypted: "Chiffrer ce coffre",
    createEncryptedHelp:
      "Les notes sont stockées chiffrées (age) sur le disque. La clé privée reste uniquement dans le magasin de secrets système de cet appareil.",
    recoveryKey: "Clé publique de récupération (facultatif)",
    recoveryKeyPlaceholder: "age1…",
    recoveryKeyHelp: "Une seconde clé qui peut aussi ouvrir le coffre si la clé principale est perdue.",
    backupTitle: "Sauvegardez la clé du coffre",
    backupText:
      "Il s’agit de la clé privée. Elle n’est affichée qu’une fois et ne peut pas être récupérée. Sans elle, le coffre chiffré ne peut pas être ouvert sur un autre appareil. Conservez-la dans un gestionnaire de mots de passe ou hors ligne.",
    copy: "Copier",
    copied: "Copié",
    backupConfirm: "J’ai enregistré la clé",
    backupContinue: "Continuer",
    importTitle: "Clé du coffre requise",
    importText:
      "Ce coffre est chiffré et cet appareil n’en possède pas la clé. Collez la clé privée (AGE-SECRET-KEY-…).",
    importPlaceholder: "AGE-SECRET-KEY-…",
    importButton: "Importer et ouvrir",
    importing: "Vérification de la clé…",
    badge: "Chiffré",
    badgeTitle: "Les notes sont stockées chiffrées (age) sur le disque",
    manageRecovery: "Gérer la clé de récupération…",
    manageRecoveryTitle: "Ajouter une clé de récupération",
    manageRecoveryText: "Ajoutez une autre clé publique age. MD Notes rechiffrera les fichiers pour tous les destinataires ; ne collez pas ici la clé privée de récupération.",
    addRecovery: "Ajouter la clé",
    addingRecovery: "Rechiffrement…",
    recoveryAdded: (files) => `Clé de récupération ajoutée. Fichiers rechiffrés : ${files}.`,
    attachmentExternalOpenUnavailable: "L’ouverture externe est désactivée pour les coffres chiffrés afin de ne pas créer de copie en clair sur le disque.",
  },
  de: {
    createEncrypted: "Tresor verschlüsseln",
    createEncryptedHelp:
      "Notizen werden verschlüsselt (age) auf der Festplatte gespeichert. Der private Schlüssel liegt nur im System-Geheimnisspeicher dieses Geräts.",
    recoveryKey: "Öffentlicher Wiederherstellungsschlüssel (optional)",
    recoveryKeyPlaceholder: "age1…",
    recoveryKeyHelp: "Ein zweiter Schlüssel, der den Tresor ebenfalls öffnet, falls der Hauptschlüssel verloren geht.",
    backupTitle: "Tresorschlüssel sichern",
    backupText:
      "Dies ist der private Schlüssel. Er wird nur einmal angezeigt und lässt sich nicht wiederherstellen. Ohne ihn kann der verschlüsselte Tresor auf einem anderen Gerät nicht geöffnet werden. Bewahren Sie ihn in einem Passwortmanager oder offline auf.",
    copy: "Kopieren",
    copied: "Kopiert",
    backupConfirm: "Ich habe den Schlüssel gesichert",
    backupContinue: "Weiter",
    importTitle: "Tresorschlüssel erforderlich",
    importText:
      "Dieser Tresor ist verschlüsselt, und dieses Gerät besitzt seinen Schlüssel nicht. Fügen Sie den privaten Schlüssel ein (AGE-SECRET-KEY-…).",
    importPlaceholder: "AGE-SECRET-KEY-…",
    importButton: "Importieren und öffnen",
    importing: "Schlüssel wird geprüft…",
    badge: "Verschlüsselt",
    badgeTitle: "Notizen werden verschlüsselt (age) auf der Festplatte gespeichert",
    manageRecovery: "Wiederherstellungsschlüssel verwalten…",
    manageRecoveryTitle: "Wiederherstellungsschlüssel hinzufügen",
    manageRecoveryText: "Fügen Sie einen weiteren öffentlichen age-Schlüssel hinzu. MD Notes verschlüsselt die Vault-Dateien für alle Empfänger neu; fügen Sie hier keinen privaten Recovery-Schlüssel ein.",
    addRecovery: "Schlüssel hinzufügen",
    addingRecovery: "Neuverschlüsselung…",
    recoveryAdded: (files) => `Wiederherstellungsschlüssel hinzugefügt. Neu verschlüsselte Dateien: ${files}.`,
    attachmentExternalOpenUnavailable: "Externes Öffnen ist bei verschlüsselten Vaults deaktiviert, damit keine Klartextkopie auf dem Datenträger entsteht.",
  },
  es: {
    createEncrypted: "Cifrar esta bóveda",
    createEncryptedHelp:
      "Las notas se guardan cifradas (age) en el disco. La clave privada solo se conserva en el almacén de secretos del sistema de este dispositivo.",
    recoveryKey: "Clave pública de recuperación (opcional)",
    recoveryKeyPlaceholder: "age1…",
    recoveryKeyHelp: "Una segunda clave que también puede abrir la bóveda si se pierde la principal.",
    backupTitle: "Guarde la clave de la bóveda",
    backupText:
      "Esta es la clave privada. Se muestra una sola vez y no se puede recuperar. Sin ella, la bóveda cifrada no se puede abrir en otro dispositivo. Guárdela en un gestor de contraseñas o sin conexión.",
    copy: "Copiar",
    copied: "Copiado",
    backupConfirm: "He guardado la clave",
    backupContinue: "Continuar",
    importTitle: "Se necesita la clave de la bóveda",
    importText:
      "Esta bóveda está cifrada y este dispositivo no tiene su clave. Pegue la clave privada (AGE-SECRET-KEY-…).",
    importPlaceholder: "AGE-SECRET-KEY-…",
    importButton: "Importar y abrir",
    importing: "Comprobando la clave…",
    badge: "Cifrada",
    badgeTitle: "Las notas se guardan cifradas (age) en el disco",
    manageRecovery: "Gestionar clave de recuperación…",
    manageRecoveryTitle: "Añadir una clave de recuperación",
    manageRecoveryText: "Añade otra clave pública age. MD Notes volverá a cifrar los archivos para todos los destinatarios; no pegues aquí la clave privada de recuperación.",
    addRecovery: "Añadir clave",
    addingRecovery: "Volviendo a cifrar…",
    recoveryAdded: (files) => `Clave de recuperación añadida. Archivos recifrados: ${files}.`,
    attachmentExternalOpenUnavailable: "La apertura externa está desactivada en bóvedas cifradas para no crear una copia en texto plano en el disco.",
  },
  ko: {
    createEncrypted: "보관함 암호화",
    createEncryptedHelp:
      "노트는 암호화(age)되어 디스크에 저장됩니다. 개인 키는 이 기기의 시스템 비밀 저장소에만 보관됩니다.",
    recoveryKey: "복구용 공개 키 (선택)",
    recoveryKeyPlaceholder: "age1…",
    recoveryKeyHelp: "기본 키를 잃어버렸을 때 보관함을 열 수 있는 두 번째 키입니다.",
    backupTitle: "보관함 키 백업",
    backupText:
      "이것은 개인 키입니다. 한 번만 표시되며 복구할 수 없습니다. 키가 없으면 다른 기기에서 암호화된 보관함을 열 수 없습니다. 비밀번호 관리자나 오프라인에 보관하세요.",
    copy: "복사",
    copied: "복사됨",
    backupConfirm: "키를 저장했습니다",
    backupContinue: "계속",
    importTitle: "보관함 키 필요",
    importText: "이 보관함은 암호화되어 있고 이 기기에는 키가 없습니다. 개인 키(AGE-SECRET-KEY-…)를 붙여넣으세요.",
    importPlaceholder: "AGE-SECRET-KEY-…",
    importButton: "가져와서 열기",
    importing: "키 확인 중…",
    badge: "암호화됨",
    badgeTitle: "노트는 암호화(age)되어 디스크에 저장됩니다",
    manageRecovery: "복구 키 관리…",
    manageRecoveryTitle: "복구 키 추가",
    manageRecoveryText: "추가 공개 age 키를 등록합니다. MD Notes가 모든 recipient용으로 파일을 다시 암호화합니다. 개인 recovery 키는 여기에 붙여 넣지 마세요.",
    addRecovery: "키 추가",
    addingRecovery: "다시 암호화 중…",
    recoveryAdded: (files) => `복구 키를 추가했습니다. 다시 암호화한 파일: ${files}개.`,
    attachmentExternalOpenUnavailable: "암호화 vault에서는 디스크에 평문 복사본을 만들지 않도록 외부 열기를 비활성화합니다.",
  },
  ja: {
    createEncrypted: "保管庫を暗号化",
    createEncryptedHelp:
      "ノートは暗号化（age）されてディスクに保存されます。秘密鍵はこの端末のシステムのシークレットストアにのみ保管されます。",
    recoveryKey: "復旧用の公開鍵（任意）",
    recoveryKeyPlaceholder: "age1…",
    recoveryKeyHelp: "メインの鍵を失った場合でも保管庫を開ける二つ目の鍵です。",
    backupTitle: "保管庫の鍵をバックアップ",
    backupText:
      "これは秘密鍵です。一度だけ表示され、復元できません。鍵がないと、別の端末で暗号化された保管庫を開けません。パスワードマネージャーまたはオフラインで保管してください。",
    copy: "コピー",
    copied: "コピーしました",
    backupConfirm: "鍵を保存しました",
    backupContinue: "続ける",
    importTitle: "保管庫の鍵が必要です",
    importText: "この保管庫は暗号化されており、この端末には鍵がありません。秘密鍵（AGE-SECRET-KEY-…）を貼り付けてください。",
    importPlaceholder: "AGE-SECRET-KEY-…",
    importButton: "インポートして開く",
    importing: "鍵を確認中…",
    badge: "暗号化済み",
    badgeTitle: "ノートは暗号化（age）されてディスクに保存されます",
    manageRecovery: "復旧キーを管理…",
    manageRecoveryTitle: "復旧キーを追加",
    manageRecoveryText: "別の公開 age キーを追加します。MD Notes は全 recipient 向けにファイルを再暗号化します。秘密の recovery key はここに貼り付けないでください。",
    addRecovery: "キーを追加",
    addingRecovery: "再暗号化中…",
    recoveryAdded: (files) => `復旧キーを追加しました。再暗号化したファイル: ${files}。`,
    attachmentExternalOpenUnavailable: "暗号化 vault では、ディスクに平文コピーを作らないため外部アプリでの表示を無効にしています。",
  },
};
