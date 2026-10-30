# Third-Party Notices

MD Notes includes or can be distributed with third-party software. Those components remain the property of their respective copyright holders and are governed by their own licenses, terms and rights.

The MD Notes proprietary license does **not** replace, restrict, or relicense third-party components.

## Rust crates (direct dependencies)

Core library `crates/notes-core`:

- chrono — MIT OR Apache-2.0, dates for notes and templates
- gix (Gitoxide) — MIT OR Apache-2.0, Git repository and HTTPS synchronization foundation
- rusqlite — MIT, SQLite bindings for the vault index; bundles **SQLite** (public domain)
- serde, serde_json — MIT OR Apache-2.0, serialization
- sha2 — MIT OR Apache-2.0, SHA-256 checksums for backup manifests and verification
- zip — MIT, ZIP container implementation used by backup/restore; stable 1.x uses the security-maintained 9.x line and MD Notes performs its own path/size/checksum validation before restore
- serde_norway (imported in code as `serde_yaml`) — MIT OR Apache-2.0, maintained YAML front matter parser compatible with the former serde_yaml API; uses maintained `unsafe-libyaml-norway` (MIT)
- thiserror — MIT OR Apache-2.0, error types
- ulid — MIT, stable note identifiers
- age (optional feature `encrypted-vault`, ADR-0007) — MIT OR Apache-2.0, file encryption in the open age format; no default features (no async, CLI, plugin or SSH code)
- secrecy (optional feature `encrypted-vault`, ADR-0007) — MIT OR Apache-2.0, reading the secret string of an age key (already a transitive dependency of age)
- tempfile — MIT OR Apache-2.0 (development dependency, tests only)

Application `app/src-tauri`:

- tauri, tauri-build — MIT OR Apache-2.0, application shell
- tauri-plugin-dialog, tauri-plugin-opener — MIT OR Apache-2.0
- keyring-core — MIT OR Apache-2.0, common credential-store API used for Git HTTPS secrets
- target-specific native credential-store crates: `windows-native-keyring-store`, `apple-native-keyring-store`, `android-native-keyring-store`, `zbus-secret-service-keyring-store`; only the backend for the target platform is linked, and its license is enforced by the Cargo license gate
- notify-debouncer-mini / notify — MIT OR Apache-2.0 / CC0-1.0, file watching (desktop)

## npm packages (direct dependencies)

- React, React DOM — MIT
- CodeMirror 6 (`@codemirror/*`), Lezer (`@lezer/highlight`) — MIT, editor
- react-markdown, remark-gfm, remark-math, rehype-katex — MIT, Markdown rendering
- KaTeX — MIT, math rendering (includes fonts under the SIL Open Font License 1.1)
- Mermaid 11 — MIT, diagrams (Mermaid 12 is avoided because it bundles EPL-2.0 elkjs)
- Sigma.js — MIT, WebGL knowledge-graph rendering
- Graphology — MIT, in-memory graph model used by Sigma.js
- `@tauri-apps/api`, `@tauri-apps/plugin-dialog`, `@tauri-apps/plugin-opener`, `@tauri-apps/cli` — MIT OR Apache-2.0
- Vite, `@vitejs/plugin-react` — MIT (build tools)
- TypeScript — Apache-2.0 (build tool)

This notice reflects dependencies present in the stable v1.0.0 source/build checkpoint; authoritative versions remain recorded in `Cargo.lock` and `app/package-lock.json`.

## License review

MD Notes ships only dependencies that can be used in a proprietary distribution. Every release checkpoint is checked automatically (`deny.toml`, `app/scripts/check-licenses.mjs`, CI job “Dependency licenses”), and the repository also runs a recurring dependency security audit. Exact dependency counts and versions are intentionally taken from the current `Cargo.lock` and `app/package-lock.json`; every newly added or updated dependency must pass the same gates before integration.

- almost all components use MIT, Apache-2.0, BSD, ISC, Zlib, CC0, Unlicense or Unicode licenses;
- dual-licensed components are used under their permissive option: `dompurify` (MPL-2.0 **or** Apache-2.0 → Apache-2.0), `r-efi` (MIT **or** Apache-2.0 **or** LGPL-2.1+ → MIT);
- **replaced:** Mermaid 12 depends on `elkjs` (EPL-2.0, copyleft), so MD Notes uses Mermaid 11.17, which has no such dependency;
- **MPL-2.0, used unmodified as part of the Tauri platform:** `cssparser`, `cssparser-macros`, `dtoa-short`, `selectors` (via `dom_query`), `option-ext` (via `dirs`). MPL-2.0 is file-level copyleft and allows inclusion in a proprietary larger work (MPL-2.0 §3.3); the source code of these crates is available on crates.io and their notices are preserved. Replacing them would require replacing Tauri;
- `khroma` has no license field in the npm registry; its upstream repository is MIT-licensed;
- `caniuse-lite` (CC-BY-4.0) is browser data used only by build tools and is not shipped;
- on Linux the app links dynamically to system libraries such as WebKitGTK and GTK (LGPL-2.1+); they remain replaceable system components.

## Platform components

Platform runtimes, system webviews (WebView2, WKWebView, WebKitGTK), operating-system credential managers, operating-system libraries, build tools, transitive dependencies and packaging/signing tools have separate copyright and licensing terms. The authoritative dependency versions for a source/build checkpoint are recorded in `Cargo.lock` and `app/package-lock.json`.

For executable or test distributions, applicable third-party notices must be preserved as required by those projects.

**MD Notes original materials:** Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved.

See [LICENSE.md](LICENSE.md) for the MD Notes proprietary license.
