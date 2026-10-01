import { useEffect, useMemo, useState } from "react";
import { convertFileSrc } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { openPath } from "@tauri-apps/plugin-opener";
import { api, type AttachmentInfo } from "../api";
import { useI18n } from "../i18n";
import "./AttachmentsPanel.css";

const IMAGE = /\.(png|jpe?g|gif|webp|svg|bmp|avif)$/i;
const PDF = /\.pdf$/i;

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
  const selectedItem = selected ? attachments.find((item) => item.path === selected) ?? null : null;

  useEffect(() => {
    let cancelled = false;
    Promise.all([api.listAttachments(), api.orphanAttachments()])
      .then(([all, unused]) => {
        if (!cancelled) {
          setAttachments(all);
          setOrphans(unused);
          if (selected && !all.some((item) => item.path === selected)) setSelected(null);
        }
      })
      .catch(onError);
    return () => {
      cancelled = true;
    };
  }, [refreshKey, onError, selected]);

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
      setSelected(imported.path);
      onChanged();
      if (canInsert) onInserted(imported);
    } catch (error) {
      onError(error);
    }
  };

  const selectedUrl = selectedItem ? convertFileSrc(absolutePath(selectedItem.path)) : null;

  return (
    <div className="attachments-panel">
      <div className="attachment-toolbar">
        <button type="button" className="attachment-add" title={t.attachmentAddTitle} onClick={() => void pickAttachment()}>
          {t.attachmentAdd}
        </button>
        <span className="attachment-summary">{t.attachmentUsageCount(attachments.length)}</span>
      </div>
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
                {IMAGE.test(item.name) ? (
                  <img src={convertFileSrc(absolutePath(item.path))} alt="" loading="lazy" />
                ) : (
                  <span className="attachment-file-icon" aria-hidden>{PDF.test(item.name) ? "PDF" : "FILE"}</span>
                )}
                <span className="attachment-name">{item.name}</span>
                {orphan && <span className="attachment-orphan">{t.attachmentOrphans}</span>}
              </button>
            );
          })}
        </div>
      )}
      {selectedItem && (
        <div className="attachment-details">
          {IMAGE.test(selectedItem.name) && selectedUrl && (
            <img className="attachment-preview-image" src={selectedUrl} alt={selectedItem.name} />
          )}
          {PDF.test(selectedItem.name) && selectedUrl && (
            <iframe className="attachment-preview-pdf" src={selectedUrl} title={selectedItem.name} />
          )}
          <strong>{t.attachmentUsageCount(usedBy.length)}</strong>
          {usedBy.length > 0 ? (
            <ul>{usedBy.map((path) => <li key={path}>{path}</li>)}</ul>
          ) : (
            <span>{t.attachmentUnused}</span>
          )}
          <div className="attachment-detail-actions">
            <button type="button" onClick={() => void openPath(absolutePath(selectedItem.path))}>
              {t.attachmentOpen}
            </button>
            {canInsert && (
              <button type="button" onClick={() => onInserted(selectedItem)}>
                {t.attachmentInsert}
              </button>
            )}
          </div>
        </div>
      )}
    </div>
  );
}
