"""Общая проверка JWT для сервисов employees и tasks (токен выдаёт сервис auth)."""
import os
from typing import Any

import jwt
from starlette.middleware.base import BaseHTTPMiddleware
from starlette.requests import Request
from starlette.responses import JSONResponse

JWT_ALGORITHM = "HS256"


def decode_token(secret: str, token: str) -> dict | None:
    try:
        return jwt.decode(token, secret, algorithms=[JWT_ALGORITHM])
    except jwt.InvalidTokenError:
        return None


class JWTValidationMiddleware(BaseHTTPMiddleware):
    """Проверяет Bearer JWT (выданный сервисом auth); выставляет request.state.user или 401."""

    def __init__(self, app: Any, *, public_path_prefixes: tuple[str, ...] = ("/docs", "/openapi.json")) -> None:
        super().__init__(app)
        self._secret = os.environ.get("JWT_SECRET", "")
        self._public = public_path_prefixes

    async def dispatch(self, request: Request, call_next: Any) -> Any:
        request.state.user = {"id": "anonymous", "username": "anonymous", "role": "user"}

        if not self._secret:
            return await call_next(request)

        path = request.scope.get("path", "")
        if any(path.startswith(p) for p in self._public):
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
        payload = decode_token(self._secret, token)
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
