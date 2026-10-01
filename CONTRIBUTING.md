# Як долучитися

Документація ведеться українською, код, коментарі в коді й ідентифікатори —
англійською.

## Коміти

Повідомлення комітів і заголовки PR пишуться за
[Conventional Commits](https://www.conventionalcommits.org/). З них
автоматично формуються CHANGELOG і номер наступної версії.

| Префікс | Коли | Вплив на версію (до 1.0) |
|---|---|---|
| `feat:` | нова можливість | 0.x.**y** → 0.x.y+1 |
| `fix:` | виправлення | 0.x.**y** → 0.x.y+1 |
| `feat!:` / `BREAKING CHANGE:` | несумісна зміна | 0.**x**.y → 0.x+1.0 |
| `docs:`, `refactor:`, `test:`, `ci:`, `chore:` | інше | без релізу |

Можна вказувати область: `feat(core): …`, `fix(app): …`, `docs(adr): …`.

## Гілки і PR

- `main` завжди збирається і проходить CI.
- Робота ведеться в гілках `feat/…`, `fix/…`, `docs/…` і потрапляє в `main`
  через PR зі squash merge.
- CI перевіряє форматування (`cargo fmt`), лінтер (`cargo clippy`), тести ядра
  на Windows, macOS і Linux, а також типи і збірку інтерфейсу.

## Релізи

[release-please](https://github.com/googleapis/release-please) підтримує
відкритий PR «chore(main): release x.y.z». Злиття цього PR створює тег,
GitHub Release і запускає збірку інсталяторів для Windows, macOS і Linux.

## Архітектурні рішення

Суттєві рішення фіксуються як ADR у [docs/adr/](docs/adr/). Скопіюйте
[шаблон](docs/adr/template.md) з наступним номером.
