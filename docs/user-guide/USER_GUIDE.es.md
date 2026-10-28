# MD Notes — Guía del usuario

[🇺🇦 Українська](USER_GUIDE.uk.md) · [🇬🇧 English](USER_GUIDE.en.md) · [🇫🇷 Français](USER_GUIDE.fr.md) · [🇩🇪 Deutsch](USER_GUIDE.de.md) · **🇪🇸 Español** · [🇰🇷 한국어](USER_GUIDE.ko.md) · [🇯🇵 日本語](USER_GUIDE.ja.md)

Aplicable a la **v0.3.19**. La versión ucraniana es el texto de referencia.

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

### Bóveda cifrada (opcional)

- Al crear la bóveda, marque **«Cifrar esta bóveda»**. Las notas se guardan en disco como archivos `.age`, ilegibles sin la clave.
- Tras crearla, la clave privada (`AGE-SECRET-KEY-…`) se muestra **una sola vez**. Guárdela en un gestor de contraseñas o sin conexión: no se puede recuperar. De forma permanente, la clave solo vive en el almacén de secretos del sistema de este dispositivo.
- Puede indicar una **clave pública de recuperación** (`age1…`): también abre la bóveda si se pierde la clave principal.
- En otro ordenador, copie la carpeta de la bóveda y ábrala: la aplicación pedirá la clave. Pegue la clave privada y se guardará en el almacén de secretos de ese ordenador.
- Por ahora una bóveda cifrada no muestra imágenes ni adjuntos, y el índice `.mdnotes/cache` no está cifrado.

### Git (solo escritorio)

El botón **Git** (o «Git…» en el menú) abre un panel: crear un repositorio, indicar la dirección remota, guardar el acceso (el token se guarda solo en el almacén de secretos del sistema) y **fetch**, **pull**, **commit** y **push**. El pull solo se ejecuta si no hay cambios sin guardar y el remoto simplemente tiene novedades; si los historiales divergen, hace falta una decisión manual. Para el push debe estar instalado Git. En una bóveda cifrada, el push comprueba antes que todos los archivos estén cifrados y, si no, se bloquea.

## 12. Grafo, árbol de relaciones y búsqueda de un sujeto

### Grafo

El panel **«Enlaces»** tiene las pestañas «Enlaces», «Grafo local» y «Grafo global». Un nodo es una nota, su color es el `type` de la nota, una línea es un enlace `[[…]]` o una relación tipada. Haga clic en un nodo para abrir la nota.

- **Física viva (grafo «flotante»).** Arrastre un nodo: sus vecinos lo siguen y lo rodean, y al soltarlo el grafo se calma suavemente. El botón **«Flotar»** añade una ligera deriva (activado por defecto hasta 400 nodos), **«Pausa»** detiene el movimiento. **«Fuerzas»** ofrece controles de centro, repulsión, fuerza y longitud de los enlaces; su elección se recuerda.
- **«Formas» y «Leyenda».** El botón **«Formas»** activa un modo en el que una **forma** indica el tipo de nota (persona — cabeza y hombros, empresa — cuadrado, institución — hexágono, caso — rombo, material — triángulo, grupo — tres puntos) y el **color del nodo** indica la **fiabilidad** de la mejor valoración de sus relaciones (verde 1–2, ámbar 3–4, rojo 5–6, gris — sin valoración). Abra **«Leyenda»** para ver todas las formas, colores y estilos de línea; un borde discontinuo y la insignia «Ч» indican un rumor, y el color nunca es la única señal.
- **Filtro «Mostrar relaciones».** La lista del panel del grafo conserva solo las relaciones con al menos la valoración elegida («fiables» — 1–2, «fiables y medias» — 1–4); las relaciones sin valoración y los enlaces simples se ocultan, los nodos permanecen.
- **Relaciones tipadas** en la propiedad `relations` (`to`, además de `type`, `grade`, `origin`, `status`, `evidence` y `dir: in` para el sentido inverso). Un `grade` es una letra y un dígito (A1 el más fiable, F6 el más débil): la letra valora la fuente, el dígito la afirmación. La línea y el borde del nodo siguen la valoración; las valoraciones 5–6 llevan la palabra **«RUMOR»**. El color nunca es la única señal.
- **«De dónde viene»** resalta la cadena de propietarios por encima del nodo elegido y los materiales a su alrededor.
- **Ventana aparte.** El botón **↗** de las pestañas del grafo, o «Archivo → Grafo en otra ventana» (solo escritorio), abre el grafo en su propia ventana con más espacio, por ejemplo en un segundo monitor. Sigue la nota activa, se actualiza tras los cambios y un clic en un nodo abre la nota en la ventana principal.
- **Botón «Atrás».** En la ventana del grafo vuelve a la nota anterior.

### Árbol de relaciones

La pestaña **«Árbol de relaciones»** muestra la cadena de relaciones tipadas desde la nota elegida: **«Propietarios y fuentes»** (hacia arriba) o **«Lo que posee»** (hacia abajo), profundidad 1–8.

- Cada nodo muestra el tipo de relación y la valoración de la línea; al lado, la valoración de **«cadena»**: el eslabón más débil desde la raíz (B2 junto con C1 da C2). Una cadena es tan fiable como su eslabón más débil.
- **«−/+»** pliega una rama, **◎** reconstruye el árbol desde otro nodo, **«Desplegar todo» / «Plegar todo»** controlan todas las ramas. Un nodo repetido se marca y no se despliega dos veces; el árbol se limita a 400 nodos.
- **«Camino entre dos notas»** encuentra la cadena más corta de relaciones tipadas (se ignora el sentido de las líneas) y muestra su eslabón más débil. La casilla **«El más fiable, no el más corto»** busca la cadena cuyo eslabón más débil sea el mejor (con menos pasos a igual fiabilidad).
- El árbol usa solo relaciones tipadas explícitas, no enlaces simples. La valoración de la cadena es una ayuda, no una conclusión jurídica.

### Buscar un sujeto por código o nombre

En la pestaña **«Búsqueda»**, un número de 8 dígitos se busca como código EDRPOU y uno de 10 como número fiscal (un código que perdió ceros iniciales, de 6–7 dígitos, se restituye). Los resultados se muestran por separado: Para un código de 8 dígitos aparece debajo el bloque **«Base «З-під Ляди» por código»**: elija la carpeta con los datos abiertos del sitio (copia local de la rama data) y muestra los hechos del código exacto — fecha de registro, estado y países extranjeros de fundadores y beneficiarios (registro), decisiones de la autoridad de competencia, contrataciones de Prozorro como comprador y ganador. Solo lectura, no se envía nada por la red; son hechos, no valoraciones.

- **«Coincidencia exacta»**: una nota con el mismo código en la propiedad `edrpou` o `ipn` (o el mismo nombre tras la normalización);
- **«Candidatos por nombre»**: nombres parecidos; una coincidencia de nombre **no es una prueba**, confírmela con un código o un documento;
- **«En el texto»**: búsqueda de texto completo habitual.

Los nombres se comparan sin distinguir mayúsculas, comillas, guiones ni formas jurídicas (S.L. y similares); las letras latinas que parecen cirílicas se reducen al cirílico.

### Importar una carpeta, fusionar por código y posibles duplicados

- **«Archivo → Importar carpeta…»** añade las notas de una carpeta. Una nota cuyo código (`edrpou`/`ipn`) ya está en la bóveda se **fusiona** con la existente en lugar de duplicarse: se añaden las propiedades que faltan, se unen etiquetas, alias y relaciones, el título de la nota importada pasa a ser alias, los valores distintos no se sobrescriben y se listan para revisión. Volver a importar la misma carpeta no cambia nada.
- **«Archivo → Posibles duplicados…»** lista grupos de notas con el mismo título o alias normalizado. Es solo una pista: no se cambia nada y las notas con códigos distintos nunca se agrupan.

## 13. Teclado

| Acción | Windows / Linux | macOS |
|---|---|---|
| Guardar | `Ctrl+S` | `Cmd+S` |
| Selector rápido | `Ctrl+O` / `Ctrl+P` | `Cmd+O` / `Cmd+P` |
| Búsqueda | `Ctrl+Shift+F` | `Cmd+Shift+F` |
| Deshacer / rehacer | `Ctrl+Z` / `Ctrl+Shift+Z` | `Cmd+Z` / `Cmd+Shift+Z` |
| Sangría | `Tab` | `Tab` |

## 14. Problemas frecuentes

- **Windows muestra SmartScreen** — la compilación no está firmada. Elige «Más información → Ejecutar de todas formas».
- **macOS no abre la aplicación** — la compilación no está notarizada. Usa **Ajustes del Sistema → Privacidad y seguridad → Abrir igualmente**.
- **La bóveda no se abre al iniciar** — la carpeta se renombró o se movió. Ábrela de nuevo con **«Otra bóveda…»**.
- **La búsqueda no encuentra una nota que acabas de cambiar** — cierra y vuelve a abrir la bóveda. Si no basta, elimina `.mdnotes/cache/index.db` y el índice se reconstruirá.
- **Una imagen no se muestra** — comprueba que el archivo está dentro de la bóveda y que la ruta es relativa.

## 15. Derechos de autor

Copyright © 2026 Roman Zavada (Роман Завада). Todos los derechos reservados. MD Notes es software propietario. Tus notas te pertenecen. Consulta [LICENSE.md](../../LICENSE.md) y [LEGAL_AND_COPYRIGHT.md](../LEGAL_AND_COPYRIGHT.md).


## Pestañas y ventanas de editor separadas

En el `main` actual después de v0.3.20 se pueden mantener varias notas del almacén abiertas en pestañas. Cada pestaña conserva su propio borrador, por lo que cambiar de nota no debe descartar texto sin guardar.

En escritorio, la nota activa del almacén ofrece **Abrir en una ventana separada**. MD Notes guarda el borrador actual antes de separarlo. Tras crear correctamente la ventana independiente, la nota se elimina de las pestañas del editor principal y la ventana principal vuelve a la pestaña anterior; si no queda ninguna, muestra un editor vacío manteniendo disponible el árbol del almacén. La ventana separada admite Editor / lado a lado / vista previa y guardado automático.

Un archivo externo `.md/.markdown` sigue siendo un documento independiente y no se importa automáticamente al almacén. M3 ya está integrado: si otra ventana o proceso cambia el archivo después de abrirlo, MD Notes bloquea la sobrescritura silenciosa; una ventana separada con cambios permanece abierta hasta que el guardado protegido termine correctamente; Salir procesa primero todos los documentos abiertos de forma segura. El modo multiventana todavía necesita la prueba manual M4.
