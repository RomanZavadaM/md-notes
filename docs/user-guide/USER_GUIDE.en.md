# MD Notes — User Guide

[🇺🇦 Українська](USER_GUIDE.uk.md) · **🇬🇧 English** · [🇫🇷 Français](USER_GUIDE.fr.md) · [🇩🇪 Deutsch](USER_GUIDE.de.md) · [🇪🇸 Español](USER_GUIDE.es.md) · [🇰🇷 한국어](USER_GUIDE.ko.md) · [🇯🇵 日本語](USER_GUIDE.ja.md)

Applies to **v0.2.0**. The Ukrainian version is the reference text.

## 1. First launch

1. Install MD Notes for your OS (see the README).
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

## 12. Keyboard

| Action | Windows / Linux | macOS |
|---|---|---|
| Save | `Ctrl+S` | `Cmd+S` |
| Quick switcher | `Ctrl+O` / `Ctrl+P` | `Cmd+O` / `Cmd+P` |
| Search | `Ctrl+Shift+F` | `Cmd+Shift+F` |
| Undo / redo | `Ctrl+Z` / `Ctrl+Shift+Z` | `Cmd+Z` / `Cmd+Shift+Z` |
| Indent | `Tab` | `Tab` |

## 13. Troubleshooting

- **Windows shows SmartScreen** — the build is not code-signed. Choose “More info → Run anyway”.
- **macOS does not open the app** — the build is not notarized. Use **System Settings → Privacy & Security → Open Anyway**.
- **The vault does not open on launch** — the folder was renamed or moved. Open it again with **“Other vault…”**.
- **Search does not find a note you just changed** — close and reopen the vault. If that does not help, delete `.mdnotes/cache/index.db` and the index will be rebuilt.
- **An image is not shown** — check that the file is inside the vault and the path is relative.

## 14. Copyright

Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved. MD Notes is proprietary software. Your notes belong to you. See [LICENSE.md](../../LICENSE.md) and [LEGAL_AND_COPYRIGHT.md](../LEGAL_AND_COPYRIGHT.md).
