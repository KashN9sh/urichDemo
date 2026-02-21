# Взаимодействие сервисов

Три способа: **общий JWT**, **RPC** (синхронный вызов другого сервиса), **события** (асинхронная шина).

---

## 1. JWT и БД

- **Auth** (порт 8001) — единственный сервис с логином и регистрацией. Выдаёт JWT. Пользователи в таблице `users`.
- **Employees** и **Tasks** не вызывают Auth: проверяют подпись токена по общему **JWT_SECRET** из env.
- Все три сервиса подключаются к одной **PostgreSQL**; таблицы создаются при первом запросе (shared database module).

---

## 2. RPC: Tasks → Employees (urich.discovery + urich.rpc)

Tasks при создании/назначении задачи проверяет, что `assignee_id` — существующий сотрудник. Для этого вызывает сервис Employees по RPC.

- **urich.discovery** — по имени сервиса возвращает URL: `DiscoveryModule`, `ServiceDiscovery`, `static_discovery`.
- **urich.rpc** — сервер принимает вызовы по пути `/rpc/{method}`, клиент шлёт запрос через `RpcTransport` (например `JsonHttpRpcTransport`).

**Как сделано в репозитории:**

1. **Employees** поднимает RPC-сервер: в `services/employees/main.py` — `RpcModule().server(path="/rpc", handler=EmployeesRpcHandler)`. Обработчик в `services/employees/rpc_handler.py`: метод `get_employee`, параметр `employee_id` → из репозитория возвращает `{id, name, role}` или `null`.

2. **Tasks** поднимает RPC-клиент: в `services/tasks/main.py` — `DiscoveryModule().adapter(static_discovery({"employees": url}))`, `RpcModule().client(discovery=..., transport=JsonHttpRpcTransport(discovery, "/rpc"))`. Реализация `IEmployeeService` — `RpcEmployeeService` в `services/tasks/infrastructure.py`: резолвит `"employees"` через `ServiceDiscovery`, вызывает `RpcTransport.call(url, "get_employee", payload)`.

3. В **docker-compose** у сервиса tasks задаётся `EMPLOYEES_SERVICE_URL=http://employees:8000`; при локальном запуске по умолчанию `http://localhost:8002`.

---

## 3. События (urich.events)

Для асинхронного уведомления других сервисов (например «Сотрудник создан», «Задача назначена») в urich есть:

- **EventBusModule**, **EventBusAdapter** — шина: `publish(event)`, `subscribe(event_type, handler)`. Можно `.in_memory()` или свой адаптер (при `publish` дополнительно слать в Redis/RabbitMQ/Kafka).
- **OutboxModule**, **OutboxStorage**, **OutboxPublisher** — транзакционный outbox: запись событий в одну транзакцию с сохранением агрегата, отдельный воркер забирает и публикует в очередь.

В этом демо события между сервисами по очереди не настроены; при необходимости поднимается адаптер EventBus, при `publish` — отправка в очередь, в других сервисах — подписка на очередь и вызов use case.

---

## Кратко

| Задача | Решение в демо |
|--------|-----------------|
| Проверка токена | Общий JWT_SECRET, без вызова Auth. |
| Tasks проверяет assignee | RPC к Employees (Discovery + RpcTransport), метод `get_employee`. |
| Уведомить другие сервисы о факте | urich.events (EventBusModule, OutboxModule); в демо не подключено. |
