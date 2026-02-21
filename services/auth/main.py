"""
Точка входа сервиса Auth (микросервис): только логин, регистрация, выдача JWT.
В монолите используется через services.auth.module.AuthModule.
"""
from urich import Application

from services.auth.module import AuthModule
from services.shared.database.module import DatabaseModule

app = Application()
app.register(DatabaseModule())
app.register(AuthModule())
app.openapi(
    title="Auth Service",
    version="0.1.0",
    security_schemes={
        "BearerAuth": {
            "type": "http",
            "scheme": "bearer",
            "bearerFormat": "JWT",
            "description": "JWT из POST /auth/login",
        },
    },
    global_security=[{"BearerAuth": []}],
)
