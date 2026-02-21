"""
Точка входа сервиса Tasks (микросервис): только команды/запросы по задачам, проверка JWT.
В монолите используется через services.tasks.module.tasks_module.
"""
from urich import Application

from services.tasks.module import tasks_module
from services.shared.database.module import DatabaseModule
from services.shared.jwt_middleware import JWTValidationMiddleware

app = Application()
app.register(DatabaseModule())
app.starlette.add_middleware(JWTValidationMiddleware)
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
