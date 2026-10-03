# MD Notes

[🇺🇦 Українська](../../README.md) · [🇬🇧 English](README.en.md) · [🇫🇷 Français](README.fr.md) · [🇩🇪 Deutsch](README.de.md) · **🇪🇸 Español** · [🇰🇷 한국어](README.ko.md) · [🇯🇵 日本語](README.ja.md)

> **Último checkpoint publicado: [MD Notes v0.2.2](https://github.com/RomanZavadaM/md-notes/releases/tag/v0.2.2)**
>
> **Estado del desarrollo: ACTIVE — etapa del roadmap v0.3 «plataformas móviles y sincronización».**

## Qué es

**MD Notes** es una aplicación multiplataforma local-first para una base de conocimiento en Markdown. Trabaja con archivos `.md` normales y adjuntos, sin un formato de datos propietario oculto.

**Tus datos te pertenecen.** Las notas pueden abrirse con cualquier editor, guardarse en tu propio sistema de archivos y versionarse con Git. MD Notes no tiene servidor de aplicación, analítica ni telemetría.

Plataformas: **Windows, macOS, Linux**. Android e iOS están en desarrollo activo dentro de v0.3; la CI valida que compilan, pero todavía no se declara validado el runtime en dispositivos físicos.

## Lo que ya está implementado

### v0.2 — base funcional completada

- bóveda local, árbol de archivos, crear/renombrar/papelera;
- CodeMirror 6, vista previa/split, guardado automático y escrituras atómicas;
- índice SQLite/FTS5, búsqueda, etiquetas y apertura rápida;
- wiki links, aliases, backlinks y actualización de enlaces al renombrar/mover;
- Mermaid 11, KaTeX, imágenes, adjuntos, plantillas y notas diarias;
- propiedades controladas por el esquema abierto `.mdnotes/schema.json`;
- presets Empty / PARA / Zettelkasten;
- grafo de conocimiento global/local;
- interfaz en siete idiomas y temas claro/oscuro/sistema.

### v0.3 — desarrollo activo

Ya integrado en `main`:

- `StorageProvider` / `VaultStorage` neutrales respecto al backend;
- bóveda sandbox móvil para Android/iOS;
- build smoke de Android + iOS en CI;
- base de decisiones de sincronización local-first y estado persistente;
- base Git HTTPS con `gix`/gitoxide;
- detección de worktree modificado;
- pipeline de commit Git local sin depender de la configuración Git del usuario/sistema;
- fetch HTTPS público seguro;
- autenticación HTTPS en memoria sin persistir el token en Git config, la bóveda o la URL remota.

El checkpoint de seguridad actual añade **almacenamiento del sistema para credenciales Git** en la capa Tauri: Windows Credential Manager, macOS Keychain, iOS Protected Data, almacenamiento Android respaldado por Keystore y Linux Secret Service. El frontend puede guardar/comprobar/borrar credenciales, pero no puede leer de vuelta el token; el fetch autenticado obtiene el secreto únicamente dentro de Rust justo antes de la llamada de red.

Aún **no está terminado**: política Git pull/merge, push, integración completa con la política de conflictos, WebDAV, validación runtime en dispositivos Android/iOS físicos y carpetas externas opcionales mediante Android SAF / iOS security-scoped.

Plan completo: [roadmap](../roadmap.md) (ucraniano, canónico).

## Instalación

Descarga los paquetes desde la [página de versiones](https://github.com/RomanZavadaM/md-notes/releases): Windows (`.exe`, `.msi`, ZIP portable), macOS (`.dmg`, `.app.tar.gz`) y Linux (`.AppImage`, `.deb`, `.rpm`).

El prerelease publicado más reciente es **v0.2.2**. Las compilaciones todavía no están firmadas/notarizadas, por lo que el sistema puede mostrar advertencias de seguridad estándar.

Guía completa: **[Guía del usuario](../user-guide/USER_GUIDE.es.md)**.

## Desarrollo, privacidad y aviso legal

Se necesita Rust stable, Node.js 20+ y los requisitos del sistema de Tauri. Los gates de integración incluyen format/clippy/tests de Rust, build del frontend, comprobación de licencias, CI de escritorio y mobile smoke Android/iOS cuando cambia la frontera mobile/Tauri.

MD Notes es local-first. Los secretos de sincronización nunca deben guardarse en la bóveda, archivos Markdown, URL remotas, Git config, logs o `localStorage`; deben permanecer en el almacén de credenciales del sistema operativo. Los conflictos de sincronización nunca deben sobrescribirse en silencio.

Copyright © 2026 Roman Zavada (Роман Завада). Todos los derechos reservados. MD Notes es software propietario; el repositorio público no concede una licencia de código abierto. Tus notas te pertenecen. Consulta [LICENSE.md](../../LICENSE.md) y los [avisos legales](../LEGAL_AND_COPYRIGHT.md).
