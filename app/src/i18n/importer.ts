import type { LanguageCode } from "./index";

export interface ImporterStrings {
  button: string;
  pickTitle: string;
  result: (created: number, updated: number, skipped: number, rejected: number) => string;
  overwritePrompt: (count: number) => string;
  /** Extra sentence for notes folded into existing ones by identity code (ADR-0008). */
  merged: (merged: number, conflicts: number) => string;
  /** Menu item and dialog texts for the read-only list of possible duplicates (ADR-0008). */
  duplicatesButton: string;
  duplicatesTitle: string;
  duplicatesNone: string;
  duplicatesFound: (groups: number) => string;
  duplicatesHint: string;
  duplicatesSameCode: string;
}

export const IMPORTER_STRINGS: Record<LanguageCode, ImporterStrings> = {
  uk: {
    button: "Імпортувати папку…",
    pickTitle: "Виберіть папку з нотатками для імпорту",
    result: (created, updated, skipped, rejected) =>
      `Імпорт: додано ${created}, оновлено ${updated}, пропущено ${skipped} (вже є), відхилено ${rejected}. Вихідну папку можна видалити.`,
    overwritePrompt: (count) => `${count} файлів уже є в сховищі. Замінити ті, що відрізняються, новішими версіями з папки?`,
    merged: (merged, conflicts) => `Об'єднано за кодом: ${merged}${conflicts > 0 ? `, розбіжностей для вичитки: ${conflicts}` : ""}.`,
    duplicatesButton: "Можливі дублікати…",
    duplicatesTitle: "Можливі дублікати",
    duplicatesNone: "Схожих за назвою нотаток не знайдено.",
    duplicatesFound: (groups) => `Знайдено груп: ${groups}`,
    duplicatesHint: "Нічого не змінено: об'єднайте нотатки вручну або імпортом із кодом.",
    duplicatesSameCode: "той самий код",
  },
  en: {
    button: "Import folder…",
    pickTitle: "Choose a folder of notes to import",
    result: (created, updated, skipped, rejected) =>
      `Import: ${created} added, ${updated} updated, ${skipped} skipped (already there), ${rejected} rejected. You can delete the source folder.`,
    overwritePrompt: (count) => `${count} files are already in the vault. Replace the ones that differ with the versions from the folder?`,
    merged: (merged, conflicts) => `Merged by code: ${merged}${conflicts > 0 ? `, conflicts to review: ${conflicts}` : ""}.`,
    duplicatesButton: "Possible duplicates…",
    duplicatesTitle: "Possible duplicates",
    duplicatesNone: "No notes with the same name were found.",
    duplicatesFound: (groups) => `${groups} groups`,
    duplicatesHint: "Nothing was changed: merge the notes by hand or import them with a code.",
    duplicatesSameCode: "same code",
  },
  fr: {
    button: "Importer un dossier…",
    pickTitle: "Choisissez un dossier de notes à importer",
    result: (created, updated, skipped, rejected) =>
      `Import : ${created} ajoutés, ${updated} mis à jour, ${skipped} ignorés (déjà présents), ${rejected} refusés. Vous pouvez supprimer le dossier source.`,
    overwritePrompt: (count) => `${count} fichiers existent déjà dans le coffre. Remplacer ceux qui diffèrent par les versions du dossier ?`,
    merged: (merged, conflicts) => `Fusionnées par code: ${merged}${conflicts > 0 ? `, conflits à relire: ${conflicts}` : ""}.`,
    duplicatesButton: "Doublons possibles…",
    duplicatesTitle: "Doublons possibles",
    duplicatesNone: "Aucune note au nom identique trouvée.",
    duplicatesFound: (groups) => `${groups} groupes`,
    duplicatesHint: "Rien n'a été modifié : fusionnez les notes à la main ou importez-les avec un code.",
    duplicatesSameCode: "même code",
  },
  de: {
    button: "Ordner importieren…",
    pickTitle: "Ordner mit Notizen zum Import wählen",
    result: (created, updated, skipped, rejected) =>
      `Import: ${created} hinzugefügt, ${updated} aktualisiert, ${skipped} übersprungen (schon vorhanden), ${rejected} abgelehnt. Der Quellordner kann gelöscht werden.`,
    overwritePrompt: (count) => `${count} Dateien sind schon im Tresor. Abweichende durch die Versionen aus dem Ordner ersetzen?`,
    merged: (merged, conflicts) => `Per Code zusammengeführt: ${merged}${conflicts > 0 ? `, Konflikte zur Prüfung: ${conflicts}` : ""}.`,
    duplicatesButton: "Mögliche Duplikate…",
    duplicatesTitle: "Mögliche Duplikate",
    duplicatesNone: "Keine Notizen mit gleichem Namen gefunden.",
    duplicatesFound: (groups) => `${groups} Gruppen`,
    duplicatesHint: "Nichts wurde geändert: Notizen von Hand zusammenführen oder mit Code importieren.",
    duplicatesSameCode: "gleicher Code",
  },
  es: {
    button: "Importar carpeta…",
    pickTitle: "Elija una carpeta de notas para importar",
    result: (created, updated, skipped, rejected) =>
      `Importación: ${created} añadidos, ${updated} actualizados, ${skipped} omitidos (ya existen), ${rejected} rechazados. Puede borrar la carpeta de origen.`,
    overwritePrompt: (count) => `${count} archivos ya están en la bóveda. ¿Reemplazar los que difieren por las versiones de la carpeta?`,
    merged: (merged, conflicts) => `Combinadas por código: ${merged}${conflicts > 0 ? `, conflictos por revisar: ${conflicts}` : ""}.`,
    duplicatesButton: "Posibles duplicados…",
    duplicatesTitle: "Posibles duplicados",
    duplicatesNone: "No se encontraron notas con el mismo nombre.",
    duplicatesFound: (groups) => `${groups} grupos`,
    duplicatesHint: "No se cambió nada: combine las notas a mano o impórtelas con un código.",
    duplicatesSameCode: "mismo código",
  },
  ko: {
    button: "폴더 가져오기…",
    pickTitle: "가져올 노트 폴더를 선택하세요",
    result: (created, updated, skipped, rejected) =>
      `가져오기: ${created}개 추가, ${updated}개 업데이트, ${skipped}개 건너뜀(이미 있음), ${rejected}개 거부됨. 원본 폴더는 삭제해도 됩니다.`,
    overwritePrompt: (count) => `${count}개 파일이 이미 보관함에 있습니다. 내용이 다른 파일을 폴더의 버전으로 바꿀까요?`,
    merged: (merged, conflicts) => `코드로 병합: ${merged}${conflicts > 0 ? `, 검토할 충돌: ${conflicts}` : ""}.`,
    duplicatesButton: "중복 가능성…",
    duplicatesTitle: "중복 가능성",
    duplicatesNone: "이름이 같은 노트를 찾지 못했습니다.",
    duplicatesFound: (groups) => `${groups} 그룹`,
    duplicatesHint: "아무것도 변경되지 않았습니다. 노트를 직접 병합하거나 코드와 함께 가져오세요.",
    duplicatesSameCode: "같은 코드",
  },
  ja: {
    button: "フォルダーをインポート…",
    pickTitle: "インポートするノートのフォルダーを選択",
    result: (created, updated, skipped, rejected) =>
      `インポート: ${created} 件追加、${updated} 件更新、${skipped} 件スキップ（既存）、${rejected} 件拒否。元のフォルダーは削除できます。`,
    overwritePrompt: (count) => `${count} 件のファイルがすでに保管庫にあります。内容が異なるものをフォルダー内の版で置き換えますか？`,
    merged: (merged, conflicts) => `コードで統合: ${merged}${conflicts > 0 ? `、確認が必要な競合: ${conflicts}` : ""}.`,
    duplicatesButton: "重複の可能性…",
    duplicatesTitle: "重複の可能性",
    duplicatesNone: "同じ名前のノートは見つかりませんでした。",
    duplicatesFound: (groups) => `${groups} グループ`,
    duplicatesHint: "何も変更していません。ノートを手動で統合するか、コード付きで取り込んでください。",
    duplicatesSameCode: "同じコード",
  },
};
