# Third-Party Notices

MD Notes includes or can be distributed with third-party software. Those components remain the property of their respective copyright holders and are governed by their own licenses, terms and rights.

The MD Notes proprietary license does **not** replace, restrict, or relicense third-party components.

## Rust crates (direct dependencies)

Core library `crates/notes-core`:

- chrono — MIT OR Apache-2.0, dates for notes and templates
- rusqlite — MIT, SQLite bindings for the vault index; bundles **SQLite** (public domain)
- serde, serde_json — MIT OR Apache-2.0, serialization
- serde_yaml — MIT OR Apache-2.0, YAML front matter parsing
- thiserror — MIT OR Apache-2.0, error types
- ulid — MIT, stable note identifiers
- tempfile — MIT OR Apache-2.0 (development dependency, tests only)

Application `app/src-tauri`:

- tauri, tauri-build — MIT OR Apache-2.0, application shell
- tauri-plugin-dialog, tauri-plugin-opener — MIT OR Apache-2.0
- notify-debouncer-mini / notify — MIT OR Apache-2.0 / CC0-1.0, file watching (desktop)

## npm packages (direct dependencies)

- React, React DOM — MIT
- CodeMirror 6 (`@codemirror/*`), Lezer (`@lezer/highlight`) — MIT, editor
- react-markdown, remark-gfm, remark-math, rehype-katex — MIT, Markdown rendering
- KaTeX — MIT, math rendering (includes fonts under the SIL Open Font License 1.1)
- Mermaid — MIT, diagrams
- `@tauri-apps/api`, `@tauri-apps/plugin-dialog`, `@tauri-apps/plugin-opener`, `@tauri-apps/cli` — MIT OR Apache-2.0
- Vite, `@vitejs/plugin-react` — MIT (build tools)
- TypeScript — Apache-2.0 (build tool)

Some packages listed here arrive with feature branches of v0.2 and apply once those changes are integrated.

## Platform components

Platform runtimes, system webviews (WebView2, WKWebView, WebKitGTK), operating-system libraries, build tools, transitive dependencies and packaging/signing tools have separate copyright and licensing terms. The authoritative dependency versions for a source/build checkpoint are recorded in `Cargo.lock` and `app/package-lock.json`.

For executable or test distributions, applicable third-party notices must be preserved as required by those projects.

**MD Notes original materials:** Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved.

See [LICENSE.md](LICENSE.md) for the MD Notes proprietary license.
