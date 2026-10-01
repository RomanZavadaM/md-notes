## Зміна

Коротко опишіть, що змінено і навіщо.

## Перевірки

- [ ] Прочитано `PROJECT_RULES.md`, `PROJECT_STATE.md` і `WORKLOG.md`
- [ ] `cargo fmt --all --check`
- [ ] `cargo clippy --workspace --all-targets -- -D warnings`
- [ ] `cargo test -p notes-core`
- [ ] `npm ci` і `npm run build` у `app/`
- [ ] Нові рядки інтерфейсу додано в усі мовні пакети (UK, EN, FR, DE, ES, KO, JA)
- [ ] User-visible зміну перевірено в запущеному застосунку (або вказано, що runtime-перевірки не було)
- [ ] Не додано особистих нотаток, сховищ, ключів, БД індексу або персональних документів
- [ ] Copyright/licensing notices не видалені й не підмінені; нові залежності внесено до `THIRD_PARTY_NOTICES.md`

## Релізний вплив

Вкажіть, чи потребує зміна нового patch/minor checkpoint і чи змінює формат файлів сховища (`.mdnotes/`, front matter, індекс).
