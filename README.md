# Urich Demo — Сотрудники и задачи

Демо-приложение на [Urich](https://github.com/KashN9sh/urich): приложение собирается из модулей. Первым регистрируется **DatabaseModule** (engine и фабрика сессий в контейнере), затем контексты **employees** и **tasks**; репозитории получают `SessionFactory` через DI.

## Установка и запуск

```bash
pip install urich uvicorn
uvicorn main:app --reload
```

Открой [http://localhost:8000/docs](http://localhost:8000/docs) — Swagger UI со всеми командами и запросами.

### Docker

```bash
docker build -t urich-demo .
docker run -p 8000:8000 urich-demo
```

### Docker Compose (приложение + PostgreSQL)

```bash
docker compose up --build
```

- Приложение: http://localhost:8000/docs  
- PostgreSQL: `localhost:5432`, БД `urichdemo`, пользователь `urich`, пароль `urich`.

Репозитории получают сессию из контейнера (DI). Таблицы создаются при первом запросе (middleware из DatabaseModule).

## Контексты

### Employees (сотрудники)

- **POST /employees/commands/create_employee** — создать сотрудника (`employee_id`, `name`, `role`)
- **GET /employees/queries/get_employee** — получить по `employee_id`
- **GET /employees/queries/list_employees** — список всех (опционально `search` для фильтра по имени/роли)

### Tasks (задачи)

- **POST /tasks/commands/create_task** — создать задачу (`task_id`, `title`, `assignee_id`)
- **POST /tasks/commands/assign_task** — переназначить задачу (`task_id`, `assignee_id`)
- **POST /tasks/commands/complete_task** — закрыть задачу (`task_id`)
- **GET /tasks/queries/get_task** — получить задачу по `task_id`
- **GET /tasks/queries/list_tasks_by_employee** — список задач сотрудника по `employee_id`

## Пример сценария

1. Создать сотрудников: `create_employee` (например `emp-1`, "Иван", "developer") и `emp-2`, "Мария", "designer".
2. Создать задачу: `create_task` (`task-1`, "Сделать API", `emp-1`).
3. Список задач Ивана: `list_tasks_by_employee` с `employee_id=emp-1`.
4. Переназначить задачу Марии: `assign_task` (`task-1`, `emp-2`).
5. Закрыть задачу: `complete_task` (`task-1`).

Данные хранятся в PostgreSQL (при запуске через `docker compose up` или при локальном запуске с указанием `DATABASE_URL`).
