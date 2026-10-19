// Node colours by note type. Okabe-Ito palette (colour-blind safe); the same
// colours are the single source of truth in the project's graph style spec.
// Unknown types get a stable colour derived from the type name, so a vault can
// use its own `type:` values without configuration.

const KNOWN: Record<string, string> = {
  person: "#E69F00",
  company: "#0072B2",
  institution: "#8E7CC3",
  group: "#CC79A7",
  asset: "#009E73",
  tender: "#56B4E9",
  market: "#F0E442",
  case: "#D55E00",
  event: "#999999",
  source: "#6B7280",
  material: "#A8AEB8",
  note: "#7C8DF0",
};

const FALLBACK = ["#E69F00", "#56B4E9", "#009E73", "#F0E442", "#0072B2", "#D55E00", "#CC79A7", "#999999"];
const UNTYPED = "#9AA1AD";

function hash(text: string): number {
  let h = 2166136261;
  for (let i = 0; i < text.length; i += 1) {
    h ^= text.charCodeAt(i);
    h = Math.imul(h, 16777619);
  }
  return h >>> 0;
}

/** Colour for a note type; `null` or empty means a note without a type. */
export function typeColor(type: string | null | undefined): string {
  if (!type) return UNTYPED;
  const known = KNOWN[type];
  if (known) return known;
  return FALLBACK[hash(type) % FALLBACK.length];
}

/** Node size from the number of connections (spec: 8 + 1.3 * min(n, 8)). */
export function nodeSize(degree: number): number {
  return 6 + 1.3 * Math.min(degree, 8) + (degree > 8 ? Math.min(Math.log2(degree - 7), 3) : 0);
}

/** True when the page background is dark (theme "dark" or system dark). */
export function isDarkBackground(): boolean {
  const colour = getComputedStyle(document.body).backgroundColor;
  const match = colour.match(/\d+(\.\d+)?/g);
  if (!match || match.length < 3) return false;
  const [r, g, b] = match.slice(0, 3).map(Number);
  return 0.2126 * r + 0.7152 * g + 0.0722 * b < 128;
}
