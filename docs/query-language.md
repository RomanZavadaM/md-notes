# Мова запитів

> Статус: специфікація. Реалізація запланована на v0.3 (таблиці й запити).

Запити дозволяють вибрати нотатки за властивостями й показати їх у будь-якому
вигляді: таблицею, канбаном, календарем, списком. Запит можна вбудувати в
нотатку або зберегти окремим файлом `*.view.json`.

## Вбудований запит

````markdown
```query
type: task
where: status != done and project = [[MD Notes]]
sort: due asc, priority desc
view: table
columns: [title, status, due, priority]
limit: 50
```
````

Блок `query` — це YAML. GitHub та інші редактори покажуть його як звичайний
код, тож нотатка залишається читабельною поза застосунком.

## Ключі

| Ключ | Обов'язковий | Значення |
|---|---|---|
| `from` | ні | папка або тег: `Проєкти/`, `#mdnotes`. Можна список. За замовчуванням усе сховище. |
| `type` | ні | тип нотатки або список типів |
| `where` | ні | вираз фільтра (див. граматику) |
| `sort` | ні | `поле [asc\|desc]`, через кому |
| `group` | ні | поле для групування (колонки канбану, розділи таблиці) |
| `view` | ні | `table` (за замовчуванням), `list`, `cards`, `kanban`, `calendar`, `timeline`, `gallery` |
| `columns` | ні | поля для таблиці або карток |
| `limit` | ні | максимальна кількість результатів |

## Граматика `where`

```ebnf
expr        = or_expr ;
or_expr     = and_expr , { "or" , and_expr } ;
and_expr    = not_expr , { "and" , not_expr } ;
not_expr    = [ "not" ] , primary ;
primary     = comparison | "(" , expr , ")" ;
comparison  = field , op , value
            | field , "in" , list
            | field , "contains" , value
            | field , "exists"
            | "has" , tag ;
op          = "=" | "!=" | "<" | "<=" | ">" | ">=" ;
field       = identifier , { "." , identifier } ;      (* file.name, file.folder *)
value       = string | number | date | bool | "null" | link | relative_date ;
list        = "[" , [ value , { "," , value } ] , "]" ;
link        = "[[" , text , "]]" ;
tag         = "#" , tag_chars ;
string      = '"' , { char } , '"' | bare_word ;
date        = digit4 , "-" , digit2 , "-" , digit2 ;
relative_date = "today" | "tomorrow" | "yesterday"
              | ( "today" , ( "+" | "-" ) , number , ( "d" | "w" | "m" ) ) ;
```

Ключові слова (`and`, `or`, `not`, `in`, `contains`, `exists`, `has`) не
чутливі до регістру.

### Семантика

- Порівняння рядків не чутливе до регістру.
- `link` дорівнює властивості, якщо обидві вказують на одну нотатку (з
  урахуванням `aliases`), а не лише за збігом тексту.
- Для списків (`tags`, `participants`) `=` означає «містить елемент».
- Нотатка без поля не проходить `=`, `<`, `>`, але проходить `!=`.
- Дати порівнюються як дати. `today` обчислюється в часовому поясі пристрою.

### Віртуальні поля

| Поле | Значення |
|---|---|
| `file.name` | ім'я файлу без розширення |
| `file.path` | шлях у сховищі |
| `file.folder` | папка |
| `file.created`, `file.modified` | час файлу у файловій системі |
| `file.links` | вихідні посилання |
| `file.backlinks` | кількість зворотних посилань |
| `file.tags` | усі теги (властивість і текст) |

## Приклади

```yaml
# Прострочені задачі
type: task
where: status != done and due < today
sort: due asc
```

```yaml
# Канбан проєкту
type: task
where: project = [[MD Notes]]
group: status
view: kanban
```

```yaml
# Зустрічі цього тижня в календарі
type: meeting
where: date >= today-7d
view: calendar
```

## Файл вигляду `*.view.json`

```json
{
  "version": 1,
  "name": "Задачі MD Notes",
  "query": {
    "type": "task",
    "where": "project = [[MD Notes]]",
    "group": "status",
    "view": "kanban"
  },
  "options": { "columnOrder": ["todo", "doing", "done"] }
}
```
