# Demo на Rust (urich-rs)

**Структура как в services/:** shared (в т.ч. database, jwt), auth, employees, tasks; три сервиса на разных портах; **БД SQLite** (таблицы users, employees, tasks).

## Структура

```
src/
  shared/           # как services/shared
    database        # SQLite, создание таблиц при старте, Db
    jwt_middleware  # JWT (Bearer) или X-Demo-Key
    http_transport  # HTTP-транспорт для RPC (Tasks → Employees)
  auth/             # как services/auth — логин, регистрация (БД)
  employees/        # как services/employees — команды/запросы в БД + RPC get_employee
  tasks/            # как services/tasks — БД + RPC к Employees для assignee
  bin/
    auth.rs         # сервис Auth (порт 8001)
    employees.rs    # сервис Employees (порт 8002)
    tasks.rs        # сервис Tasks (порт 8003)
```

## БД

- **SQLite:** по умолчанию файл `urich_demo.db` в текущей директории.
- Переменная **`DATABASE_URL`**: путь к файлу (например `auth.db`, `employees.db`) или `:memory:` для in-memory.
- Таблицы **users**, **employees**, **tasks** создаются при первом запуске каждого бинарника.
- Для «одной БД на все сервисы» задайте один и тот же путь в `DATABASE_URL` при запуске auth, employees и tasks.

## Запуск (три терминала)

Репозиторий urich рядом с urichDemo (path в Cargo.toml: `../../urich/urich-rs`).

```bash
cd rust

# Общая БД для всех (опционально)
export DATABASE_URL=urich_demo.db

# Сервис Auth
cargo run --bin auth
# → http://127.0.0.1:8001  POST /auth/login  POST /auth/register

# Сервис Employees (в другом терминале)
cargo run --bin employees
# → http://127.0.0.1:8002  commands/queries + RPC /rpc  method get_employee

# Сервис Tasks (в третьем терминале; вызывает Employees по RPC)
EMPLOYEES_SERVICE_URL=http://127.0.0.1:8002 cargo run --bin tasks
# → http://127.0.0.1:8003  commands/queries, при create_task/assign_task — проверка assignee по RPC
```

- **Auth:** пользователи в БД (users), токен в ответе login (демо: `demo-token-<id>`).
- **Employees:** JWT middleware, DomainModule + RpcModule.get_employee, данные в БД (employees).
- **Tasks:** JWT, данные в БД (tasks), проверка assignee через RPC к employees.

По смыслу то же, что и Python services (auth, employees, tasks) с общей PostgreSQL, здесь — SQLite и один файл или отдельные файлы на сервис.

**DI как в Python:** зависимости регистрируются в контейнере (`app.with_container_mut(|c| c.register_instance(db))`), обработчики получают `(body, &Container)` и делают `container.resolve::<Db>()` и т.д. Как в services (Python): `app.container` и резолв при вызове handler’а.
