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

export interface RegistryStrings {
  title: string;
  noDir: string;
  pick: string;
  change: string;
  none: string;
  registered: string;
  state: string;
  form: string;
  founders: string;
  beneficiaries: string;
  amcu: string;
  appealed: string;
  tenders: string;
  asBuyer: string;
  asWinner: string;
  latest: string;
  buyer: string;
  winner: string;
  note: string;
}

export const REGISTRY_STRINGS: Record<LanguageCode, RegistryStrings> = {
  uk: {
    title: "База «З-під Ляди» за кодом",
    noDir: "Вкажіть папку з відкритими даними сайту (локальна копія гілки data): тоді тут з’являться факти за цим кодом. Нічого не надсилається в мережу.",
    pick: "Вибрати папку даних…",
    change: "Змінити папку",
    none: "У даних за цим кодом нічого немає.",
    registered: "Зареєстровано",
    state: "Стан",
    form: "Форма",
    founders: "Іноземні країни засновників",
    beneficiaries: "Іноземні країни бенефіціарів",
    amcu: "Рішення АМКУ про спотворення торгів",
    appealed: "оскаржувалося",
    tenders: "Закупівлі (Prozorro)",
    asBuyer: "Як замовник",
    asWinner: "Як переможець",
    latest: "Останні",
    buyer: "замовник",
    winner: "переможець",
    note: "Факти з відкритих наборів (ЄДР, АМКУ, Prozorro) за точним кодом; це не оцінки й не доказ порушення.",
  },
  en: {
    title: "“З-під Ляди” database by code",
    noDir: "Choose the folder with the site’s open data (a local copy of the data branch) and the facts for this code will appear here. Nothing is sent over the network.",
    pick: "Choose data folder…",
    change: "Change folder",
    none: "The data has nothing for this code.",
    registered: "Registered",
    state: "State",
    form: "Form",
    founders: "Foreign countries of founders",
    beneficiaries: "Foreign countries of beneficiaries",
    amcu: "Competition authority decisions on distorted tenders",
    appealed: "appealed",
    tenders: "Procurements (Prozorro)",
    asBuyer: "As buyer",
    asWinner: "As winner",
    latest: "Latest",
    buyer: "buyer",
    winner: "winner",
    note: "Facts from open datasets (state register, competition authority, Prozorro) by exact code; they are not an assessment or proof of a violation.",
  },
  fr: {
    title: "Base « З-під Ляди » par code",
    noDir: "Choisissez le dossier des données ouvertes du site (copie locale de la branche data) : les faits pour ce code apparaîtront ici. Rien n’est envoyé sur le réseau.",
    pick: "Choisir le dossier de données…",
    change: "Changer de dossier",
    none: "Les données n’ont rien pour ce code.",
    registered: "Enregistrée",
    state: "État",
    form: "Forme",
    founders: "Pays étrangers des fondateurs",
    beneficiaries: "Pays étrangers des bénéficiaires",
    amcu: "Décisions de l’autorité de la concurrence sur les offres faussées",
    appealed: "contestée",
    tenders: "Marchés (Prozorro)",
    asBuyer: "Comme acheteur",
    asWinner: "Comme gagnant",
    latest: "Derniers",
    buyer: "acheteur",
    winner: "gagnant",
    note: "Faits issus de jeux de données ouverts (registre, autorité de la concurrence, Prozorro) par code exact ; ce n’est ni une évaluation ni la preuve d’une infraction.",
  },
  de: {
    title: "Datenbank „З-під Ляди“ nach Code",
    noDir: "Wählen Sie den Ordner mit den offenen Daten der Seite (lokale Kopie des Zweigs data); dann erscheinen hier die Fakten zu diesem Code. Nichts wird über das Netzwerk gesendet.",
    pick: "Datenordner wählen…",
    change: "Ordner ändern",
    none: "Die Daten enthalten nichts zu diesem Code.",
    registered: "Registriert",
    state: "Status",
    form: "Form",
    founders: "Ausländische Länder der Gründer",
    beneficiaries: "Ausländische Länder der wirtschaftlich Berechtigten",
    amcu: "Entscheidungen der Wettbewerbsbehörde zu verzerrten Ausschreibungen",
    appealed: "angefochten",
    tenders: "Vergaben (Prozorro)",
    asBuyer: "Als Auftraggeber",
    asWinner: "Als Gewinner",
    latest: "Neueste",
    buyer: "Auftraggeber",
    winner: "Gewinner",
    note: "Fakten aus offenen Datensätzen (Register, Wettbewerbsbehörde, Prozorro) zum genauen Code; keine Bewertung und kein Beleg für einen Verstoß.",
  },
  es: {
    title: "Base «З-під Ляди» por código",
    noDir: "Elija la carpeta con los datos abiertos del sitio (copia local de la rama data) y aquí aparecerán los hechos de este código. No se envía nada por la red.",
    pick: "Elegir carpeta de datos…",
    change: "Cambiar carpeta",
    none: "Los datos no tienen nada para este código.",
    registered: "Registrada",
    state: "Estado",
    form: "Forma",
    founders: "Países extranjeros de los fundadores",
    beneficiaries: "Países extranjeros de los beneficiarios",
    amcu: "Decisiones de la autoridad de competencia sobre licitaciones distorsionadas",
    appealed: "recurrida",
    tenders: "Contrataciones (Prozorro)",
    asBuyer: "Como comprador",
    asWinner: "Como ganador",
    latest: "Últimas",
    buyer: "comprador",
    winner: "ganador",
    note: "Hechos de conjuntos de datos abiertos (registro, autoridad de competencia, Prozorro) por código exacto; no son una valoración ni prueba de una infracción.",
  },
  ko: {
    title: "코드로 보는 “З-під Ляди” 데이터",
    noDir: "사이트의 공개 데이터 폴더(data 브랜치의 로컬 사본)를 선택하면 이 코드의 사실이 여기에 표시됩니다. 네트워크로는 아무것도 전송되지 않습니다.",
    pick: "데이터 폴더 선택…",
    change: "폴더 변경",
    none: "이 코드에 대한 데이터가 없습니다.",
    registered: "등록일",
    state: "상태",
    form: "형태",
    founders: "창립자의 외국 국가",
    beneficiaries: "실소유자의 외국 국가",
    amcu: "입찰 왜곡에 관한 경쟁당국 결정",
    appealed: "이의 제기됨",
    tenders: "조달 (Prozorro)",
    asBuyer: "발주자로서",
    asWinner: "낙찰자로서",
    latest: "최근",
    buyer: "발주자",
    winner: "낙찰자",
    note: "공개 데이터(등기부, 경쟁당국, Prozorro)에서 정확한 코드로 가져온 사실이며 평가나 위반의 증거가 아닙니다.",
  },
  ja: {
    title: "コードで見る「З-під Ляди」データ",
    noDir: "サイトの公開データのフォルダ（data ブランチのローカルコピー）を選ぶと、このコードの事実がここに表示されます。ネットワークには何も送信されません。",
    pick: "データフォルダを選ぶ…",
    change: "フォルダを変更",
    none: "このコードに関するデータはありません。",
    registered: "登録日",
    state: "状態",
    form: "形態",
    founders: "創設者の外国の国",
    beneficiaries: "受益者の外国の国",
    amcu: "入札の歪曲に関する競争当局の決定",
    appealed: "不服申立てあり",
    tenders: "調達（Prozorro）",
    asBuyer: "発注者として",
    asWinner: "落札者として",
    latest: "最新",
    buyer: "発注者",
    winner: "落札者",
    note: "公開データセット（登記、競争当局、Prozorro）から正確なコードで得た事実であり、評価や違反の証拠ではありません。",
  },
};
