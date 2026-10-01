# MD Notes

[🇺🇦 Українська](../../README.md) · [🇬🇧 English](README.en.md) · [🇫🇷 Français](README.fr.md) · [🇩🇪 Deutsch](README.de.md) · **🇪🇸 Español** · [🇰🇷 한국어](README.ko.md) · [🇯🇵 日本語](README.ja.md)

> **Checkpoint actual: [MD Notes v0.1.0](https://github.com/RomanZavadaM/md-notes/releases/tag/v0.1.0)**
>
> **Estado del desarrollo: ACTIVE — etapa v0.2 «estructura y enlaces».**

## Qué es

**MD Notes** es una aplicación multiplataforma y local-first para una base de conocimiento personal en Markdown: ver, editar, estructurar y visualizar archivos `.md` sencillos junto con sus adjuntos.

**Tus datos te pertenecen.** Las notas son archivos de texto abiertos. Puedes abrirlas en cualquier editor, verlas en GitHub y versionarlas con Git. La aplicación nunca crea formatos ocultos.

Plataformas: **Windows, macOS, Linux**; Android e iOS están previstos para la v0.3.

## Novedades de la v0.1.0

- una carpeta local como bóveda (vault); árbol de archivos con creación, cambio de nombre y papelera;
- editor CodeMirror 6, vista previa, modo en paralelo, guardado automático, escritura atómica;
- `[[enlaces wiki]]` que abren la nota de destino o la crean si no existe;
- `#etiquetas` y propiedades en YAML front matter;
- temas claro, oscuro y del sistema; diseño para pantallas estrechas;
- la base de conocimiento de ejemplo `sample-vault/`.

En desarrollo (v0.2): índice y búsqueda de texto completo, enlaces entrantes, actualización de enlaces al renombrar, Mermaid y KaTeX, plantillas y notas diarias, interfaz en siete idiomas. Plan completo: [roadmap](../roadmap.md) (en ucraniano).

## Instalación

Descarga el paquete para tu sistema desde la [página de la versión](https://github.com/RomanZavadaM/md-notes/releases/latest):

- **Windows** — `MD.Notes_<versión>_x64-setup.exe` o `.msi`. La compilación no está firmada, por lo que Windows puede mostrar SmartScreen.
- **macOS** — `.dmg` / `.app.tar.gz` (universal). La compilación no está notarizada; puede ser necesario **Ajustes del Sistema → Privacidad y seguridad → Abrir igualmente**.
- **Linux** — `.AppImage`, `.deb` o `.rpm`.

Tras iniciar, pulsa **«Abrir carpeta»** y elige una carpeta con notas o `sample-vault/` de este repositorio.

Guía completa: **[Guía del usuario](../user-guide/USER_GUIDE.es.md)**.

## Para desarrolladores

Requiere [Rust](https://rustup.rs/) (stable), [Node.js](https://nodejs.org/) 20+ y los [requisitos del sistema de Tauri](https://v2.tauri.app/start/prerequisites/). Ejecuta `npm ci` y `npm run tauri dev` en `app/`. Reglas de desarrollo: [PROJECT_RULES.md](../../PROJECT_RULES.md) (en ucraniano, versión de referencia).

## Privacidad y aviso legal

MD Notes es local-first: las notas, los adjuntos y el índice permanecen en tu dispositivo o en el almacenamiento que elijas. No hay servidores, analítica ni telemetría. Los conflictos de sincronización nunca se sobrescriben en silencio. Haz copias de seguridad de tus bóvedas.

Copyright © 2026 Roman Zavada (Роман Завада). Todos los derechos reservados. MD Notes es software propietario; el repositorio público no concede una licencia de código abierto. Tus notas te pertenecen. Consulta [LICENSE.md](../../LICENSE.md) y los [avisos legales](../LEGAL_AND_COPYRIGHT.md).
