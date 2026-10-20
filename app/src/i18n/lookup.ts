import type { LanguageCode } from "./index";

export interface LookupStrings {
  exact: string;
  candidates: string;
  candidateNote: string;
  inText: string;
  codeBadge: string;
  nameBadge: string;
}

export const LOOKUP_STRINGS: Record<LanguageCode, LookupStrings> = {
  uk: {
    exact: "Точний збіг",
    candidates: "Кандидати за назвою",
    candidateNote: "збіг за назвою не доказ: підтвердіть кодом чи документом",
    inText: "У тексті",
    codeBadge: "код",
    nameBadge: "назва",
  },
  en: {
    exact: "Exact match",
    candidates: "Candidates by name",
    candidateNote: "a name match is not proof: confirm with a code or a document",
    inText: "In text",
    codeBadge: "code",
    nameBadge: "name",
  },
  fr: {
    exact: "Correspondance exacte",
    candidates: "Candidats par nom",
    candidateNote: "un nom identique ne prouve rien : confirmez par un code ou un document",
    inText: "Dans le texte",
    codeBadge: "code",
    nameBadge: "nom",
  },
  de: {
    exact: "Exakter Treffer",
    candidates: "Kandidaten nach Name",
    candidateNote: "ein Namenstreffer ist kein Beweis: mit Code oder Dokument bestätigen",
    inText: "Im Text",
    codeBadge: "Code",
    nameBadge: "Name",
  },
  es: {
    exact: "Coincidencia exacta",
    candidates: "Candidatos por nombre",
    candidateNote: "un nombre igual no es prueba: confirme con un código o documento",
    inText: "En el texto",
    codeBadge: "código",
    nameBadge: "nombre",
  },
  ko: {
    exact: "정확히 일치",
    candidates: "이름 후보",
    candidateNote: "이름이 같다고 같은 대상은 아닙니다. 코드나 문서로 확인하세요",
    inText: "본문",
    codeBadge: "코드",
    nameBadge: "이름",
  },
  ja: {
    exact: "完全一致",
    candidates: "名前の候補",
    candidateNote: "名前の一致は証拠ではありません。コードや文書で確認してください",
    inText: "本文",
    codeBadge: "コード",
    nameBadge: "名前",
  },
};
