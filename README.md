# Urich Demo — микросервисы (auth, employees, tasks)

Демо на [Urich](https://github.com/KashN9sh/urich): три сервиса, общая БД, обмен через **urich.discovery** и **urich.rpc**.

## Запуск

```bash
docker compose up --build
```

- **Приложение (фронт + API):** http://localhost:8080 — вход, регистрация, сотрудники, задачи. Единая точка входа через Nginx (прокси к сервисам).
- **Auth:** http://localhost:8001/docs — логин, регистрация, выдача JWT  
- **Employees:** http://localhost:8002/docs — сотрудники  
- **Tasks:** http://localhost:8003/docs — задачи  

В браузере откройте http://localhost:8080 — зарегистрируйтесь или войдите, после этого доступны разделы «Сотрудники» и «Задачи».

## Локально (три терминала)

```bash
export JWT_SECRET=secret
export DATABASE_URL=postgresql+asyncpg://urich:urich@localhost:5432/urichdemo

uvicorn services.auth.main:app --port 8001 --reload
uvicorn services.employees.main:app --port 8002 --reload
uvicorn services.tasks.main:app --port 8003 --reload
```

PostgreSQL должен быть запущен (например `docker run -p 5432:5432 -e POSTGRES_USER=urich -e POSTGRES_PASSWORD=urich -e POSTGRES_DB=urichdemo postgres:16-alpine`).

## Разработка urich (без пуша в PyPI)

Если правишь сам фреймворк urich и хочешь тестировать в демо:

- **Docker:** urich уже подмонтирован из `../urich` и ставится как `pip install -e /urich` при старте контейнеров. Держи репозитории рядом: `…/urich` и `…/urichDemo`.
- **Локально (venv):** перед запуском сервисов выполни `pip install -e ../urich` — дальше демо будет использовать локальный urich.

## Структура

```
frontend/           # SPA (Vite + React + TypeScript), сборка в Docker, раздача через Nginx в сервисе web
python/             # Минимальный демо на urich (Python): один контекст tasks, команды/запросы
rust/               # Минимальный демо на urich-rs (Rust): тот же контекст tasks
services/           # Полный демо на новом urich (core-only, без Starlette)
  shared/
    database/       # БД, создание таблиц при старте (без middleware)
  auth/             # логин, регистрация, выдача JWT (маршруты через add_route)
  employees/        # команды/запросы, RPC-сервер для Tasks
  tasks/            # команды/запросы, RPC-клиент к Employees
```

Как сервисы обмениваются данными (RPC, события) — [services/COMMUNICATION.md](services/COMMUNICATION.md).

## API

- **Auth:** POST /auth/register, POST /auth/login  
- **Employees:** POST /employees/commands/create_employee, GET /employees/queries/get_employee, GET /employees/queries/list_employees  
- **Tasks:** POST /tasks/commands/create_task, assign_task, complete_task; GET /tasks/queries/get_task, list_tasks_by_employee  

Tasks при создании/назначении задачи проверяет, что assignee существует, через RPC к Employees (метод `get_employee`).
