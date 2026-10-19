import { useCallback, useEffect, useState } from "react";
import { api, type GitRelation, type GitStatus } from "../api";
import { useI18n } from "../i18n";
import { GIT_STRINGS, type GitStrings } from "../i18n/git";

interface Props {
  encrypted: boolean;
  onClose: () => void;
  /** Called after the working tree may have changed (pull). */
  onVaultChanged: () => void;
}

function relationText(s: GitStrings, relation: GitRelation | null): string {
  switch (relation) {
    case "upToDate":
      return s.relUpToDate;
    case "fastForward":
      return s.relFastForward;
    case "localAhead":
      return s.relLocalAhead;
    case "diverged":
      return s.relDiverged;
    case "unrelated":
      return s.relUnrelated;
    default:
      return s.relNone;
  }
}

/** Git panel: status, access, fetch / pull / commit / push for the open vault. */
export function GitPanel({ encrypted, onClose, onVaultChanged }: Props) {
  const { language } = useI18n();
  const s = GIT_STRINGS[language];
  const [status, setStatus] = useState<GitStatus | null>(null);
  const [hasCreds, setHasCreds] = useState(false);
  const [busy, setBusy] = useState(false);
  const [message, setMessage] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [blocked, setBlocked] = useState<string[]>([]);
  const [remoteUrl, setRemoteUrl] = useState("");
  const [user, setUser] = useState("");
  const [token, setToken] = useState("");
  const [commitMessage, setCommitMessage] = useState(s.commitDefault);

  const refresh = useCallback(async () => {
    try {
      setStatus(await api.gitStatus());
      setHasCreds(await api.hasGitCredentials());
    } catch (e) {
      setError(String(e));
    }
  }, []);

  useEffect(() => {
    void refresh();
  }, [refresh]);

  const run = async (action: () => Promise<string | void>) => {
    setBusy(true);
    setError(null);
    setMessage(null);
    setBlocked([]);
    try {
      const text = await action();
      if (text) setMessage(text);
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
      await refresh();
    }
  };

  const fetchRemote = () =>
    run(async () => {
      await api.gitFetch();
      return s.doneFetch;
    });

  const pull = () =>
    run(async () => {
      const result = await api.gitPull();
      onVaultChanged();
      if (result.applied) return s.donePulled;
      if (result.relation === "upToDate" || result.relation === "localAhead") return s.donePullUpToDate;
      return result.worktreeClean ? s.pullBlockedOther : s.pullBlockedDirty;
    });

  const commit = () =>
    run(async () => {
      const id = await api.gitCommit(commitMessage.trim() || s.commitDefault);
      return id ? `${s.doneCommit} ${id.slice(0, 8)}` : s.doneNothing;
    });

  const push = () =>
    run(async () => {
      if (encrypted) {
        const findings = await api.pushSafety();
        if (findings.length > 0) {
          setBlocked(findings.map((f) => f.path));
          return s.guardBlocked;
        }
      }
      return (await api.gitPush()) === "pushed" ? s.donePush : s.donePushUpToDate;
    });

  const ready = status?.isRepo === true;
  const canSync = ready && !!status?.remoteUrl && hasCreds && !busy;

  return (
    <div className="dialog-backdrop" onMouseDown={onClose}>
      <div
        className="dialog git-panel"
        role="dialog"
        aria-label={s.title}
        onMouseDown={(e) => e.stopPropagation()}
        onKeyDown={(e) => {
          if (e.key === "Escape") onClose();
        }}
      >
        <h2>{s.title}</h2>
        {encrypted && <p className="git-note">{s.encryptedNote}</p>}

        {status && !status.isRepo && (
          <section>
            <p>{s.notRepo}</p>
            <button type="button" className="primary" disabled={busy} onClick={() => void run(() => api.gitInit())}>
              {s.initRepo}
            </button>
          </section>
        )}

        {status?.isRepo && (
          <>
            <dl className="git-status">
              <dt>{s.branchLabel}</dt>
              <dd>{status.branch ?? "—"}</dd>
              <dt>{s.remoteLabel}</dt>
              <dd className="git-remote">{status.remoteUrl ?? s.noRemote}</dd>
              <dt>{s.changesLabel}</dt>
              <dd>{status.hasChanges ? s.changesYes : s.changesNo}</dd>
              <dt>{s.relationLabel}</dt>
              <dd>{relationText(s, status.relation)}</dd>
            </dl>

            <section>
              <label>
                {s.remoteLabel}
                <input
                  value={remoteUrl}
                  placeholder={s.remotePlaceholder}
                  spellCheck={false}
                  onChange={(e) => setRemoteUrl(e.target.value)}
                />
              </label>
              <button
                type="button"
                disabled={busy || !remoteUrl.trim()}
                onClick={() => void run(() => api.gitSetRemote(remoteUrl.trim()))}
              >
                {s.setRemote}
              </button>
            </section>
          </>
        )}

        <section>
          <h3>{s.credsHeading}</h3>
          <p className="git-note">{hasCreds ? s.credsSet : s.credsMissing}</p>
          <label>
            {s.userLabel}
            <input value={user} autoComplete="off" onChange={(e) => setUser(e.target.value)} />
          </label>
          <label>
            {s.tokenLabel}
            <input
              type="password"
              value={token}
              autoComplete="off"
              spellCheck={false}
              onChange={(e) => setToken(e.target.value)}
            />
          </label>
          <small>{s.tokenHelp}</small>
          <div className="dialog-actions">
            <button
              type="button"
              disabled={busy || !hasCreds}
              onClick={() => void run(() => api.clearGitCredentials())}
            >
              {s.clearCreds}
            </button>
            <button
              type="button"
              disabled={busy || !user.trim() || !token}
              onClick={() =>
                void run(async () => {
                  await api.saveGitCredentials(user.trim(), token);
                  setToken("");
                })
              }
            >
              {s.saveCreds}
            </button>
          </div>
        </section>

        {ready && (
          <section>
            <h3>{s.actionsHeading}</h3>
            <label>
              {s.commitMessage}
              <input value={commitMessage} onChange={(e) => setCommitMessage(e.target.value)} />
            </label>
            <div className="git-actions">
              <button type="button" disabled={busy} onClick={() => void commit()}>
                {s.commit}
              </button>
              <button type="button" disabled={!canSync} onClick={() => void fetchRemote()}>
                {s.fetch}
              </button>
              <button type="button" disabled={!canSync} onClick={() => void pull()}>
                {s.pull}
              </button>
              <button type="button" className="primary" disabled={!canSync} onClick={() => void push()}>
                {s.push}
              </button>
            </div>
          </section>
        )}

        {busy && <p className="git-note">{s.working}</p>}
        {message && <p role="status">{message}</p>}
        {blocked.length > 0 && (
          <ul className="git-blocked">
            {blocked.map((path) => (
              <li key={path}>{path}</li>
            ))}
          </ul>
        )}
        {error && <p className="welcome-error">{error}</p>}

        <div className="dialog-actions">
          <button type="button" disabled={busy} onClick={() => void refresh()}>
            {s.refresh}
          </button>
          <button type="button" className="primary" onClick={onClose}>
            {s.close}
          </button>
        </div>
      </div>
    </div>
  );
}
