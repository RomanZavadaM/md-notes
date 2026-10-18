# MD Notes

[🇺🇦 Українська](../../README.md) · [🇬🇧 English](README.en.md) · [🇫🇷 Français](README.fr.md) · [🇩🇪 Deutsch](README.de.md) · **🇪🇸 Español** · [🇰🇷 한국어](README.ko.md) · [🇯🇵 日本語](README.ja.md)

> **Checkpoint de versión: MD Notes v0.3.1**
>
> **Estado del desarrollo: ACTIVE — etapa del roadmap v0.3 «plataformas móviles y sincronización».**

## Qué es

**MD Notes** es una aplicación multiplataforma local-first para una base de conocimiento Markdown. Trabaja con archivos `.md/.markdown`, adjuntos y metadatos JSON abiertos, sin un formato propietario oculto.

**Tus datos te pertenecen.** Las notas permanecen en tu sistema de archivos, pueden abrirse con cualquier editor y versionarse con Git. MD Notes no tiene servidor de aplicación, analítica ni telemetría.

Plataformas: **Windows, macOS, Linux**. Android e iOS son objetivos activos de v0.3; la CI valida la compilación, pero aún no se declara validado el runtime en dispositivos físicos.

## Funciones integradas

### v0.2 — base funcional completada

- bóveda local, árbol de archivos, crear/renombrar/papelera;
- CodeMirror 6, vista previa/split, guardado automático y escrituras atómicas;
- índice SQLite/FTS5, búsqueda, etiquetas y apertura rápida;
- wiki links, aliases, backlinks y actualización de enlaces al renombrar/mover;
- Mermaid 11, KaTeX, imágenes, adjuntos, plantillas y notas diarias;
- propiedades mediante `.mdnotes/schema.json`;
- presets Empty / PARA / Zettelkasten;
- grafo de conocimiento global/local;
- interfaz en siete idiomas y temas claro/oscuro/sistema.

### v0.3 — integrado hasta el checkpoint v0.3.1

- `StorageProvider` / `VaultStorage` neutrales respecto al backend;
- bóveda sandbox móvil para Android/iOS;
- build smoke Android + iOS en CI;
- base local-first de sincronización y estado persistente;
- base Git HTTPS con `gix`/gitoxide;
- detección de worktree modificado y commits Git locales;
- fetch HTTPS público seguro;
- autenticación HTTPS en memoria sin persistir tokens;
- almacenamiento del sistema para credenciales Git en Windows, macOS, iOS, Android y Linux;
- abrir/editar/guardar `.md/.markdown` sin vault;
- integración Markdown en Windows/Linux/macOS, con asociación predeterminada solo tras una acción explícita del usuario;
- planner Git pull: `UpToDate / FastForward / LocalAhead / Diverged / Unrelated`;
- preflight que bloquea un worktree modificado;
- fast-forward seguro solo para `FastForward` limpio, sin `hard reset` ciego, commit de merge ni sobrescritura silenciosa.

Aún **no está terminado**: Git push, integración completa con `SyncManifest` / ADR-0005, WebDAV, validación runtime en dispositivos Android/iOS físicos y carpetas externas opcionales Android SAF / iOS security-scoped.

Plan completo: [roadmap](../roadmap.md) (ucraniano, canónico).

## Instalación

Descarga los paquetes desde la [página de versiones](https://github.com/RomanZavadaM/md-notes/releases): Windows (`.exe`, `.msi`, ZIP portable), macOS (`.dmg`, `.app.tar.gz`) y Linux (`.AppImage`, `.deb`, `.rpm`).

El checkpoint **v0.3.1** incluye Markdown standalone y el pipeline Git fast-forward seguro. Las builds siguen sin firma/notarización salvo que una versión concreta indique lo contrario.

Guía completa: **[Guía del usuario](../user-guide/USER_GUIDE.es.md)**.

## Desarrollo, privacidad y aviso legal

Se necesita Rust stable, Node.js 20+ y los requisitos de Tauri. Los gates incluyen fmt/clippy/tests de Rust, build del frontend, comprobación de licencias, CI de escritorio y mobile smoke Android/iOS para cambios relevantes.

MD Notes es local-first. Los secretos de sincronización nunca deben guardarse en la bóveda, archivos Markdown, URL remotas, Git config, logs o `localStorage`. Los conflictos nunca deben sobrescribirse en silencio; el apply Git automático solo se permite para un fast-forward limpio.

Copyright © 2026 Roman Zavada (Роман Завада). Todos los derechos reservados. MD Notes es software propietario; el repositorio público no concede una licencia de código abierto. Tus notas te pertenecen. Consulta [LICENSE.md](../../LICENSE.md) y los [avisos legales](../LEGAL_AND_COPYRIGHT.md).
