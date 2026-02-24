"""Точка входа сервиса Employees: команды/запросы, JWT, RPC для Tasks (новый urich + middleware)."""
from urich import Application
from urich.rpc import RpcModule

from services.employees.module import employees_module
from services.employees.rpc_handler import EmployeesRpcHandler
from services.shared.database.module import DatabaseModule
from services.shared.jwt_middleware import jwt_validation_middleware

app = Application()
app.register(DatabaseModule())
app.add_middleware(jwt_validation_middleware(public_path_prefixes=("docs", "openapi.json", "rpc")))
app.register(employees_module)
# RPC для вызовов из Tasks: handler регистрируется и резолвится из контейнера
app.register(RpcModule().server(path="/rpc", handler=EmployeesRpcHandler))
app.openapi(
    title="Employees Service",
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
