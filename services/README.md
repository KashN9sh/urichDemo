# Сервисы (auth, employees, tasks)

Вся доменная логика разнесена по каталогам так же, как при запуске микросервисов.

## Структура

```
services/
  shared/           # общее
    database/      # БД, SessionFactory, создание таблиц
    jwt_middleware.py   # проверка JWT для employees/tasks
  auth/            # логин, регистрация, выдача JWT
    main.py        # точка входа микросервиса Auth
    module.py      # AuthModule для монолита
    ...
  employees/
    main.py
    module.py
    ...
  tasks/
    main.py
    module.py
    ...
```

## Монолит (одно приложение)

Из корня репозитория:

```bash
uvicorn main:app --reload
```

`main.py` подключает DatabaseModule, AuthModule, employees_module, tasks_module из `services.*`.

## Микросервисы (три процесса)

Из корня репозитория:

```bash
docker compose -f services/docker-compose.yml up --build
```

- Auth: http://localhost:8001/docs  
- Employees: http://localhost:8002/docs  
- Tasks: http://localhost:8003/docs  

Токен получают на 8001, подставляют в Authorize на 8002 и 8003.

Локально без Docker (три терминала):

```bash
export JWT_SECRET=secret
export DATABASE_URL=postgresql+asyncpg://urich:urich@localhost:5432/urichdemo

uvicorn services.auth.main:app --port 8001 --reload
uvicorn services.employees.main:app --port 8002 --reload
uvicorn services.tasks.main:app --port 8003 --reload
```
