---
id: "{{id}}"
type: project
created: "{{date}}"
status: idea
start: 
due: 
tags: []
---

# {{title}}

## Мета

## Задачі

```query
type: task
where: project = [[{{title}}]]
sort: due asc
view: table
columns: [title, status, due, priority]
```

## Нотатки

