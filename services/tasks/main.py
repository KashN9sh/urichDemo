"""Точка входа сервиса Tasks: команды/запросы, JWT, вызов Employees по RPC (новый urich + middleware)."""
from urich import Application, Config
from urich.discovery import DiscoveryModule, static_discovery
from urich.rpc import JsonHttpRpcTransport, RpcModule

from services.shared.database.module import DatabaseModule
from services.shared.jwt_middleware import jwt_validation_middleware
from services.tasks.infrastructure import RpcEmployeeService
from services.tasks.module import tasks_module
from services.tasks.ports import IEmployeeService

# Discovery из env: EMPLOYEES_SERVICE_URL -> "employees"; RpcClient регистрируется автоматически
discovery = static_discovery(Config.services_from_env())
transport = JsonHttpRpcTransport(discovery, base_path="/rpc")

app = Application()
app.register(DatabaseModule())
app.add_middleware(jwt_validation_middleware(public_path_prefixes=("/docs", "/openapi.json")))
app.register(DiscoveryModule().adapter(discovery))
app.register(RpcModule().client(discovery=discovery, transport=transport))
app.container.register_class(RpcEmployeeService)
app.container.register(IEmployeeService, lambda: app.container.resolve(RpcEmployeeService))
app.register(tasks_module)
app.openapi(
    title="Tasks Service",
    version="0.1.0",
    security_schemes={
        "BearerAuth": {
            "type": "http",
            "scheme": "bearer",
            "bearerFormat": "JWT",
            "description": "JWT из Auth Service (POST /auth/login)",
        },
    },
    global_security=[{"BearerAuth": []}],
)

if __name__ == "__main__":
    app.run()
