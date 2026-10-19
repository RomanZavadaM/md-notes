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
  },
};
