# Urich Demo — микросервисы (auth, employees, tasks)

Демо на [Urich](https://github.com/KashN9sh/urich): три сервиса, общая БД, обмен через **urich.discovery** и **urich.rpc**.

## Запуск

```bash
docker compose up --build
```

- **Auth:** http://localhost:8001/docs — логин, регистрация, выдача JWT  
- **Employees:** http://localhost:8002/docs — сотрудники  
- **Tasks:** http://localhost:8003/docs — задачи  

Получить токен: **POST /auth/login** на 8001 → скопировать `token` → в Swagger на 8002 и 8003 нажать **Authorize** и вставить токен.

## Локально (три терминала)

```bash
export JWT_SECRET=secret
export DATABASE_URL=postgresql+asyncpg://urich:urich@localhost:5432/urichdemo

uvicorn services.auth.main:app --port 8001 --reload
uvicorn services.employees.main:app --port 8002 --reload
uvicorn services.tasks.main:app --port 8003 --reload
```

PostgreSQL должен быть запущен (например `docker run -p 5432:5432 -e POSTGRES_USER=urich -e POSTGRES_PASSWORD=urich -e POSTGRES_DB=urichdemo postgres:16-alpine`).

## Структура

```
services/
  shared/           # общее
    database/       # БД, создание таблиц
    jwt_middleware.py
  auth/             # логин, регистрация, JWT
    main.py
  employees/        # команды/запросы по сотрудникам, RPC-сервер для Tasks
    main.py
  tasks/            # команды/запросы по задачам, вызов Employees по RPC
    main.py
```

Как сервисы обмениваются данными (RPC, события) — [services/COMMUNICATION.md](services/COMMUNICATION.md).

## API

- **Auth:** POST /auth/register, POST /auth/login  
- **Employees:** POST /employees/commands/create_employee, GET /employees/queries/get_employee, GET /employees/queries/list_employees  
- **Tasks:** POST /tasks/commands/create_task, assign_task, complete_task; GET /tasks/queries/get_task, list_tasks_by_employee  

Tasks при создании/назначении задачи проверяет, что assignee существует, через RPC к Employees (метод `get_employee`).
