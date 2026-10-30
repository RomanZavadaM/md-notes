# MD Notes

[🇺🇦 Українська](../../README.md) · [🇬🇧 English](README.en.md) · [🇫🇷 Français](README.fr.md) · [🇩🇪 Deutsch](README.de.md) · **🇪🇸 Español** · [🇰🇷 한국어](README.ko.md) · [🇯🇵 日本語](README.ja.md)

<!-- x-release-please-start-version -->
> **Checkpoint de versión: MD Notes v1.0.0**
>
> **Estado: desktop funcionalmente completo; stable v1.0.0 publicada. El proyecto pasa al modo de uso y estabilización.**

## Descargar MD Notes v1.0.0

**Instalación rápida:**

- **Windows 64-bit:** [instalador .exe](https://github.com/RomanZavadaM/md-notes/releases/download/v1.0.0/MD.Notes_1.0.0_x64-setup.exe) · [MSI](https://github.com/RomanZavadaM/md-notes/releases/download/v1.0.0/MD.Notes_1.0.0_x64_en-US.msi) · [ZIP portable](https://github.com/RomanZavadaM/md-notes/releases/download/v1.0.0/MD-Notes-1.0.0-Windows-x64-portable.zip)
- **macOS Universal:** [descargar DMG](https://github.com/RomanZavadaM/md-notes/releases/download/v1.0.0/MD.Notes_1.0.0_universal.dmg)
- **Linux x86_64:** [AppImage](https://github.com/RomanZavadaM/md-notes/releases/download/v1.0.0/MD.Notes_1.0.0_amd64.AppImage) · [DEB](https://github.com/RomanZavadaM/md-notes/releases/download/v1.0.0/MD.Notes_1.0.0_amd64.deb) · [RPM](https://github.com/RomanZavadaM/md-notes/releases/download/v1.0.0/MD.Notes-1.0.0-1.x86_64.rpm)

[**Todos los archivos de v1.0.0 →**](https://github.com/RomanZavadaM/md-notes/releases/tag/v1.0.0)
<!-- x-release-please-end -->

> Las builds todavía no están firmadas/notarizadas, por lo que el sistema puede mostrar una advertencia de seguridad estándar.

> **Empaquetado Windows 1.0:** NSIS/MSI incluyen el instalador offline de WebView2 y el runtime VC; el ZIP portable ligero utiliza el WebView2 Runtime ya instalado en Windows.

## Qué es

**MD Notes** es una aplicación multiplataforma local-first para una base de conocimiento Markdown. Trabaja con archivos `.md/.markdown`, adjuntos y metadatos JSON abiertos, sin un formato propietario oculto.

**Tus datos te pertenecen.** Las notas permanecen en tu sistema de archivos, pueden abrirse con cualquier editor y versionarse con Git. MD Notes no tiene servidor de aplicación, analítica ni telemetría.

Plataformas desktop 1.0: **Windows, macOS, Linux**. Android/iOS siguen compilándose en CI, pero la validación en dispositivos físicos y la publicación en stores móviles quedan para después de 1.0.

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
- física viva del grafo (grafo «flotante»), grafo en otra ventana, árbol de relaciones valorado por su eslabón más débil, camino entre dos notas;
- búsqueda de un sujeto por código exacto (EDRPOU/número fiscal) o nombre, importación de carpeta con fusión por código, posibles duplicados;
- interfaz en siete idiomas y temas claro/oscuro/sistema.

### v0.3 — integrado hasta el checkpoint v0.3.18

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
- fast-forward seguro solo para `FastForward` limpio, sin `hard reset` ciego, commit de merge ni sobrescritura silenciosa;
- bóveda cifrada opcional (age): las notas se guardan en disco como archivos `.age` y la clave privada vive solo en el almacén de secretos del sistema del dispositivo (Windows Credential Manager, macOS Keychain, Linux Secret Service); hay diálogos para crearla con copia única de la clave, importar la clave en otro dispositivo y una clave de recuperación opcional;
- Git push mediante el `git` del sistema (Git debe estar instalado) con una protección para bóvedas cifradas: nunca se envía texto en claro; panel Git en la aplicación (estado, fetch, pull, commit, push).

Aún **no está terminado**: integración completa con `SyncManifest` / ADR-0005, WebDAV, validación runtime en dispositivos Android/iOS físicos y carpetas externas opcionales Android SAF / iOS security-scoped.

La **bóveda cifrada** sigue siendo opcional: las bóvedas normales funcionan sin cambios y una build sin la feature `encrypted-vault` no contiene código de cifrado. En la estabilización 1.0, las imágenes y PDF se previsualizan descifrando solo en memoria y una bóveda existente puede añadir un destinatario público de recuperación desde el menú Archivo. La apertura externa de adjuntos cifrados está desactivada para evitar una copia en texto plano en disco; el índice `.mdnotes/cache` sigue sin cifrar y Git push conserva su protección contra texto plano.

Plan completo: [roadmap](../roadmap.md) (ucraniano, canónico).

> Las traducciones FR / DE / ES / KO / JA de este README, de la guía y de la interfaz son automáticas y no están revisadas por hablantes nativos; el texto ucraniano es el de referencia. Todo lo anterior está cubierto por pruebas automáticas y la build de GitHub; las comprobaciones manuales en la aplicación y en dispositivos Android/iOS reales aún no están confirmadas por el propietario.

## Instalación

Descarga los paquetes desde la [página de versiones](https://github.com/RomanZavadaM/md-notes/releases): Windows (`.exe`, `.msi`, ZIP portable), macOS (`.dmg`, `.app.tar.gz`) y Linux (`.AppImage`, `.deb`, `.rpm`).

El checkpoint **v0.3.18** incluye Markdown standalone y el pipeline Git fast-forward seguro. Las builds siguen sin firma/notarización salvo que una versión concreta indique lo contrario.

Guía completa: **[Guía del usuario](../user-guide/USER_GUIDE.es.md)**.

## Desarrollo, privacidad y aviso legal

Se necesita Rust stable, Node.js 20+ y los requisitos de Tauri. Los gates incluyen fmt/clippy/tests de Rust, build del frontend, comprobación de licencias, CI de escritorio y mobile smoke Android/iOS para cambios relevantes.

MD Notes es local-first. Los secretos de sincronización nunca deben guardarse en la bóveda, archivos Markdown, URL remotas, Git config, logs o `localStorage`. Los conflictos nunca deben sobrescribirse en silencio; el apply Git automático solo se permite para un fast-forward limpio.

Copyright © 2026 Roman Zavada (Роман Завада). Todos los derechos reservados. MD Notes es software propietario; el repositorio público no concede una licencia de código abierto. Tus notas te pertenecen. Consulta [LICENSE.md](../../LICENSE.md) y los [avisos legales](../LEGAL_AND_COPYRIGHT.md).


## Editor multiventana (`main` actual)

**v0.3.21** publica ADR-0010 M1–M3. Se pueden mantener varias notas del almacén abiertas en pestañas, cada una con su propio borrador. En escritorio, la nota activa se puede mover con **Abrir en una ventana separada** a una ventana de editor independiente. Los cambios se guardan antes de moverla y la ventana principal vuelve a la pestaña anterior o a un área vacía.

M1–M3 forman parte de **v0.3.21** publicada: protección de conflictos, cierre seguro de editores separados y guardado global/salida segura. CI de Windows/macOS/Linux y los build smoke de Android/iOS pasan. **M4 ahora significa aceptación manual en tiempo de ejecución**; la prerelease de prueba ya está publicada.


## Copia de seguridad y restauración

El menú **Archivo** incluye **Crear copia de seguridad…** y **Restaurar copia de seguridad…**.

- la copia es un ZIP portátil con `manifest.json` y SHA-256 para cada archivo;
- MD Notes guarda las notas abiertas antes de crear la copia;
- se incluyen notas, adjuntos, configuración, plantillas y papelera; se excluyen `.git`, `.mdnotes/cache` y archivos temporales del sistema;
- las bóvedas cifradas se copian **sin descifrar**; la clave privada/de recuperación nunca se guarda en el archivo;
- la restauración verifica primero rutas, tamaños y sumas de control y después crea una **nueva** carpeta de bóveda sin sobrescribir la existente.

## Principios y procedimientos detallados

- [Enlaces y grafo de conocimiento](../concepts/KNOWLEDGE_GRAPH.es.md)
- [Almacén cifrado](../security/ENCRYPTED_VAULT.es.md)
- [Backup y recuperación](../security/BACKUP_AND_RECOVERY.es.md)
- [Contratos canónicos para IA/automatización](../ai/README.md)

