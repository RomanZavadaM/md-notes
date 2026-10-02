import type { LanguageCode } from "./index";

export interface GraphStrings {
  linksMode: string;
  localMode: string;
  globalMode: string;
  localTitle: string;
  globalTitle: string;
  loading: string;
  empty: string;
  nodes: (count: number) => string;
  edges: (count: number) => string;
  openHint: string;
}

export const GRAPH_STRINGS: Record<LanguageCode, GraphStrings> = {
  uk: {
    linksMode: "Зв’язки",
    localMode: "Локальний граф",
    globalMode: "Глобальний граф",
    localTitle: "Зв’язки активної нотатки",
    globalTitle: "Граф сховища",
    loading: "Побудова графа…",
    empty: "У цьому режимі ще немає зв’язаних нотаток.",
    nodes: (count) => `Нотаток: ${count}`,
    edges: (count) => `Зв’язків: ${count}`,
    openHint: "Натисніть вузол, щоб відкрити нотатку.",
  },
  en: {
    linksMode: "Links",
    localMode: "Local graph",
    globalMode: "Global graph",
    localTitle: "Active note connections",
    globalTitle: "Vault graph",
    loading: "Building graph…",
    empty: "There are no connected notes in this view yet.",
    nodes: (count) => `Notes: ${count}`,
    edges: (count) => `Links: ${count}`,
    openHint: "Click a node to open the note.",
  },
  fr: {
    linksMode: "Liens",
    localMode: "Graphe local",
    globalMode: "Graphe global",
    localTitle: "Connexions de la note active",
    globalTitle: "Graphe du coffre",
    loading: "Construction du graphe…",
    empty: "Aucune note liée dans cette vue pour le moment.",
    nodes: (count) => `Notes : ${count}`,
    edges: (count) => `Liens : ${count}`,
    openHint: "Cliquez sur un nœud pour ouvrir la note.",
  },
  de: {
    linksMode: "Verknüpfungen",
    localMode: "Lokaler Graph",
    globalMode: "Globaler Graph",
    localTitle: "Verknüpfungen der aktiven Notiz",
    globalTitle: "Tresor-Graph",
    loading: "Graph wird aufgebaut…",
    empty: "In dieser Ansicht gibt es noch keine verknüpften Notizen.",
    nodes: (count) => `Notizen: ${count}`,
    edges: (count) => `Verknüpfungen: ${count}`,
    openHint: "Klicken Sie auf einen Knoten, um die Notiz zu öffnen.",
  },
  es: {
    linksMode: "Enlaces",
    localMode: "Grafo local",
    globalMode: "Grafo global",
    localTitle: "Conexiones de la nota activa",
    globalTitle: "Grafo de la bóveda",
    loading: "Construyendo el grafo…",
    empty: "Todavía no hay notas conectadas en esta vista.",
    nodes: (count) => `Notas: ${count}`,
    edges: (count) => `Enlaces: ${count}`,
    openHint: "Haz clic en un nodo para abrir la nota.",
  },
  ko: {
    linksMode: "링크",
    localMode: "로컬 그래프",
    globalMode: "전체 그래프",
    localTitle: "현재 노트 연결",
    globalTitle: "보관함 그래프",
    loading: "그래프 만드는 중…",
    empty: "이 보기에는 아직 연결된 노트가 없습니다.",
    nodes: (count) => `노트: ${count}`,
    edges: (count) => `링크: ${count}`,
    openHint: "노드를 클릭하면 해당 노트를 엽니다.",
  },
  ja: {
    linksMode: "リンク",
    localMode: "ローカルグラフ",
    globalMode: "全体グラフ",
    localTitle: "現在のノートのつながり",
    globalTitle: "保管庫グラフ",
    loading: "グラフを構築中…",
    empty: "この表示にはまだ接続されたノートがありません。",
    nodes: (count) => `ノート: ${count}`,
    edges: (count) => `リンク: ${count}`,
    openHint: "ノードをクリックするとノートを開きます。",
  },
};
