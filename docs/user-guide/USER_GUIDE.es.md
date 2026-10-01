# MD Notes — Guía del usuario

[🇺🇦 Українська](USER_GUIDE.uk.md) · [🇬🇧 English](USER_GUIDE.en.md) · [🇫🇷 Français](USER_GUIDE.fr.md) · [🇩🇪 Deutsch](USER_GUIDE.de.md) · **🇪🇸 Español** · [🇰🇷 한국어](USER_GUIDE.ko.md) · [🇯🇵 日本語](USER_GUIDE.ja.md)

Aplicable a la **v0.1.0**. La versión ucraniana es el texto de referencia.

## 1. Primer inicio

1. Instala MD Notes para tu sistema (consulta el README).
2. Pulsa **«Abrir carpeta»** y elige una carpeta con notas. Sirve cualquier carpeta con archivos `.md`, incluida `sample-vault/` del repositorio.
3. MD Notes recuerda la carpeta y la abre en el siguiente inicio.

Abrir una carpeta no la modifica. La carpeta de servicio `.mdnotes/` solo se crea cuando de verdad hace falta (papelera, ajustes, índice).

## 2. La ventana

- **Barra de herramientas** — botón del panel lateral ☰, nombre de la bóveda, título de la nota abierta (un punto indica cambios sin guardar), los modos «Editor» / «En paralelo» / «Vista previa» y el selector de tema.
- **Panel lateral** — el árbol de archivos de la bóveda. Las carpetas aparecen primero; las carpetas ocultas (`.mdnotes`, `.git`) no se muestran. Los archivos de otros formatos aparecen en gris.
- **Área de trabajo** — el editor, la vista previa o ambos en paralelo.
- **Barra de estado** — ruta de la nota, etiquetas, número de enlaces, errores en las propiedades y estado del guardado.

En pantallas estrechas, el panel lateral se abre sobre el contenido y se oculta al elegir una nota.

## 3. Notas y carpetas

- **+ Nota** crea una nota en la carpeta seleccionada (o junto al archivo seleccionado). El título pasa a ser el nombre del archivo; se eliminan los caracteres no permitidos en nombres de archivo.
- Una nota nueva recibe las propiedades `id` (identificador estable), `type: note` y `created`.
- **+ Carpeta** crea una carpeta.
- **✎** cambia el nombre del elemento seleccionado. La extensión `.md` se añade automáticamente.
- **🗑** mueve el elemento a la papelera de la bóveda `.mdnotes/trash/`. Nada se elimina definitivamente: el archivo se puede devolver a mano.

## 4. Edición

- Los cambios se guardan automáticamente poco después de dejar de escribir y también con `Ctrl+S` / `Cmd+S`.
- La escritura es atómica: un fallo a mitad del guardado nunca deja un archivo dañado.
- El modo **«En paralelo»** muestra el editor y la vista previa a la vez. La vista previa admite GitHub Flavored Markdown: tablas, listas de tareas, tachado.

## 5. Enlaces

- `[[Título de la nota]]` enlaza con otra nota y es clicable en la vista previa.
- `[[Título|texto]]` muestra otro texto, `[[Título#Sección]]` enlaza con una sección.
- Si no existe una nota con ese nombre, al hacer clic se crea.
- Los enlaces se resuelven por ruta y luego por nombre de archivo, sin distinguir mayúsculas. Los enlaces dentro de código se ignoran.
- Los enlaces normales `https://…` se abren en el navegador del sistema.

## 6. Etiquetas y propiedades

- `#etiqueta` en el texto o una lista `tags` en las propiedades marca un tema. Se admiten etiquetas anidadas: `#proyecto/diseño`.
- Las propiedades se escriben como YAML front matter al principio del archivo, entre líneas `---`.
- En la vista previa, las propiedades aparecen plegadas en el bloque **«Propiedades»**.
- Si el YAML contiene un error, la barra de estado lo muestra y la nota se abre como texto normal.

## 7. Temas

Elige **Sistema**, **Claro** u **Oscuro** en la esquina derecha de la barra de herramientas. La elección se recuerda.

## 8. Dónde se guardan los datos

- Las notas son archivos normales en la carpeta elegida y siguen siendo utilizables sin MD Notes.
- `.mdnotes/` es la carpeta de servicio de la bóveda: ajustes, plantillas, papelera, caché.
- MD Notes no envía tus datos a ninguna parte: sin servidores, analítica ni telemetría.
- Haz copias de seguridad de la carpeta de la bóveda. Git es una forma cómoda de conservar versiones.

## 9. Teclado

| Acción | Windows / Linux | macOS |
|---|---|---|
| Guardar | `Ctrl+S` | `Cmd+S` |
| Deshacer / rehacer | `Ctrl+Z` / `Ctrl+Shift+Z` | `Cmd+Z` / `Cmd+Shift+Z` |
| Sangría | `Tab` | `Tab` |

## 10. Problemas frecuentes

- **Windows muestra SmartScreen** — la compilación no está firmada. Elige «Más información → Ejecutar de todas formas».
- **macOS no abre la aplicación** — la compilación no está notarizada. Usa **Ajustes del Sistema → Privacidad y seguridad → Abrir igualmente**.
- **La bóveda no se abre al iniciar** — la carpeta se renombró o se movió. Ábrela de nuevo con **«Otra bóveda…»**.
- **Una nota no se abre** — MD Notes solo abre archivos `.md` y `.markdown`.

## 11. Derechos de autor

Copyright © 2026 Roman Zavada (Роман Завада). Todos los derechos reservados. MD Notes es software propietario. Tus notas te pertenecen. Consulta [LICENSE.md](../../LICENSE.md) y [LEGAL_AND_COPYRIGHT.md](../LEGAL_AND_COPYRIGHT.md).
