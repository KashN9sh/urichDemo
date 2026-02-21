"""
Urich demo: employees + tasks. DB and domain as modules.
Run: uvicorn main:app --reload
Docs: http://localhost:8000/docs
"""
from urich import Application

from auth.module import AuthModule
from database.module import DatabaseModule
from employees.module import employees_module
from tasks.module import tasks_module

app = Application()
app.register(DatabaseModule())
app.register(AuthModule())
app.register(employees_module)
app.register(tasks_module)
app.openapi(
    title="Urich Demo — Employees & Tasks",
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
