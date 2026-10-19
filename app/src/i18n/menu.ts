import type { LanguageCode } from "./index";

export interface MenuStrings {
  file: string;
  quit: string;
}

export const MENU_STRINGS: Record<LanguageCode, MenuStrings> = {
  uk: { file: "Файл", quit: "Вийти" },
  en: { file: "File", quit: "Quit" },
  fr: { file: "Fichier", quit: "Quitter" },
  de: { file: "Datei", quit: "Beenden" },
  es: { file: "Archivo", quit: "Salir" },
  ko: { file: "파일", quit: "종료" },
  ja: { file: "ファイル", quit: "終了" },
};
