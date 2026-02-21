# Urich Demo — Сотрудники и задачи

Демо на [Urich](https://github.com/KashN9sh/urich): приложение собирается из модулей. Код разнесён по **services/** (auth, employees, tasks, shared); тот же layout используется и для монолита, и для запуска микросервисов.

- **Монолит**: один процесс, `uvicorn main:app` или `docker compose up`.
- **Микросервисы**: три процесса (auth :8001, employees :8002, tasks :8003), см. [services/README.md](services/README.md).

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

### Авторизация

Данные для авторизации хранятся в БД (таблица `users`: логин, хэш пароля, роль). Модуль **AuthModule**:

- **POST /auth/register** — регистрация: тело `{"username": "...", "password": "...", "role": "user"}`. Возвращает `201` и `id` пользователя.
- **POST /auth/login** — вход: тело `{"username": "...", "password": "..."}`. При успехе возвращает `{"token": "<jwt>", "user": {...}}`.

Если задана переменная окружения **JWT_SECRET**, все запросы к API (кроме `/docs`, `/openapi.json`, `/auth/login`, `/auth/register`) должны содержать заголовок **Authorization: Bearer &lt;jwt&gt;**. Без токена или с невалидным токеном возвращается 401. Если `JWT_SECRET` не задан, проверка отключена (удобно для локальной разработки). В Swagger: кнопка **Authorize**, ввести полученный при логине JWT в поле Bearer.

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
