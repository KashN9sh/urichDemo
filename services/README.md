# Сервисы

Три приложения: **auth**, **employees**, **tasks**. Каждый — отдельный процесс, точка входа `services/<name>/main.py`.

## Запуск

**Docker (из корня репозитория):**
```bash
docker compose up --build
```
Файл `docker-compose.yml` в корне репозитория.

**Локально (три терминала, из корня):**
```bash
export JWT_SECRET=secret
export DATABASE_URL=postgresql+asyncpg://urich:urich@localhost:5432/urichdemo

uvicorn services.auth.main:app --port 8001 --reload
uvicorn services.employees.main:app --port 8002 --reload
uvicorn services.tasks.main:app --port 8003 --reload
```

- Auth: http://localhost:8001/docs  
- Employees: http://localhost:8002/docs  
- Tasks: http://localhost:8003/docs  

Токен с 8001 подставлять в Authorize на 8002 и 8003.

## Обмен между сервисами

[RPC (Tasks → Employees), события, JWT](COMMUNICATION.md).
