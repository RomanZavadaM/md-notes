import { useEffect, useState } from "react";
import { getVersion } from "@tauri-apps/api/app";
import { LANGUAGES, useI18n } from "../i18n";

/** Copyright holder; never derived from user or vault data (PROJECT_RULES §6). */
export const COPYRIGHT = "Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved.";

export function AboutDialog({ onClose }: { onClose: () => void }) {
  const { t } = useI18n();
  // The version comes from build metadata, never from a hardcoded string.
  const [version, setVersion] = useState<string | null>(null);

  useEffect(() => {
    getVersion()
      .then(setVersion)
      .catch(() => setVersion(null));
  }, []);

  return (
    <div className="dialog-backdrop" onMouseDown={onClose}>
      <div
        className="dialog about"
        role="dialog"
        aria-label={t.about}
        onMouseDown={(e) => e.stopPropagation()}
        onKeyDown={(e) => {
          if (e.key === "Escape") onClose();
        }}
      >
        <h2>MD Notes</h2>
        {version && <p className="about-version">{t.aboutVersion(version)}</p>}
        <p>{t.aboutDescription}</p>
        <p className="about-copyright">{COPYRIGHT}</p>
        <p>{t.aboutProprietary}</p>
        <p>{t.aboutYourNotes}</p>
        <p className="about-muted">{t.aboutThirdParty}</p>
        <h3>{t.aboutLanguages}</h3>
        <ul className="about-languages">
          {LANGUAGES.map((lang) => (
            <li key={lang.code}>
              {lang.flag} {lang.nativeName} — {lang.ukrainianDescription}
            </li>
          ))}
        </ul>
        <div className="dialog-actions">
          <button type="button" className="primary" autoFocus onClick={onClose}>
            {t.close}
          </button>
        </div>
      </div>
    </div>
  );
}
