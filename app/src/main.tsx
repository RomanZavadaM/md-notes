import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { I18nProvider } from "./i18n";
import { GraphWindow } from "./GraphWindow";
import { isGraphWindow } from "./graphWindow";
import { StartupGate } from "./StartupGate";
import "./styles.css";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <I18nProvider>
      {isGraphWindow() ? <GraphWindow /> : <StartupGate />}
    </I18nProvider>
  </StrictMode>,
);
