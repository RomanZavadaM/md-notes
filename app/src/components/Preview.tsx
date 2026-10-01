import Markdown, { defaultUrlTransform } from "react-markdown";
import remarkGfm from "remark-gfm";
import { openUrl } from "@tauri-apps/plugin-opener";

const WIKI_SCHEME = "wikilink:";
const FRONT_MATTER = /^﻿?---\r?\n(?:([\s\S]*?)\r?\n)?(?:---|\.\.\.)[ \t]*(?:\r?\n|$)/;
// Code is kept as is: fenced blocks, then inline code spans.
const CODE = /(```[\s\S]*?(?:```|$)|~~~[\s\S]*?(?:~~~|$)|`[^`\n]*`)/;
const WIKILINK = /!?\[\[([^[\]\n]+?)\]\]/g;

export function splitFrontMatter(content: string): { frontMatter: string | null; body: string } {
  const match = FRONT_MATTER.exec(content);
  if (!match) return { frontMatter: null, body: content };
  return { frontMatter: match[1] ?? "", body: content.slice(match[0].length) };
}

/** Turns `[[Note|Alias]]` into regular links with the `wikilink:` scheme. */
function wikiLinksToMarkdown(src: string): string {
  return src
    .split(CODE)
    .map((part, i) =>
      i % 2 === 1
        ? part
        : part.replace(WIKILINK, (_match, inner: string) => {
            const [link, alias] = inner.split("|", 2);
            const target = link.trim();
            const label = (alias ?? link).trim().replace(/[[\]]/g, "") || target;
            return `[${label}](<${WIKI_SCHEME}${encodeURIComponent(target)}>)`;
          }),
    )
    .join("");
}

interface Props {
  content: string;
  onOpenLink: (target: string) => void;
}

export function Preview({ content, onOpenLink }: Props) {
  const { frontMatter, body } = splitFrontMatter(content);
  return (
    <article className="preview">
      {frontMatter !== null && frontMatter.trim() !== "" && (
        <details className="front-matter">
          <summary>Властивості</summary>
          <pre>{frontMatter}</pre>
        </details>
      )}
      <Markdown
        remarkPlugins={[remarkGfm]}
        urlTransform={(url) => (url.startsWith(WIKI_SCHEME) ? url : defaultUrlTransform(url))}
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
            return (
              <a
                href={href}
                onClick={(e) => {
                  e.preventDefault();
                  if (href && /^(https?|mailto):/i.test(href)) void openUrl(href);
                }}
              >
                {children}
              </a>
            );
          },
        }}
      >
        {wikiLinksToMarkdown(body)}
      </Markdown>
    </article>
  );
}
