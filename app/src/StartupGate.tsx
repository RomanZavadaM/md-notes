import { useEffect, useMemo, useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import App from "./App";
import { api, type RuntimePlatform, type VaultPreset } from "./api";
import { AboutDialog, COPYRIGHT } from "./components/AboutDialog";
import { LANGUAGES, useI18n, type LanguageCode } from "./i18n";
import { STARTUP_STRINGS } from "./i18n/startup";
import { getRememberedVault, hasPendingVaultOpen, requestVaultOpen } from "./storage";
import "./StartupGate.css";

export function StartupGate() {
  const { t, language, setLanguage } = useI18n();
  const s = STARTUP_STRINGS[language];
  const [aboutOpen, setAboutOpen] = useState(false);
  const [preset, setPreset] = useState<VaultPreset>("empty");
  const [name, setName] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [creating, setCreating] = useState(false);
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

  const openExisting = async () => {
    setError(null);
    const dir = await open({ directory: true, multiple: false, title: s.pickExistingTitle });
    if (typeof dir === "string") enterVault(dir);
  };

  const createVault = async () => {
    setError(null);
    const dir = await open({ directory: true, multiple: false, title: s.pickNewTitle });
    if (typeof dir !== "string") return;
    setCreating(true);
    try {
      await api.createVault(dir, name.trim() || null, preset);
      enterVault(dir);
    } catch (e) {
      setError(String(e));
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
                <button type="button" className="primary" onClick={() => enterVault(remembered)}>
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
            <option key={lang.code} value={lang.code} title={lang.ukrainianDescription}>
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
    </div>
  );
}
