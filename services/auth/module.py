"""Auth module: регистрация сервиса, команд login/register (новый urich, без Starlette)."""
import os
from typing import Any

import jwt
from urich.core.app import Application
from urich.core.module import Module
from urich.core.request import Request
from urich.core.responses import JSONResponse

from .application import Login, LoginHandler, Register, RegisterHandler
from .infrastructure import AuthService

JWT_SECRET = os.environ.get("JWT_SECRET", "")
JWT_ALGORITHM = "HS256"
JWT_EXPIRES_HOURS = 24


def _create_token(payload: dict) -> str:
    import datetime
    exp = datetime.datetime.utcnow() + datetime.timedelta(hours=JWT_EXPIRES_HOURS)
    payload = {**payload, "exp": exp}
    return jwt.encode(payload, JWT_SECRET, algorithm=JWT_ALGORITHM)


class AuthModule(Module):
    """Регистрирует AuthService, хендлеры Login/Register, маршруты (core-only, без middleware)."""

    def register_into(self, app: Application) -> None:
        app.container.register_class(AuthService)
        app.container.register_class(LoginHandler)
        app.container.register_class(RegisterHandler)
        container = app.container

        async def login_endpoint(req: Request) -> Any:
            if not JWT_SECRET:
                return JSONResponse({"detail": "JWT_SECRET not configured"}, status_code=503)
            try:
                body = await req.json()
            except Exception:
                body = {}
            cmd = Login(**{k: body.get(k, "") for k in ("username", "password")})
            handler = container.resolve(LoginHandler)
            status, data = await handler(cmd)
            if status == 200 and "user" in data:
                data = {**data, "token": _create_token({
                    "sub": data["user"]["id"],
                    "username": data["user"]["username"],
                    "role": data["user"]["role"],
                })}
            return JSONResponse(data, status_code=status)

        async def register_endpoint(req: Request) -> Any:
            if not JWT_SECRET:
                return JSONResponse({"detail": "JWT_SECRET not configured"}, status_code=503)
            try:
                body = await req.json()
            except Exception:
                body = {}
            cmd = Register(
                username=body.get("username", ""),
                password=body.get("password", ""),
                role=body.get("role", "user"),
            )
            handler = container.resolve(RegisterHandler)
            status, data = await handler(cmd)
            return JSONResponse(data, status_code=status)

        login_body = {
            "type": "object",
            "required": ["username", "password"],
            "properties": {"username": {"type": "string"}, "password": {"type": "string"}},
        }
        register_body = {
            "type": "object",
            "required": ["username", "password"],
            "properties": {
                "username": {"type": "string"},
                "password": {"type": "string"},
                "role": {"type": "string", "default": "user"},
            },
        }
        app.add_route("/auth/login", login_endpoint, methods=["POST"], openapi_body_schema=login_body, openapi_tags=["Auth"])
        app.add_route("/auth/register", register_endpoint, methods=["POST"], openapi_body_schema=register_body, openapi_tags=["Auth"])
