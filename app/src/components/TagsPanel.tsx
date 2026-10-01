import { useEffect, useState } from "react";
import { api, type NoteSummary, type TagCount } from "../api";
import { useI18n } from "../i18n";

interface Props {
  refreshKey: number;
  onOpen: (path: string) => void;
}

/** Tag cloud; picking a tag lists its notes (nested tags included). */
export function TagsPanel({ refreshKey, onOpen }: Props) {
  const { t } = useI18n();
  const [tags, setTags] = useState<TagCount[]>([]);
  const [selected, setSelected] = useState<string | null>(null);
  const [notes, setNotes] = useState<NoteSummary[]>([]);

  useEffect(() => {
    api
      .listTags()
      .then(setTags)
      .catch(() => setTags([]));
  }, [refreshKey]);

  useEffect(() => {
    if (!selected) {
      setNotes([]);
      return;
    }
    const matches = (tag: string) => tag === selected || tag.startsWith(`${selected}/`);
    api
      .listNotes()
      .then((all) => setNotes(all.filter((n) => n.tags.some(matches))))
      .catch(() => setNotes([]));
  }, [selected, refreshKey]);

  if (tags.length === 0) {
    return <p className="panel-note">{t.tagsEmpty}</p>;
  }
  return (
    <div className="panel">
      <div className="tag-cloud">
        {tags.map(({ tag, count }) => (
          <button
            key={tag}
            type="button"
            className={`tag ${tag === selected ? "on" : ""}`}
            onClick={() => setSelected(tag === selected ? null : tag)}
          >
            #{tag} <span className="tag-count">{count}</span>
          </button>
        ))}
      </div>
      {selected && (
        <ul className="result-list">
          {notes.map((note) => (
            <li key={note.path}>
              <button type="button" className="result" onClick={() => onOpen(note.path)} title={note.path}>
                <span className="result-title">{note.title}</span>
                <span className="result-path">{note.path}</span>
              </button>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}
