"""Auth module: регистрация сервиса, команд (login/register), JWT middleware."""
import inspect
import os
from typing import Any

import jwt
from starlette.middleware.base import BaseHTTPMiddleware
from starlette.requests import Request
from starlette.responses import JSONResponse

from urich.core.app import Application
from urich.core.module import Module

from .application import Login, LoginHandler, Register, RegisterHandler
from .infrastructure import AuthService

JWT_SECRET = os.environ.get("JWT_SECRET", "")
JWT_ALGORITHM = "HS256"
JWT_EXPIRES_HOURS = 24
PUBLIC_PREFIXES = ("/docs", "/openapi.json", "/auth/login", "/auth/register")


def _create_token(payload: dict) -> str:
    import datetime
    exp = datetime.datetime.utcnow() + datetime.timedelta(hours=JWT_EXPIRES_HOURS)
    payload = {**payload, "exp": exp}
    return jwt.encode(payload, JWT_SECRET, algorithm=JWT_ALGORITHM)


def _decode_token(token: str) -> dict | None:
    try:
        return jwt.decode(token, JWT_SECRET, algorithms=[JWT_ALGORITHM])
    except jwt.InvalidTokenError:
        return None


class AuthMiddleware(BaseHTTPMiddleware):
    """Проверяет Bearer JWT; выставляет request.state.user или возвращает 401."""

    async def dispatch(self, request: Request, call_next: Any) -> Any:
        request.state.user = {"id": "anonymous", "username": "anonymous", "role": "user"}

        if not JWT_SECRET:
            return await call_next(request)

        path = request.scope.get("path", "")
        if any(path.startswith(p) for p in PUBLIC_PREFIXES):
            return await call_next(request)

        auth = request.headers.get("Authorization") or ""
        token = auth.strip()
        while token and token.lower().startswith("bearer "):
            token = token[7:].strip()
        if not token:
            return JSONResponse(
                status_code=401,
                content={"detail": "Missing Authorization: Bearer <token>"},
            )
        payload = _decode_token(token)
        if not payload:
            return JSONResponse(
                status_code=401,
                content={"detail": "Invalid or expired token"},
            )
        request.state.user = {
            "id": payload.get("sub", ""),
            "username": payload.get("username", ""),
            "role": payload.get("role", "user"),
        }
        return await call_next(request)


class AuthModule(Module):
    """Регистрирует AuthService, хендлеры Login/Register, маршруты и JWT middleware."""

    def register_into(self, app: Application) -> None:
        app.container.register_class(AuthService)
        app.container.register_class(LoginHandler)
        app.container.register_class(RegisterHandler)
        container = app.container

        async def login_endpoint(req: Request) -> Any:
            if not JWT_SECRET:
                return JSONResponse(status_code=503, content={"detail": "JWT_SECRET not configured"})
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
            return JSONResponse(status_code=status, content=data)

        async def register_endpoint(req: Request) -> Any:
            if not JWT_SECRET:
                return JSONResponse(status_code=503, content={"detail": "JWT_SECRET not configured"})
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
            return JSONResponse(status_code=status, content=data)

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
        route_kw: dict[str, Any] = {"openapi_tags": ["Auth"]}
        if "openapi_security" in inspect.signature(app.add_route).parameters:
            route_kw["openapi_security"] = []

        app.add_route("/auth/login", login_endpoint, methods=["POST"], openapi_body_schema=login_body, **route_kw)
        app.add_route("/auth/register", register_endpoint, methods=["POST"], openapi_body_schema=register_body, **route_kw)
        app.starlette.add_middleware(AuthMiddleware)
