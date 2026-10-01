# MD Notes — Guía del usuario

[🇺🇦 Українська](USER_GUIDE.uk.md) · [🇬🇧 English](USER_GUIDE.en.md) · [🇫🇷 Français](USER_GUIDE.fr.md) · [🇩🇪 Deutsch](USER_GUIDE.de.md) · **🇪🇸 Español** · [🇰🇷 한국어](USER_GUIDE.ko.md) · [🇯🇵 日本語](USER_GUIDE.ja.md)

Aplicable a la **v0.2.0**. La versión ucraniana es el texto de referencia.

## 1. Primer inicio

1. Instala MD Notes para tu sistema (consulta el README).
2. Si lo necesitas, elige el idioma de la interfaz en la pantalla de bienvenida.
3. Pulsa **«Abrir carpeta»** y elige una carpeta con notas. Sirve cualquier carpeta con archivos `.md`, incluida `sample-vault/` del repositorio.
4. MD Notes recuerda la carpeta y la abre en el siguiente inicio.

Abrir una carpeta no modifica tus notas. MD Notes crea la carpeta de servicio `.mdnotes/` para el índice de búsqueda, la papelera y los ajustes.

## 2. La ventana

- **Barra de herramientas**:
  - botón del panel lateral ☰ y nombre de la bóveda;
  - título de la nota abierta: al pulsarlo se abre el selector rápido; un punto indica cambios sin guardar;
  - los modos «Editor» / «En paralelo» / «Vista previa»;
  - botón del panel de enlaces ⇆;
  - selectores de tema e idioma, botón «Acerca de» ⓘ.
- **Panel lateral** — pestañas **Archivos**, **Búsqueda** y **Etiquetas**.
  - En el árbol de archivos, las carpetas aparecen primero y las carpetas ocultas (`.mdnotes`, `.git`) no se muestran.
  - Los archivos de otros formatos aparecen en gris.
- **Área de trabajo** — el editor, la vista previa o ambos en paralelo.
- **Panel «Enlaces»** a la derecha — enlaces entrantes, enlaces salientes y etiquetas de la nota abierta.
- **Barra de estado** — ruta de la nota, etiquetas, número de enlaces, errores en las propiedades y estado del guardado.

En pantallas estrechas, el panel lateral y el panel de enlaces se abren sobre el contenido.

## 3. Notas y carpetas

- **+ Nota** crea una nota en la carpeta seleccionada (o junto al archivo seleccionado). En el diálogo se puede elegir una **plantilla** (sección 9).
- El título pasa a ser el nombre del archivo; se eliminan los caracteres no permitidos en nombres de archivo. Una nota nueva recibe las propiedades `id`, `type` y `created`.
- **Hoy** abre la nota diaria (sección 9).
- **+ Carpeta** crea una carpeta.
- **✎** cambia el nombre del elemento seleccionado. La extensión `.md` se añade automáticamente. **Los enlaces a la nota renombrada, o a las notas de una carpeta renombrada, se actualizan en todas las demás notas**, propiedades incluidas. La aplicación indica en cuántas notas se actualizaron enlaces.
- **🗑** mueve el elemento a la papelera de la bóveda `.mdnotes/trash/`. Nada se elimina definitivamente: el archivo se puede devolver a mano.

## 4. Edición

- Los cambios se guardan automáticamente poco después de dejar de escribir y también con `Ctrl+S` / `Cmd+S`.
- La escritura es atómica: un fallo a mitad del guardado nunca deja un archivo dañado.
- Si otro programa (un editor, Git, un cliente en la nube) cambia un archivo, MD Notes actualiza el árbol y vuelve a cargar la nota abierta. Si tienes cambios sin guardar, la aplicación avisa de que guardar sobrescribirá los cambios externos.

## 5. Vista previa

- GitHub Flavored Markdown: tablas, listas de tareas, tachado.
- **Diagramas Mermaid** — un bloque de código con el lenguaje `mermaid`.
- **Fórmulas KaTeX** — `$…$` en línea y `$$…$$` como bloque.
- **Imágenes** de la bóveda: ruta relativa `![](../attachments/2026/10/esquema.png)` o incrustación `![[esquema.png]]`. Solo se muestran archivos de la bóveda abierta.
- Los enlaces relativos a archivos `.md` abren la nota en la aplicación; los enlaces `https://…` se abren en el navegador del sistema.

## 6. Enlaces

- `[[Título de la nota]]` enlaza con otra nota y es clicable en la vista previa.
- `[[Título|texto]]` muestra otro texto, `[[Título#Sección]]` enlaza con una sección.
- Si no existe una nota con ese nombre, al hacer clic se crea.
- Los enlaces se resuelven por ruta, nombre de archivo o la propiedad `aliases`, sin distinguir mayúsculas. Los enlaces dentro de código se ignoran.
- También cuentan los enlaces en las propiedades (`project: "[[MD Notes]]"`).
- El panel **«Enlaces»** (⇆) muestra qué notas enlazan con la nota abierta, con la línea de contexto.

## 7. Búsqueda, etiquetas y selector rápido

- **Búsqueda** (`Ctrl+Shift+F` / `Cmd+Shift+F`) recorre el texto y los títulos de todas las notas. Cada palabra de la consulta coincide con el inicio de una palabra. Las coincidencias en el título aparecen antes y las palabras encontradas se resaltan.
- **Etiquetas** — todas las etiquetas con el número de notas. Al elegir una etiqueta se muestran sus notas, incluidas las etiquetas anidadas (`#proyecto` también encuentra `#proyecto/diseño`).
- **Selector rápido** (`Ctrl+O` / `Cmd+O`, también `Ctrl+P`) — escribe parte del título, un alias o la ruta. `↑`/`↓` seleccionan, `Intro` abre. Si la nota no existe, `Intro` la crea.

## 8. Etiquetas y propiedades

- `#etiqueta` en el texto o una lista `tags` en las propiedades marca un tema. Las etiquetas al final del encabezado no forman parte del título de la nota.
- Las propiedades se escriben como YAML front matter al principio del archivo, entre líneas `---`.
- En la vista previa, las propiedades aparecen plegadas en el bloque **«Propiedades»**.
- Si el YAML contiene un error, la barra de estado lo muestra y la nota se abre como texto normal.

## 9. Plantillas y notas diarias

- Las plantillas son archivos `.md` normales en `.mdnotes/templates/`. Los nombres de tipo de la lista proceden de `.mdnotes/schema.json`.
- Marcadores: `{{title}}` — título, `{{date}}` — fecha `AAAA-MM-DD`, `{{time}}` — hora `HH:MM`, `{{id}}` — nuevo identificador.
- **Hoy** abre la nota `AAAA-MM-DD.md` en la carpeta de notas diarias (`dailyNotesDir` en `.mdnotes/config.json`, `daily` por defecto). Si no existe, se crea a partir de la plantilla `daily`.

## 10. Temas e idiomas

- Tema: **Sistema**, **Claro** u **Oscuro**.
- Idioma de la interfaz: 🇺🇦 Українська (principal) · 🇬🇧 English · 🇫🇷 Français · 🇩🇪 Deutsch · 🇪🇸 Español · 🇰🇷 한국어 · 🇯🇵 日本語.
- Ambas elecciones se recuerdan. La ventana **«Acerca de»** (ⓘ) muestra la versión, el titular de los derechos y la lista de idiomas.

## 11. Dónde se guardan los datos

- Las notas son archivos normales en la carpeta elegida y siguen siendo utilizables sin MD Notes.
- `.mdnotes/` es la carpeta de servicio de la bóveda: ajustes, plantillas, papelera, caché.
- `.mdnotes/cache/index.db` es el índice de búsqueda y enlaces. Solo es una caché: puedes eliminarla y se reconstruirá. La caché y la papelera quedan fuera de Git (`.mdnotes/.gitignore`).
- MD Notes no envía tus datos a ninguna parte: sin servidores, analítica ni telemetría.
- Haz copias de seguridad de la carpeta de la bóveda. Git es una forma cómoda de conservar versiones.

## 12. Teclado

| Acción | Windows / Linux | macOS |
|---|---|---|
| Guardar | `Ctrl+S` | `Cmd+S` |
| Selector rápido | `Ctrl+O` / `Ctrl+P` | `Cmd+O` / `Cmd+P` |
| Búsqueda | `Ctrl+Shift+F` | `Cmd+Shift+F` |
| Deshacer / rehacer | `Ctrl+Z` / `Ctrl+Shift+Z` | `Cmd+Z` / `Cmd+Shift+Z` |
| Sangría | `Tab` | `Tab` |

## 13. Problemas frecuentes

- **Windows muestra SmartScreen** — la compilación no está firmada. Elige «Más información → Ejecutar de todas formas».
- **macOS no abre la aplicación** — la compilación no está notarizada. Usa **Ajustes del Sistema → Privacidad y seguridad → Abrir igualmente**.
- **La bóveda no se abre al iniciar** — la carpeta se renombró o se movió. Ábrela de nuevo con **«Otra bóveda…»**.
- **La búsqueda no encuentra una nota que acabas de cambiar** — cierra y vuelve a abrir la bóveda. Si no basta, elimina `.mdnotes/cache/index.db` y el índice se reconstruirá.
- **Una imagen no se muestra** — comprueba que el archivo está dentro de la bóveda y que la ruta es relativa.

## 14. Derechos de autor

Copyright © 2026 Roman Zavada (Роман Завада). Todos los derechos reservados. MD Notes es software propietario. Tus notas te pertenecen. Consulta [LICENSE.md](../../LICENSE.md) y [LEGAL_AND_COPYRIGHT.md](../LEGAL_AND_COPYRIGHT.md).
