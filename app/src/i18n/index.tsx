import { createContext, useContext, useEffect, type ReactNode } from "react";
import { useStoredState } from "../storage";
import { de } from "./de";
import { en } from "./en";
import { es } from "./es";
import { fr } from "./fr";
import { ja } from "./ja";
import { ko } from "./ko";
import { uk, type Strings } from "./uk";

export type { Strings };

export type LanguageCode = "uk" | "en" | "fr" | "de" | "es" | "ko" | "ja";

export interface Language {
  code: LanguageCode;
  /** Name of the language in that language. */
  nativeName: string;
  /** Description in Ukrainian, the canonical project language. */
  ukrainianDescription: string;
  flag: string;
}

/** Supported interface languages, in the order used across the project. */
export const LANGUAGES: readonly Language[] = [
  { code: "uk", nativeName: "Українська", ukrainianDescription: "Основна мова", flag: "🇺🇦" },
  { code: "en", nativeName: "English", ukrainianDescription: "Англійська", flag: "🇬🇧" },
  { code: "fr", nativeName: "Français", ukrainianDescription: "Французька", flag: "🇫🇷" },
  { code: "de", nativeName: "Deutsch", ukrainianDescription: "Німецька", flag: "🇩🇪" },
  { code: "es", nativeName: "Español", ukrainianDescription: "Іспанська", flag: "🇪🇸" },
  { code: "ko", nativeName: "한국어", ukrainianDescription: "Корейська", flag: "🇰🇷" },
  { code: "ja", nativeName: "日本語", ukrainianDescription: "Японська", flag: "🇯🇵" },
];

const PACKS: Record<LanguageCode, Strings> = { uk, en, fr, de, es, ko, ja };

interface I18n {
  language: LanguageCode;
  setLanguage: (code: LanguageCode) => void;
  t: Strings;
}

const I18nContext = createContext<I18n>({ language: "uk", setLanguage: () => {}, t: uk });

const isLanguage = (code: string): code is LanguageCode => code in PACKS;

/** Ukrainian is the default; the user's choice is remembered. */
export function I18nProvider({ children }: { children: ReactNode }) {
  const [stored, setStored] = useStoredState<string>("mdnotes.language", "uk");
  const language: LanguageCode = isLanguage(stored) ? stored : "uk";

  useEffect(() => {
    document.documentElement.lang = language;
  }, [language]);

  return (
    <I18nContext.Provider value={{ language, setLanguage: setStored, t: PACKS[language] }}>
      {children}
    </I18nContext.Provider>
  );
}

export function useI18n(): I18n {
  return useContext(I18nContext);
}
