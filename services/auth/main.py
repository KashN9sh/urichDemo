"""Точка входа сервиса Auth: логин, регистрация, выдача JWT."""
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

if __name__ == "__main__":
    # Наш ядерный ASGI (как в Rust): host/port из env HOST, PORT или --host, --port
    app.run()
