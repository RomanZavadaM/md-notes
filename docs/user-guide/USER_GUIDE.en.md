# MD Notes — User Guide

[🇺🇦 Українська](USER_GUIDE.uk.md) · **🇬🇧 English** · [🇫🇷 Français](USER_GUIDE.fr.md) · [🇩🇪 Deutsch](USER_GUIDE.de.md) · [🇪🇸 Español](USER_GUIDE.es.md) · [🇰🇷 한국어](USER_GUIDE.ko.md) · [🇯🇵 日本語](USER_GUIDE.ja.md)

Applies to **v1.0.0**. The Ukrainian version is the reference text.

## 1. First launch

1. Install MD Notes for your OS (see the README). On Windows, the v1.0 NSIS/MSI installers include offline WebView2 and the VC runtime; the portable ZIP uses the WebView2 Runtime already installed in Windows.
2. If needed, choose the interface language on the welcome screen.
3. Click **“Open folder”** and choose a folder with notes. Any folder with `.md` files works, including `sample-vault/` from the repository.
4. MD Notes remembers the folder and opens it on the next launch.

Opening a folder does not change your notes. MD Notes creates the service folder `.mdnotes/` for the search index, the trash and settings.

## 2. The window

- **Toolbar**:
  - sidebar button ☰ and the vault name;
  - the title of the open note: clicking it opens the quick switcher, a dot means unsaved changes;
  - the “Editor” / “Side by side” / “Preview” modes;
  - the links panel button ⇆;
  - theme and language selectors, the “About” button ⓘ.
- **Sidebar** — the **Files**, **Search** and **Tags** tabs.
  - In the file tree, folders come first and hidden folders (`.mdnotes`, `.git`) are not shown.
  - Files in other formats are greyed out.
- **Workspace** — the editor, the preview, or both side by side.
- **“Links” panel** on the right — backlinks, outgoing links and tags of the open note.
- **Status bar** — note path, tags, number of links, property errors and save state.

On narrow screens the sidebar and the links panel open over the content.

## 3. Notes and folders

- **+ Note** creates a note in the selected folder (or next to the selected file). The dialog lets you pick a **template** (section 9).
- The title becomes the file name; characters not allowed in file names are removed. A new note gets the properties `id`, `type` and `created`.
- **Today** opens the daily note (section 9).
- **+ Folder** creates a folder.
- **✎** renames the selected item. The `.md` extension is added automatically. **Links to the renamed note, or to notes inside a renamed folder, are updated in all other notes**, including properties. The app tells you how many notes were updated.
- **🗑** moves the item to the vault trash `.mdnotes/trash/`. Nothing is deleted permanently: you can move the file back by hand.
- **Multiple selection:** `Ctrl` (Windows/Linux) or `Cmd` (macOS) adds or removes an item from the selection; `Shift` selects a visible range in the tree. A normal click returns to a single selection and the usual open behavior.
- When several items are selected, **🗑** moves them to trash as one batch. If a folder and an item inside it are both selected, the nested item is not processed twice. Rename is still available only for one selected item.

## 4. Editing

- Changes are saved automatically a moment after you stop typing, and with `Ctrl+S` / `Cmd+S`.
- Writes are atomic: a failure in the middle of saving never leaves a damaged file.
- If another program (an editor, Git, a cloud client) changes a file, MD Notes refreshes the tree and reloads the open note. If you have unsaved changes, the app warns you that saving will overwrite the external changes.

## 5. Preview

- GitHub Flavored Markdown: tables, task lists, strikethrough.
- **Mermaid diagrams** — a code block with the `mermaid` language.
- **KaTeX math** — `$…$` inline and `$$…$$` as a block.
- **Images** from the vault: a relative path `![](../attachments/2026/10/diagram.png)` or an embed `![[diagram.png]]`. Only files inside the open vault are shown.
- Relative links to `.md` files open the note in the app; `https://…` links open in the system browser.

## 6. Links

- `[[Note title]]` links to another note. It is clickable in the preview.
- `[[Title|text]]` shows different text, `[[Title#Section]]` links to a section.
- If no note with that name exists, clicking the link creates it.
- Links are resolved by path, file name or the `aliases` property, ignoring case. Links inside code are ignored.
- Links in properties (`project: "[[MD Notes]]"`) count too.
- The **“Links”** panel (⇆) shows which notes link to the open note, with the line of context.

## 7. Search, tags and quick switcher

- **Search** (`Ctrl+Shift+F` / `Cmd+Shift+F`) looks through the text and titles of all notes. Every word of the query matches the start of a word. Title matches rank higher, and found words are highlighted.
- **Tags** — all tags with the number of notes. Picking a tag lists its notes, including nested tags (`#project` also finds `#project/design`).
- **Quick switcher** (`Ctrl+O` / `Cmd+O`, also `Ctrl+P`) — type part of a title, alias or path. `↑`/`↓` select, `Enter` opens. If there is no such note, `Enter` creates it.

## 8. Tags and properties

- `#tag` in the text or a `tags` list in the properties marks a topic. Tags at the end of the heading do not become part of the note title.
- Properties are written as YAML front matter at the top of the file between `---` lines.
- In the preview, properties are collapsed into a **“Properties”** block.
- If the YAML contains an error, the status bar shows it and the note opens as plain text.

## 9. Templates and daily notes

- Templates are ordinary `.md` files in `.mdnotes/templates/`. Type names for the list come from `.mdnotes/schema.json`.
- Placeholders: `{{title}}` — title, `{{date}}` — date `YYYY-MM-DD`, `{{time}}` — time `HH:MM`, `{{id}}` — a new identifier.
- **Today** opens the note `YYYY-MM-DD.md` in the daily notes folder (`dailyNotesDir` in `.mdnotes/config.json`, `daily` by default). If it does not exist, it is created from the `daily` template.

## 10. Themes and languages

- Theme: **System**, **Light** or **Dark**.
- Interface language: 🇺🇦 Українська (main) · 🇬🇧 English · 🇫🇷 Français · 🇩🇪 Deutsch · 🇪🇸 Español · 🇰🇷 한국어 · 🇯🇵 日本語.
- Both choices are remembered. The **“About”** window (ⓘ) shows the version, the copyright holder and the list of languages.

## 11. Where data is stored

- Notes are ordinary files in the chosen folder. They remain usable without MD Notes.
- `.mdnotes/` is the vault service folder: settings, templates, trash, cache.
- `.mdnotes/cache/index.db` is the search and link index. It is only a cache: you can delete it and it will be rebuilt. The cache and the trash are kept out of Git (`.mdnotes/.gitignore`).
- MD Notes sends your data nowhere: no servers, analytics or telemetry.
- Back up the vault folder. Git is a convenient way to keep versions.

### Encrypted vault (optional)

- When creating a vault, tick **“Encrypt this vault”**. Notes are stored on disk as `.age` files that cannot be read without the key.
- After creation the private key (`AGE-SECRET-KEY-…`) is shown **once**. Keep it in a password manager or offline: it cannot be recovered. Permanently the key lives only in this device's system secret store.
- You can enter a **recovery public key** (`age1…`): it can also open the vault if the main key is lost.
- On another computer, copy the vault folder and open it: the app asks for the key. Paste the private key and it is then kept in that computer's system secret store.
- In the 1.0 stabilization, images and PDFs in an encrypted vault are displayed without writing a plaintext copy to disk: data is decrypted only in memory. For an existing encrypted vault, **File → “Manage recovery key…”** can add another public `age1…` recipient. External “Open file” is disabled for encrypted attachments, and the `.mdnotes/cache` index remains unencrypted.

### Git (desktop only)

The **Git** button (or “Git…” in the menu) opens a panel: create a repository, set the remote address, save access (the token is kept only in the system secret store), and **fetch**, **pull**, **commit** and **push**. Pull runs only when there are no unsaved changes and the remote simply has something new; if histories have diverged, a manual decision is needed. Pushing requires Git to be installed on the computer. In an encrypted vault, push first checks that every file is encrypted and is blocked otherwise.

## 12. Graph, relation tree and subject lookup

### Graph

The **“Links”** panel has the tabs “Links”, “Local graph” and “Global graph”. A node is a note, its colour is the note `type`, a line is a `[[…]]` link or a typed relation. Click a node to open the note.

- **Live physics (a “floating” graph).** Drag a node: its neighbours follow and flow around it, and the graph settles smoothly after you let go. The **“Float”** button adds a gentle drift (on by default for graphs up to 400 nodes), **“Pause”** stops the motion. **“Forces”** has sliders for centre, repulsion, link strength and link length; your choice is remembered.
- **“Shapes” and “Legend”.** The **“Shapes”** button switches to a mode where a **shape** shows the note type (person — head and shoulders, company — square, institution — hexagon, case — diamond, material — triangle, group — three dots) and the **node colour** shows the **reliability** of the best grade among its relations (green 1–2, amber 3–4, red 5–6, grey — no grade). Open **“Legend”** to see every shape, colour and line style; a dashed outline and the “Ч” badge mean a rumour, and colour is never the only signal.
- **“Show relations” filter.** The list in the graph panel keeps only relations graded at least as well as chosen (“reliable” — 1–2, “reliable and medium” — 1–4); ungraded relations and plain links are hidden, the nodes stay.
- **Typed relations** are written in the `relations` property (`to`, plus `type`, `grade`, `origin`, `status`, `evidence`, and `dir: in` for the reverse direction). A `grade` is a letter plus a digit (A1 most reliable, F6 weakest): the letter rates the source, the digit the claim. Line and node outline follow the grade; grades 5–6 carry the word **“RUMOUR”**. Colour is never the only signal.
- **“Where it came from”** highlights the chain of owners above the selected node and the materials around it.
- **Separate window.** The **↗** button in the graph tabs, or “File → Graph in a separate window” (desktop only), opens the graph in its own window with more room, for example on a second monitor. It follows the active note, refreshes after changes, and a click on a node opens the note in the main window.
- **Back button.** In the graph window it returns to the previous note.

### Relation tree

The **“Relation tree”** tab shows the chain of typed relations from the selected note: **“Owners and sources”** (upwards) or **“What it owns”** (downwards), depth 1–8.

- Each node shows the relation kind and the line grade; next to it is the **“chain”** grade: the weakest link on the way from the root (B2 together with C1 gives C2). A chain is only as reliable as its weakest link.
- **“−/+”** folds a branch, **◎** rebuilds the tree from another node, **“Expand all” / “Collapse all”** control every branch. A repeated node is marked and not unfolded twice; the tree is limited to 400 nodes.
- **“Path between two notes”** finds the shortest chain of typed relations (line direction ignored) and shows its weakest link. The **“Most reliable, not shortest”** checkbox looks for the chain whose weakest link is the best (fewest steps among equally reliable ones).
- The tree uses explicit typed relations only, not plain links. The chain grade is an aid, not a legal conclusion.

### Looking up a subject by code or name

In the **“Search”** tab a number of 8 digits is looked up as an EDRPOU code, of 10 digits as a tax number (a code that lost its leading zeros, 6–7 digits, is restored). Results are shown separately: For an 8-digit code, a **““З-під Ляди” database by code”** block appears below: choose the folder with the site’s open data (a local copy of the data branch) and it shows the facts for the exact code — registration date, state and foreign countries of founders and beneficiaries (state register), competition authority decisions, Prozorro procurements as buyer and winner. Read-only, nothing is sent over the network; these are facts, not assessments.

- **“Exact match”**: a note with the same code in the `edrpou` or `ipn` property (or the same name after normalisation);
- **“Candidates by name”**: similar names; a name match is **not proof**, confirm it with a code or a document;
- **“In text”**: ordinary full-text search.

Names are compared ignoring case, quotes, hyphens and legal forms (LLC and similar); Latin letters that look like Cyrillic ones are folded to Cyrillic.

### Folder import, merging by code and possible duplicates

- **“File → Import folder…”** adds notes from a folder. A note whose code (`edrpou`/`ipn`) is already in the vault is **merged** into the existing one instead of duplicated: missing properties are added, tags, aliases and relations are united, the imported note’s title becomes an alias, differing values are not overwritten but listed for review. Importing the same folder again changes nothing.
- **“File → Possible duplicates…”** lists groups of notes with the same normalised title or alias. It is only a hint: nothing is changed, and notes with different codes are never grouped.

## 13. Keyboard

| Action | Windows / Linux | macOS |
|---|---|---|
| Save | `Ctrl+S` | `Cmd+S` |
| Quick switcher | `Ctrl+O` / `Ctrl+P` | `Cmd+O` / `Cmd+P` |
| Search | `Ctrl+Shift+F` | `Cmd+Shift+F` |
| Undo / redo | `Ctrl+Z` / `Ctrl+Shift+Z` | `Cmd+Z` / `Cmd+Shift+Z` |
| Indent | `Tab` | `Tab` |

## 14. Troubleshooting

- **Windows shows SmartScreen** — the build is not code-signed. Choose “More info → Run anyway”.
- **macOS does not open the app** — the build is not notarized. Use **System Settings → Privacy & Security → Open Anyway**.
- **The vault does not open on launch** — the folder was renamed or moved. Open it again with **“Other vault…”**.
- **Search does not find a note you just changed** — close and reopen the vault. If that does not help, delete `.mdnotes/cache/index.db` and the index will be rebuilt.
- **An image is not shown** — check that the file is inside the vault and the path is relative.

## 15. Copyright

Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved. MD Notes is proprietary software. Your notes belong to you. See [LICENSE.md](../../LICENSE.md) and [LEGAL_AND_COPYRIGHT.md](../LEGAL_AND_COPYRIGHT.md).


## Tabs and separate editor windows

In **v0.3.21**, several vault notes can remain open in tabs. Each tab keeps its own draft, so switching notes must not discard unsaved text.

On desktop, the active vault note has an **Open in separate window** action. MD Notes saves the current draft before detaching it. After the independent window is created successfully, the note is removed from the main editor tabs and the main window returns to the previous tab; if none remains, it shows an empty editor while the vault tree stays available. The detached window supports Editor / Side by side / Preview modes and autosave.

An external `.md/.markdown` remains a separate document and is not imported into the vault automatically. M3 is now integrated: if another window or process changes the file after it was opened, MD Notes blocks a silent overwrite; a dirty detached window stays open until a guarded save succeeds; full Quit safely processes all open documents first. The feature is included in published **v0.3.21**; M4 is the manual runtime acceptance step on real desktop systems.


## Vault backup

The **File** menu includes **Create backup…**. Before creating the backup, MD Notes tries to save every open tab. The result is a portable ZIP with `manifest.json`; each file is recorded with its path, size and SHA-256.

The archive contains notes, attachments, service configuration, templates and `.mdnotes/trash`. It excludes `.git`, `.mdnotes/cache` and temporary OS files. For an encrypted vault, the on-disk encrypted files are copied directly; private/recovery keys are never added to the ZIP.

**Restore backup…** validates the manifest, paths, sizes and SHA-256 values for all files before writing anything. Only after complete validation are files extracted to a temporary directory and atomically renamed into a new vault. Existing folders are never overwritten; names such as `Vault-restored`, `Vault-restored-2` are chosen automatically.
