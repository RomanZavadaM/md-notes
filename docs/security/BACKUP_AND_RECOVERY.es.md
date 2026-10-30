# Backup y recuperación — funcionamiento y procedimiento seguro

[🇺🇦 Українська](BACKUP_AND_RECOVERY.uk.md) · [🇬🇧 English](BACKUP_AND_RECOVERY.en.md) · [🇫🇷 Français](BACKUP_AND_RECOVERY.fr.md) · [🇩🇪 Deutsch](BACKUP_AND_RECOVERY.de.md) · **🇪🇸 Español** · [🇰🇷 한국어](BACKUP_AND_RECOVERY.ko.md) · [🇯🇵 日本語](BACKUP_AND_RECOVERY.ja.md)

> Válido para MD Notes v0.3.23. El ucraniano es canónico. Backup/restore está implementado desde v0.3.22; existen pruebas desktop automatizadas, pero no hay registrado un test runtime completo separado por el propietario.

## 1. Backup, sincronización y Git no son lo mismo

Un **backup** es un snapshot ZIP autónomo para recuperación. La **sincronización** mantiene copias actuales y también puede propagar errores o borrados. **Git** conserva historial, pero no contiene automáticamente todos los datos locales ni sustituye una copia de las claves de un vault cifrado.

Mantenga al menos un backup independiente del vault de trabajo y de su canal de sincronización.

## 2. Qué hace “Crear backup…”

La orden del menú Archivo:
1. intenta guardar documentos abiertos;
2. recorre los archivos físicos del vault;
3. excluye datos no portables;
4. calcula SHA-256 de cada archivo incluido;
5. escribe el ZIP;
6. añade un `manifest.json` abierto;
7. comprueba de nuevo que el origen no cambió durante la operación;
8. solo entonces finaliza el archivo.

Si el vault cambia mientras se crea el snapshot, la operación debe fallar en vez de guardar silenciosamente un estado mezclado.

## 3. Incluido

Notas Markdown, adjuntos, configuración/schema/templates/views portables, `.mdnotes/trash/` y configuración del vault cifrado cuando corresponda.

## 4. Excluido

- `.git/`;
- `.mdnotes/cache/`;
- archivos temporales de MD Notes;
- `.DS_Store`, `Thumbs.db`, `desktop.ini`;
- claves privadas age/recovery.

Los enlaces simbólicos se rechazan para evitar que la copia salga del perímetro del vault.

## 5. Manifest

`manifest.json` registra formato `md-notes-backup`, versión del formato, versión de la aplicación, fecha, nombre del vault, indicador encrypted y, para cada archivo, ruta relativa, tamaño y SHA-256.

Sirve para **integridad y completitud**, no para cifrado. Es metadato abierto.

## 6. Límites v1

Hasta 100 000 archivos, 4 GiB por archivo y 16 GiB en total; sin rutas absolutas, `..`, path traversal, duplicados ni colisiones que solo difieran por mayúsculas/minúsculas.

## 7. Validación antes de restaurar

MD Notes primero valida manifest, formato, rutas, entradas ZIP, tamaños y SHA-256; también comprueba que no falten archivos ni sobren entradas. Cualquier error detiene la restauración antes de crear un vault utilizable.

## 8. Destino

Tras validar, se extrae a una carpeta temporal y se promueve a un **nuevo** vault. Nunca se sobrescribe silenciosamente una carpeta existente: `My Vault-restored`, `My Vault-restored-2`, etc.

## 9. Vault normal

El ZIP contiene Markdown y adjuntos legibles. Debe protegerse como la carpeta original.

## 10. Vault cifrado

El backup lee el estado físico, así que los archivos age permanecen cifrados. `.mdnotes/encryption.json` y ciertos metadatos siguen abiertos; cache y claves privadas quedan fuera.

La recuperación completa necesita:
1. backup del vault;
2. al menos una clave privada age válida.

Véase [Vault cifrado](ENCRYPTED_VAULT.es.md).

## 11. Modelo recomendado

Copia de trabajo + backups periódicos en otro soporte/ubicación + recovery key separada para vault cifrado. Para datos críticos, el principio 3-2-1 es una buena referencia.

## 12. Procedimiento de backup

Guardar cambios → crear backup → guardar ZIP fuera del vault → confirmar éxito → copiar a almacenamiento independiente → para vault cifrado confirmar que la recovery key está aparte.

## 13. Procedimiento de restore

No borrar el vault anterior → restaurar a nueva carpeta → dejar completar manifest/checksums → abrir `*-restored` → revisar notas importantes, adjuntos y grafo → importar clave si hace falta → solo entonces decidir qué hacer con la copia antigua.

## 14. Simulacro de recuperación

Periódicamente restaure un backup actual en una carpeta separada y ábralo de verdad. Para un vault cifrado, confirme también que dispone de una clave recovery utilizable.

## 15. Errores comunes

Backup en el mismo disco físico; ciphertext sin clave; clave sin copia actual; confundir sync con backup; publicar un ZIP de vault normal; borrar el vault viejo demasiado pronto; crear backup durante reescrituras externas masivas.

## 16. Error de checksum

No modifique el manifest para “arreglarlo”. Una discrepancia significa que el archivo ya no coincide con el snapshot registrado. Conserve el ZIP para análisis y use otra copia.

## 17. IA y automatización

Una IA no debe borrar la última copia buena antes de un restore verificado, tratar sync/Git como backup equivalente, meter claves privadas en ZIP, saltarse validaciones, extraer sobre un vault existente ni declarar éxito sin comprobar el nuevo vault.

Reglas encrypted: [ENCRYPTED_VAULT_CONTRACT.md](../ai/ENCRYPTED_VAULT_CONTRACT.md).

> Traducción generada automáticamente y no revisada por un hablante nativo; el ucraniano es canónico.

## Verificación recovery para 1.0

Antes de desktop 1.0 se añadió un recovery drill end-to-end automatizado para bóvedas cifradas:

1. se crea una bóveda cifrada real con un destinatario recovery separado;
2. se escribe una nota Markdown y se importa un adjunto binario;
3. se crea el backup ZIP portable;
4. el backup se restaura en una carpeta nueva;
5. la bóveda restaurada se vuelve a abrir con la **recovery private key**;
6. se leen la nota restaurada y los bytes del adjunto mediante el proveedor de almacenamiento cifrado.

Este escenario pasó CI antes de 1.0 en los gates core de Windows, macOS y Linux. Es evidencia automatizada del mecanismo recovery y no sustituye un ensayo manual separado del propietario con datos reales.

