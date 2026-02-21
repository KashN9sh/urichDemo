"""Точка входа сервиса Employees: команды/запросы по сотрудникам, JWT, RPC для Tasks."""
from urich import Application
from urich.rpc import RpcModule

from services.employees.module import employees_module
from services.employees.rpc_handler import EmployeesRpcHandler
from services.shared.database.module import DatabaseModule
from services.shared.jwt_middleware import JWTValidationMiddleware

app = Application()
app.register(DatabaseModule())
app.starlette.add_middleware(JWTValidationMiddleware)
app.register(employees_module)
# RPC для вызовов из Tasks (get_employee)
app.container.register_class(EmployeesRpcHandler)
rpc_handler = app.container.resolve(EmployeesRpcHandler)
app.register(RpcModule().server(path="/rpc", handler=rpc_handler))
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
