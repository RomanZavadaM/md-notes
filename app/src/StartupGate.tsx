import { useEffect, useMemo, useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import App from "./App";
import { api, type RuntimePlatform, type VaultPreset } from "./api";
import { AboutDialog, COPYRIGHT } from "./components/AboutDialog";
import { ImportKeyDialog, KeyBackupDialog } from "./components/EncryptionDialogs";
import { LANGUAGES, useI18n, type LanguageCode } from "./i18n";
import { ENCRYPTION_STRINGS } from "./i18n/encryption";
import { STARTUP_STRINGS } from "./i18n/startup";
import { getRememberedVault, hasPendingVaultOpen, requestVaultOpen } from "./storage";
import "./StartupGate.css";

export function StartupGate() {
  const { t, language, setLanguage } = useI18n();
  const s = STARTUP_STRINGS[language];
  const e = ENCRYPTION_STRINGS[language];
  const [aboutOpen, setAboutOpen] = useState(false);
  const [preset, setPreset] = useState<VaultPreset>("empty");
  const [name, setName] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [creating, setCreating] = useState(false);
  const [encrypt, setEncrypt] = useState(false);
  const [recoveryKey, setRecoveryKey] = useState("");
  const [backup, setBackup] = useState<{ path: string; secretKey: string } | null>(null);
  const [needKey, setNeedKey] = useState<string | null>(null);
  const [platform, setPlatform] = useState<RuntimePlatform | null>(null);
  const pending = useMemo(() => hasPendingVaultOpen(), []);
  const remembered = useMemo(() => getRememberedVault(), []);
  const mobile = platform === "android" || platform === "ios";

  useEffect(() => {
    void api
      .runtimePlatform()
      .then(setPlatform)
      .catch((e) => setError(String(e)));
  }, []);

  if (pending) return <App />;

  const enterVault = (path: string) => {
    requestVaultOpen(path);
    window.location.reload();
  };

  // Encrypted vaults whose key is not on this device go through the import dialog.
  const enterChecked = async (path: string) => {
    setError(null);
    try {
      const status = await api.encryptionStatus(path);
      if (status.encrypted && !status.hasKey) {
        setNeedKey(path);
        return;
      }
    } catch (err) {
      setError(String(err));
      return;
    }
    enterVault(path);
  };

  const openExisting = async () => {
    setError(null);
    const dir = await open({ directory: true, multiple: false, title: s.pickExistingTitle });
    if (typeof dir === "string") await enterChecked(dir);
  };

  const createVault = async () => {
    setError(null);
    const dir = await open({ directory: true, multiple: false, title: s.pickNewTitle });
    if (typeof dir !== "string") return;
    setCreating(true);
    try {
      if (encrypt) {
        const created = await api.createEncryptedVault(dir, name.trim() || null, recoveryKey.trim() || null);
        setBackup({ path: dir, secretKey: created.secretKey });
        setCreating(false);
        return;
      }
      await api.createVault(dir, name.trim() || null, preset);
      enterVault(dir);
    } catch (err) {
      setError(String(err));
      setCreating(false);
    }
  };

  const openMobileLocal = async () => {
    setError(null);
    setCreating(true);
    try {
      const info = await api.openMobileSandboxVault();
      enterVault(info.root);
    } catch (e) {
      setError(String(e));
      setCreating(false);
    }
  };

  const presetHelp =
    preset === "para" ? s.presetParaHelp : preset === "zettelkasten" ? s.presetZettelkastenHelp : s.presetEmptyHelp;

  return (
    <div className="welcome startup-gate">
      <h1>MD Notes</h1>
      <p>{t.appTagline}</p>

      {mobile ? (
        <section className="startup-card">
          <button type="button" className="primary" disabled={creating} onClick={() => void openMobileLocal()}>
            {creating ? s.openingLocal : s.mobileLocal}
          </button>
          <small>{s.mobileLocalHelp}</small>
        </section>
      ) : (
        <>
          <div className="startup-actions">
            {remembered && (
              <>
                <button type="button" className="primary" onClick={() => void enterChecked(remembered)}>
                  {s.openRecent}
                </button>
                <small className="startup-recent-path" title={remembered}>
                  {s.recentPath}: {remembered}
                </small>
              </>
            )}
            <button type="button" className={remembered ? "" : "primary"} onClick={() => void openExisting()}>
              {s.openExisting}
            </button>
          </div>

          <section className="startup-card">
            <h2>{s.createHeading}</h2>
            <label>
              {s.vaultName}
              <input
                value={name}
                onChange={(event) => setName(event.target.value)}
                placeholder={s.vaultNamePlaceholder}
              />
            </label>
            <label>
              {s.preset}
              <select value={preset} onChange={(event) => setPreset(event.target.value as VaultPreset)}>
                <option value="empty">{s.presetEmpty}</option>
                <option value="para">{s.presetPara}</option>
                <option value="zettelkasten">{s.presetZettelkasten}</option>
              </select>
            </label>
            <small>{presetHelp}</small>
            <label className="key-confirm">
              <input type="checkbox" checked={encrypt} onChange={(event) => setEncrypt(event.target.checked)} />
              {e.createEncrypted}
            </label>
            {encrypt && (
              <>
                <small>{e.createEncryptedHelp}</small>
                <label>
                  {e.recoveryKey}
                  <input
                    value={recoveryKey}
                    onChange={(event) => setRecoveryKey(event.target.value)}
                    placeholder={e.recoveryKeyPlaceholder}
                    spellCheck={false}
                  />
                </label>
                <small>{e.recoveryKeyHelp}</small>
              </>
            )}
            <button type="button" className="primary" disabled={creating} onClick={() => void createVault()}>
              {creating ? s.creating : s.createButton}
            </button>
            <small>{s.emptyFolderOnly}</small>
          </section>
        </>
      )}

      {error && <p className="welcome-error">{error}</p>}

      <div className="startup-footer">
        <select
          className="welcome-language"
          value={language}
          onChange={(event) => setLanguage(event.target.value as LanguageCode)}
          aria-label={t.language}
        >
          {LANGUAGES.map((lang) => (
            <option key={lang.code} value={lang.code}>
              {lang.flag} {lang.nativeName}
            </option>
          ))}
        </select>
        <button type="button" className="link-button" onClick={() => setAboutOpen(true)}>
          {t.about}
        </button>
      </div>
      <p className="welcome-copyright">{COPYRIGHT}</p>
      {aboutOpen && <AboutDialog onClose={() => setAboutOpen(false)} />}
      {backup && <KeyBackupDialog secretKey={backup.secretKey} onDone={() => enterVault(backup.path)} />}
      {needKey && (
        <ImportKeyDialog path={needKey} onImported={() => enterVault(needKey)} onCancel={() => setNeedKey(null)} />
      )}
    </div>
  );
}
