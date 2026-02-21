"""Auth module: users in DB, login/register, JWT Bearer validation."""
import os
import uuid
from typing import Any

import jwt
from starlette.middleware.base import BaseHTTPMiddleware
from starlette.requests import Request
from starlette.responses import JSONResponse

from urich.core.app import Application
from urich.core.module import Module

from .service import AuthService

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
    """Validates Bearer JWT; sets request.state.user from token payload or returns 401."""

    async def dispatch(self, request: Request, call_next: Any) -> Any:
        request.state.user = {"id": "anonymous", "username": "anonymous", "role": "user"}

        if not JWT_SECRET:
            return await call_next(request)

        path = request.scope.get("path", "")
        if any(path.startswith(p) for p in PUBLIC_PREFIXES):
            return await call_next(request)

        auth = request.headers.get("Authorization")
        token = None
        if auth and auth.startswith("Bearer "):
            token = auth[7:].strip()
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
    """Registers AuthService, POST /auth/login and /auth/register, and JWT auth middleware."""

    def register_into(self, app: Application) -> None:
        app.container.register_class(AuthService)
        container = app.container

        async def login(req: Request) -> Any:
            if not JWT_SECRET:
                return JSONResponse(status_code=503, content={"detail": "JWT_SECRET not configured"})
            try:
                body = await req.json()
            except Exception:
                body = {}
            username = body.get("username") or ""
            password = body.get("password") or ""
            if not username or not password:
                return JSONResponse(status_code=400, content={"detail": "username and password required"})
            auth_service = container.resolve(AuthService)
            user = await auth_service.verify_user(username, password)
            if not user:
                return JSONResponse(status_code=401, content={"detail": "Invalid username or password"})
            token = _create_token({"sub": user["id"], "username": user["username"], "role": user["role"]})
            return JSONResponse({"token": token, "user": user})

        async def register(req: Request) -> Any:
            if not JWT_SECRET:
                return JSONResponse(status_code=503, content={"detail": "JWT_SECRET not configured"})
            try:
                body = await req.json()
            except Exception:
                body = {}
            username = (body.get("username") or "").strip()
            password = body.get("password") or ""
            role = (body.get("role") or "user").strip()
            if not username or not password:
                return JSONResponse(status_code=400, content={"detail": "username and password required"})
            auth_service = container.resolve(AuthService)
            existing = await auth_service.get_by_username(username)
            if existing:
                return JSONResponse(status_code=400, content={"detail": "Username already exists"})
            user_id = str(uuid.uuid4())
            await auth_service.create_user(user_id, username, password, role)
            return JSONResponse(status_code=201, content={"id": user_id, "username": username, "role": role})

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
        app.add_route(
            "/auth/login",
            login,
            methods=["POST"],
            openapi_tags=["Auth"],
            openapi_body_schema=login_body,
            openapi_security=[],  # публичный эндпоинт
        )
        app.add_route(
            "/auth/register",
            register,
            methods=["POST"],
            openapi_tags=["Auth"],
            openapi_body_schema=register_body,
            openapi_security=[],  # публичный эндпоинт
        )
        app.starlette.add_middleware(AuthMiddleware)
