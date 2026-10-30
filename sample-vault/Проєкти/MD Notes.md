---
id: 01K6F0A0000000000000000002
type: project
status: doing
start: 2026-09-15
due: 2027-06-30
tags: [mdnotes, project]
related: ["[[Local-first]]"]
created: 2026-09-15
---

# MD Notes

Кросплатформний застосунок для бази знань у Markdown. Принципи —
[[Local-first]] і відкриті файли.

![Шари архітектури](../attachments/2026/10/layers.svg)

## Задачі

```query
type: task
where: project = [[MD Notes]]
sort: due asc
view: table
columns: [title, status, due, priority]
```

У stable v1.0.0 мова запитів із прикладу вище ще не реалізована, тому нижче — звичайний список:

- [x] [[Дерево файлів]]
- [x] [[Граф знань]]
- [ ] [[Мобільна версія]] — build smoke є; фізичне приймання пристроїв post-1.0

## Люди

- [[Олена Коваль]] — дизайн інтерфейсу

#mdnotes
