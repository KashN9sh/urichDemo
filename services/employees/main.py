"""
Точка входа сервиса Employees (микросервис): только команды/запросы по сотрудникам, проверка JWT.
В монолите используется через services.employees.module.employees_module.
"""
from urich import Application

from services.employees.module import employees_module
from services.shared.database.module import DatabaseModule
from services.shared.jwt_middleware import JWTValidationMiddleware

app = Application()
app.register(DatabaseModule())
app.starlette.add_middleware(JWTValidationMiddleware)
app.register(employees_module)
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
