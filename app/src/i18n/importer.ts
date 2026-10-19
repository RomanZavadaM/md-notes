import type { LanguageCode } from "./index";

export interface ImporterStrings {
  button: string;
  pickTitle: string;
  result: (created: number, skipped: number, rejected: number) => string;
}

export const IMPORTER_STRINGS: Record<LanguageCode, ImporterStrings> = {
  uk: {
    button: "Імпортувати папку…",
    pickTitle: "Виберіть папку з нотатками для імпорту",
    result: (created, skipped, rejected) =>
      `Імпорт: додано ${created}, пропущено ${skipped} (вже є), відхилено ${rejected}. Вихідну папку можна видалити.`,
  },
  en: {
    button: "Import folder…",
    pickTitle: "Choose a folder of notes to import",
    result: (created, skipped, rejected) =>
      `Import: ${created} added, ${skipped} skipped (already there), ${rejected} rejected. You can delete the source folder.`,
  },
  fr: {
    button: "Importer un dossier…",
    pickTitle: "Choisissez un dossier de notes à importer",
    result: (created, skipped, rejected) =>
      `Import : ${created} ajoutés, ${skipped} ignorés (déjà présents), ${rejected} refusés. Vous pouvez supprimer le dossier source.`,
  },
  de: {
    button: "Ordner importieren…",
    pickTitle: "Ordner mit Notizen zum Import wählen",
    result: (created, skipped, rejected) =>
      `Import: ${created} hinzugefügt, ${skipped} übersprungen (schon vorhanden), ${rejected} abgelehnt. Der Quellordner kann gelöscht werden.`,
  },
  es: {
    button: "Importar carpeta…",
    pickTitle: "Elija una carpeta de notas para importar",
    result: (created, skipped, rejected) =>
      `Importación: ${created} añadidos, ${skipped} omitidos (ya existen), ${rejected} rechazados. Puede borrar la carpeta de origen.`,
  },
  ko: {
    button: "폴더 가져오기…",
    pickTitle: "가져올 노트 폴더를 선택하세요",
    result: (created, skipped, rejected) =>
      `가져오기: ${created}개 추가, ${skipped}개 건너뜀(이미 있음), ${rejected}개 거부됨. 원본 폴더는 삭제해도 됩니다.`,
  },
  ja: {
    button: "フォルダーをインポート…",
    pickTitle: "インポートするノートのフォルダーを選択",
    result: (created, skipped, rejected) =>
      `インポート: ${created} 件追加、${skipped} 件スキップ（既存）、${rejected} 件拒否。元のフォルダーは削除できます。`,
  },
};
