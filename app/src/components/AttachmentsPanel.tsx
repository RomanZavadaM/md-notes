import { useEffect, useMemo, useState } from "react";
import { convertFileSrc } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { openPath } from "@tauri-apps/plugin-opener";
import { api, type AttachmentInfo } from "../api";
import { useI18n } from "../i18n";

const IMAGE = /\.(png|jpe?g|gif|webp|svg|bmp|avif)$/i;

interface Props {
  vaultRoot: string;
  refreshKey: number;
  canInsert: boolean;
  onInserted: (attachment: AttachmentInfo) => void;
  onChanged: () => void;
  onError: (error: unknown) => void;
}

export function AttachmentsPanel({ vaultRoot, refreshKey, canInsert, onInserted, onChanged, onError }: Props) {
  const { t } = useI18n();
  const [attachments, setAttachments] = useState<AttachmentInfo[]>([]);
  const [orphans, setOrphans] = useState<AttachmentInfo[]>([]);
  const [selected, setSelected] = useState<string | null>(null);
  const [usedBy, setUsedBy] = useState<string[]>([]);

  const orphanPaths = useMemo(() => new Set(orphans.map((item) => item.path)), [orphans]);
  const sep = vaultRoot.includes("\\") ? "\\" : "/";
  const absolutePath = (rel: string) => `${vaultRoot}${sep}${rel.split("/").join(sep)}`;

  useEffect(() => {
    let cancelled = false;
    Promise.all([api.listAttachments(), api.orphanAttachments()])
      .then(([all, unused]) => {
        if (!cancelled) {
          setAttachments(all);
          setOrphans(unused);
        }
      })
      .catch(onError);
    return () => {
      cancelled = true;
    };
  }, [refreshKey, onError]);

  useEffect(() => {
    if (!selected) {
      setUsedBy([]);
      return;
    }
    api.attachmentUsedBy(selected).then(setUsedBy).catch(onError);
  }, [selected, refreshKey, onError]);

  const pickAttachment = async () => {
    const picked = await open({ multiple: false, directory: false, title: t.attachmentPickTitle });
    if (typeof picked !== "string") return;
    try {
      const imported = await api.importAttachment(picked);
      onChanged();
      if (canInsert) onInserted(imported);
    } catch (error) {
      onError(error);
    }
  };

  return (
    <div className="attachments-panel">
      <button type="button" className="attachment-add" title={t.attachmentAddTitle} onClick={() => void pickAttachment()}>
        {t.attachmentAdd}
      </button>
      {attachments.length === 0 ? (
        <p className="panel-empty">{t.attachmentNone}</p>
      ) : (
        <div className="attachment-list">
          {attachments.map((item) => {
            const orphan = orphanPaths.has(item.path);
            const active = selected === item.path;
            return (
              <button
                key={item.path}
                type="button"
                className={`attachment-item ${active ? "on" : ""}`}
                onClick={() => setSelected(item.path)}
                onDoubleClick={() => void openPath(absolutePath(item.path))}
                title={`${item.path}\n${orphan ? t.attachmentUnused : t.attachmentUsedBy}`}
              >
                {IMAGE.test(item.name) && (
                  <img src={convertFileSrc(absolutePath(item.path))} alt="" loading="lazy" />
                )}
                <span className="attachment-name">{item.name}</span>
                {orphan && <span className="attachment-orphan">{t.attachmentOrphans}</span>}
              </button>
            );
          })}
        </div>
      )}
      {selected && (
        <div className="attachment-details">
          <strong>{t.attachmentUsageCount(usedBy.length)}</strong>
          {usedBy.length > 0 ? (
            <ul>{usedBy.map((path) => <li key={path}>{path}</li>)}</ul>
          ) : (
            <span>{t.attachmentUnused}</span>
          )}
          <button type="button" onClick={() => void openPath(absolutePath(selected))}>
            {t.attachmentOpen}
          </button>
        </div>
      )}
    </div>
  );
}
