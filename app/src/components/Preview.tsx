import { useEffect, useId, useState } from "react";
import Markdown, { defaultUrlTransform } from "react-markdown";
import rehypeKatex from "rehype-katex";
import remarkGfm from "remark-gfm";
import remarkMath from "remark-math";
import { convertFileSrc } from "@tauri-apps/api/core";
import { openUrl } from "@tauri-apps/plugin-opener";
import "katex/dist/katex.min.css";
import { parentPath } from "../api";
import { useI18n } from "../i18n";

const WIKI_SCHEME = "wikilink:";
const EMBED_SCHEME = "wikiembed:";
const FRONT_MATTER = /^﻿?---\r?\n(?:([\s\S]*?)\r?\n)?(?:---|\.\.\.)[ \t]*(?:\r?\n|$)/;
// Code is kept as is: fenced blocks, then inline code spans.
const CODE = /(```[\s\S]*?(?:```|$)|~~~[\s\S]*?(?:~~~|$)|`[^`\n]*`)/;
const WIKILINK = /(!?)\[\[([^[\]\n]+?)\]\]/g;
const IMAGE = /\.(png|jpe?g|gif|webp|svg|bmp|avif)$/i;
const NOTE = /\.(md|markdown)$/i;

export function splitFrontMatter(content: string): { frontMatter: string | null; body: string } {
  const match = FRONT_MATTER.exec(content);
  if (!match) return { frontMatter: null, body: content };
  return { frontMatter: match[1] ?? "", body: content.slice(match[0].length) };
}

/** Turns `[[Note|Alias]]` into links and `![[image.png]]` into images. */
function wikiLinksToMarkdown(src: string): string {
  return src
    .split(CODE)
    .map((part, i) =>
      i % 2 === 1
        ? part
        : part.replace(WIKILINK, (_match, bang: string, inner: string) => {
            const [link, alias] = inner.split("|", 2);
            const target = link.replace(/\\$/, "").trim();
            const label = (alias ?? link).trim().replace(/[[\]\\]/g, "") || target;
            if (bang && IMAGE.test(target)) {
              return `![${alias ? label : ""}](<${EMBED_SCHEME}${encodeURIComponent(target)}>)`;
            }
            return `[${label}](<${WIKI_SCHEME}${encodeURIComponent(target)}>)`;
          }),
    )
    .join("");
}

const isExternal = (url: string) => /^[a-z][a-z0-9+.-]*:/i.test(url) || url.startsWith("//");

/** Resolves a relative link against the note's folder; `null` if it leaves the vault. */
function resolveRelative(notePath: string, href: string): string | null {
  let clean: string;
  try {
    clean = decodeURIComponent(href.split(/[?#]/)[0]);
  } catch {
    return null;
  }
  const parts = clean.startsWith("/") ? [] : parentPath(notePath).split("/").filter(Boolean);
  for (const segment of clean.split("/")) {
    if (segment === "" || segment === ".") continue;
    if (segment === "..") {
      if (parts.length === 0) return null;
      parts.pop();
    } else {
      parts.push(segment);
    }
  }
  return parts.join("/");
}

function isDarkTheme(): boolean {
  const theme = document.documentElement.dataset.theme;
  if (theme) return theme === "dark";
  return window.matchMedia("(prefers-color-scheme: dark)").matches;
}

function MermaidDiagram({ code }: { code: string }) {
  const id = `mermaid-${useId().replace(/[^a-zA-Z0-9]/g, "")}`;
  const [svg, setSvg] = useState("");
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;
    // Loaded on demand: Mermaid is large and most notes have no diagrams.
    import("mermaid")
      .then(async ({ default: mermaid }) => {
        mermaid.initialize({
          startOnLoad: false,
          securityLevel: "strict",
          theme: isDarkTheme() ? "dark" : "default",
        });
        const result = await mermaid.render(id, code);
        if (!cancelled) {
          setSvg(result.svg);
          setError(null);
        }
      })
      .catch((e) => {
        if (!cancelled) setError(String(e));
      });
    return () => {
      cancelled = true;
    };
  }, [code, id]);

  if (error) return <pre className="mermaid-error">{error}</pre>;
  return <div className="mermaid" dangerouslySetInnerHTML={{ __html: svg }} />;
}

interface Props {
  content: string;
  /** Vault-relative path of the note, for relative links and images. */
  notePath: string;
  /** Absolute path of the vault folder. */
  vaultRoot: string;
  /** Vault-relative paths of all files, for `![[image.png]]` embeds. */
  files: string[];
  onOpenLink: (target: string) => void;
  onOpenPath: (path: string) => void;
}

export function Preview({ content, notePath, vaultRoot, files, onOpenLink, onOpenPath }: Props) {
  const { t } = useI18n();
  const { frontMatter, body } = splitFrontMatter(content);
  const sep = vaultRoot.includes("\\") ? "\\" : "/";
  const assetUrl = (rel: string) => convertFileSrc(`${vaultRoot}${sep}${rel.split("/").join(sep)}`);

  const findFile = (name: string): string | undefined => {
    const wanted = name.toLowerCase();
    return (
      files.find((f) => f.toLowerCase() === wanted) ??
      files.find((f) => f.toLowerCase().endsWith(`/${wanted}`))
    );
  };

  const imageSrc = (src: string): string | undefined => {
    if (src.startsWith(EMBED_SCHEME)) {
      const found = findFile(decodeURIComponent(src.slice(EMBED_SCHEME.length)));
      return found ? assetUrl(found) : undefined;
    }
    if (isExternal(src)) return src;
    const rel = resolveRelative(notePath, src);
    return rel ? assetUrl(rel) : undefined;
  };

  return (
    <article className="preview">
      {frontMatter !== null && frontMatter.trim() !== "" && (
        <details className="front-matter">
          <summary>{t.properties}</summary>
          <pre>{frontMatter}</pre>
        </details>
      )}
      <Markdown
        remarkPlugins={[remarkGfm, remarkMath]}
        rehypePlugins={[rehypeKatex]}
        urlTransform={(url) =>
          url.startsWith(WIKI_SCHEME) || url.startsWith(EMBED_SCHEME) ? url : defaultUrlTransform(url)
        }
        components={{
          a: ({ href, children }) => {
            if (href?.startsWith(WIKI_SCHEME)) {
              const target = decodeURIComponent(href.slice(WIKI_SCHEME.length));
              return (
                <a
                  href="#"
                  className="wikilink"
                  title={target}
                  onClick={(e) => {
                    e.preventDefault();
                    onOpenLink(target);
                  }}
                >
                  {children}
                </a>
              );
            }
            if (href?.startsWith("#")) return <a href={href}>{children}</a>;
            return (
              <a
                href={href}
                onClick={(e) => {
                  e.preventDefault();
                  if (!href) return;
                  if (/^(https?|mailto):/i.test(href)) {
                    void openUrl(href);
                    return;
                  }
                  const rel = isExternal(href) ? null : resolveRelative(notePath, href);
                  if (rel && NOTE.test(rel)) onOpenPath(rel);
                }}
              >
                {children}
              </a>
            );
          },
          img: ({ src, alt, title }) => (
            <img src={typeof src === "string" ? imageSrc(src) : undefined} alt={alt ?? ""} title={title} loading="lazy" />
          ),
          code: ({ className, children }) => {
            if (className?.includes("language-mermaid")) {
              return <MermaidDiagram code={String(children).replace(/\n$/, "")} />;
            }
            return <code className={className}>{children}</code>;
          },
        }}
      >
        {wikiLinksToMarkdown(body)}
      </Markdown>
    </article>
  );
}
