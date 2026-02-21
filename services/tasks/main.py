"""Точка входа сервиса Tasks: команды/запросы по задачам, JWT, вызов Employees по RPC."""
import os

from urich import Application
from urich.discovery import DiscoveryModule, static_discovery
from urich.rpc import JsonHttpRpcTransport, RpcModule

from services.shared.database.module import DatabaseModule
from services.shared.jwt_middleware import JWTValidationMiddleware
from services.tasks.infrastructure import RpcEmployeeService
from services.tasks.module import tasks_module
from services.tasks.ports import IEmployeeService

employees_url = os.environ.get("EMPLOYEES_SERVICE_URL", "http://localhost:8002")
discovery = static_discovery({"employees": employees_url})
transport = JsonHttpRpcTransport(discovery, base_path="/rpc")

app = Application()
app.register(DatabaseModule())
app.starlette.add_middleware(JWTValidationMiddleware)
# Tasks вызывает Employees через Discovery + RPC (urich.discovery, urich.rpc)
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
