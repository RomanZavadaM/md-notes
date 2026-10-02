import { useEffect, useRef, useState } from "react";
import { EditorState } from "@codemirror/state";
import { EditorView, drawSelection, highlightActiveLine, keymap } from "@codemirror/view";
import { defaultKeymap, history, historyKeymap, indentWithTab } from "@codemirror/commands";
import { markdown, markdownLanguage } from "@codemirror/lang-markdown";
import { languages } from "@codemirror/language-data";
import { HighlightStyle, syntaxHighlighting } from "@codemirror/language";
import { tags as t } from "@lezer/highlight";
import { useI18n } from "../i18n";
import { PropertiesPanel } from "./PropertiesPanel";

const highlight = HighlightStyle.define([
  { tag: t.heading1, fontSize: "1.4em", fontWeight: "700", color: "var(--accent)" },
  { tag: t.heading2, fontSize: "1.2em", fontWeight: "700", color: "var(--accent)" },
  { tag: [t.heading3, t.heading4, t.heading5, t.heading6], fontWeight: "700", color: "var(--accent)" },
  { tag: t.strong, fontWeight: "700" },
  { tag: t.emphasis, fontStyle: "italic" },
  { tag: t.strikethrough, textDecoration: "line-through" },
  { tag: [t.link, t.url], color: "var(--link)" },
  { tag: t.monospace, fontFamily: "var(--mono)", color: "var(--code)" },
  { tag: t.quote, color: "var(--muted)", fontStyle: "italic" },
  { tag: [t.processingInstruction, t.meta, t.contentSeparator], color: "var(--muted)" },
  { tag: [t.keyword, t.operatorKeyword], color: "var(--syntax-keyword)" },
  { tag: [t.string, t.special(t.string)], color: "var(--syntax-string)" },
  { tag: [t.number, t.bool, t.null], color: "var(--syntax-number)" },
  { tag: [t.comment, t.lineComment, t.blockComment], color: "var(--muted)", fontStyle: "italic" },
  { tag: [t.function(t.variableName), t.typeName, t.className], color: "var(--syntax-name)" },
]);

interface Props {
  /** Changing the key recreates the editor (e.g. when another note opens). */
  docKey: string;
  value: string;
  onChange: (value: string) => void;
  onSave: () => void;
}

export function Editor({ docKey, value, onChange, onSave }: Props) {
  const { t: strings } = useI18n();
  const host = useRef<HTMLDivElement>(null);
  const viewRef = useRef<EditorView | null>(null);
  const handlers = useRef({ onChange, onSave });
  const [propertiesOpen, setPropertiesOpen] = useState(false);
  const [currentContent, setCurrentContent] = useState(value);
  handlers.current = { onChange, onSave };

  const separator = docKey.lastIndexOf("#");
  const notePath = separator >= 0 ? docKey.slice(0, separator) : docKey;

  const replaceContent = (content: string) => {
    const view = viewRef.current;
    if (!view) return;
    view.dispatch({ changes: { from: 0, to: view.state.doc.length, insert: content } });
    view.focus();
  };

  useEffect(() => {
    if (!host.current) return;
    setCurrentContent(value);
    const view = new EditorView({
      parent: host.current,
      state: EditorState.create({
        doc: value,
        extensions: [
          history(),
          drawSelection(),
          highlightActiveLine(),
          EditorView.lineWrapping,
          markdown({ base: markdownLanguage, codeLanguages: languages }),
          syntaxHighlighting(highlight),
          keymap.of([
            {
              key: "Mod-s",
              preventDefault: true,
              run: () => {
                handlers.current.onSave();
                return true;
              },
            },
            indentWithTab,
            ...defaultKeymap,
            ...historyKeymap,
          ]),
          EditorView.updateListener.of((update) => {
            if (update.docChanged) {
              const content = update.state.doc.toString();
              setCurrentContent(content);
              handlers.current.onChange(content);
            }
          }),
        ],
      }),
    });
    viewRef.current = view;
    view.focus();
    return () => {
      viewRef.current = null;
      view.destroy();
    };
    // The editor owns the document after creation; only a new key resets it.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [docKey]);

  return (
    <div className="editor-shell">
      <div className="editor-tools">
        <button
          type="button"
          className={propertiesOpen ? "on" : ""}
          aria-expanded={propertiesOpen}
          onClick={() => setPropertiesOpen((open) => !open)}
        >
          {strings.properties}
        </button>
      </div>
      {propertiesOpen && (
        <div className="editor-properties">
          <PropertiesPanel path={notePath} content={currentContent} refreshKey={0} onContentChange={replaceContent} />
        </div>
      )}
      <div className="editor" ref={host} />
    </div>
  );
}
