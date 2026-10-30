import { useState } from "react";
import { api, type VaultInfo } from "../api";
import { useI18n } from "../i18n";
import { ENCRYPTION_STRINGS } from "../i18n/encryption";

interface BackupProps {
  secretKey: string;
  onDone: () => void;
}

/** Shows the private key once after an encrypted vault is created. */
export function KeyBackupDialog({ secretKey, onDone }: BackupProps) {
  const { language } = useI18n();
  const s = ENCRYPTION_STRINGS[language];
  const [copied, setCopied] = useState(false);
  const [confirmed, setConfirmed] = useState(false);

  const copy = async () => {
    try {
      await navigator.clipboard.writeText(secretKey);
      setCopied(true);
    } catch {
      // Clipboard can be unavailable; the key stays selectable in the field.
    }
  };

  return (
    <div className="dialog-backdrop">
      <div className="dialog" role="dialog" aria-label={s.backupTitle}>
        <h2>{s.backupTitle}</h2>
        <p>{s.backupText}</p>
        <textarea className="key-field" readOnly rows={3} value={secretKey} onFocus={(e) => e.target.select()} />
        <div className="dialog-actions">
          <button type="button" onClick={() => void copy()}>
            {copied ? s.copied : s.copy}
          </button>
        </div>
        <label className="key-confirm">
          <input type="checkbox" checked={confirmed} onChange={(e) => setConfirmed(e.target.checked)} />
          {s.backupConfirm}
        </label>
        <div className="dialog-actions">
          <button type="button" className="primary" disabled={!confirmed} onClick={onDone}>
            {s.backupContinue}
          </button>
        </div>
      </div>
    </div>
  );
}

interface ImportProps {
  path: string;
  onImported: (info: VaultInfo) => void;
  onCancel: () => void;
}

/** Asks for the private key of an encrypted vault this device cannot open yet. */
export function ImportKeyDialog({ path, onImported, onCancel }: ImportProps) {
  const { t, language } = useI18n();
  const s = ENCRYPTION_STRINGS[language];
  const [value, setValue] = useState("");
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const trimmed = value.trim();

  const submit = async () => {
    setBusy(true);
    setError(null);
    try {
      onImported(await api.importEncryptionKey(path, trimmed));
    } catch (e) {
      setError(String(e));
      setBusy(false);
    }
  };

  return (
    <div className="dialog-backdrop" onMouseDown={onCancel}>
      <form
        className="dialog"
        role="dialog"
        aria-label={s.importTitle}
        onMouseDown={(e) => e.stopPropagation()}
        onSubmit={(e) => {
          e.preventDefault();
          if (trimmed && !busy) void submit();
        }}
        onKeyDown={(e) => {
          if (e.key === "Escape") onCancel();
        }}
      >
        <h2>{s.importTitle}</h2>
        <p>{s.importText}</p>
        <textarea
          className="key-field"
          autoFocus
          rows={3}
          value={value}
          placeholder={s.importPlaceholder}
          autoComplete="off"
          spellCheck={false}
          onChange={(e) => setValue(e.target.value)}
        />
        {error && <p className="welcome-error">{error}</p>}
        <div className="dialog-actions">
          <button type="button" onClick={onCancel}>
            {t.cancel}
          </button>
          <button type="submit" className="primary" disabled={!trimmed || busy}>
            {busy ? s.importing : s.importButton}
          </button>
        </div>
      </form>
    </div>
  );
}

interface RecoveryRecipientProps {
  onAdded: (rewritten: number) => void;
  onCancel: () => void;
}

export function RecoveryRecipientDialog({ onAdded, onCancel }: RecoveryRecipientProps) {
  const { t, language } = useI18n();
  const s = ENCRYPTION_STRINGS[language];
  const [value, setValue] = useState("");
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const trimmed = value.trim();

  const submit = async () => {
    setBusy(true);
    setError(null);
    try {
      const rewritten = await api.addEncryptionRecipient(trimmed);
      onAdded(rewritten);
    } catch (e) {
      setError(String(e));
      setBusy(false);
    }
  };

  return (
    <div className="dialog-backdrop" onMouseDown={onCancel}>
      <form
        className="dialog"
        role="dialog"
        aria-label={s.manageRecoveryTitle}
        onMouseDown={(e) => e.stopPropagation()}
        onSubmit={(e) => {
          e.preventDefault();
          if (trimmed && !busy) void submit();
        }}
        onKeyDown={(e) => {
          if (e.key === "Escape") onCancel();
        }}
      >
        <h2>{s.manageRecoveryTitle}</h2>
        <p>{s.manageRecoveryText}</p>
        <label>
          {s.recoveryKey}
          <textarea
            className="key-field"
            autoFocus
            rows={2}
            value={value}
            placeholder={s.recoveryKeyPlaceholder}
            autoComplete="off"
            spellCheck={false}
            onChange={(e) => setValue(e.target.value)}
          />
        </label>
        <p className="dialog-help">{s.recoveryKeyHelp}</p>
        {error && <p className="welcome-error">{error}</p>}
        <div className="dialog-actions">
          <button type="button" onClick={onCancel}>{t.cancel}</button>
          <button type="submit" className="primary" disabled={!trimmed || busy}>
            {busy ? s.addingRecovery : s.addRecovery}
          </button>
        </div>
      </form>
    </div>
  );
}
