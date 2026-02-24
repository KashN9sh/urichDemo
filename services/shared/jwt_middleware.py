"""JWT middleware для нового urich (core): проверяет Bearer, выставляет request.state["user"] или возвращает 401."""
import os
from typing import Any

import jwt
from urich.core.request import Request
from urich.core.responses import JSONResponse

JWT_ALGORITHM = "HS256"


def decode_token(secret: str, token: str) -> dict | None:
    try:
        return jwt.decode(token, secret, algorithms=[JWT_ALGORITHM])
    except jwt.InvalidTokenError:
        return None


def jwt_validation_middleware(
    public_path_prefixes: tuple[str, ...] = ("docs", "openapi.json", "rpc"),  # path from core has no leading /
) -> Any:
    """Фабрика: возвращает middleware (request) -> None | Response для app.add_middleware()."""

    secret = os.environ.get("JWT_SECRET", "")

    def middleware(request: Request) -> Any:
        request.state["user"] = {"id": "anonymous", "username": "anonymous", "role": "user"}

        if not secret:
            return None

        path = getattr(request, "path", "") or ""
        if any(path.startswith(p) for p in public_path_prefixes):
            return None

        auth = request.headers.get("authorization") or request.headers.get("Authorization") or ""
        token = auth.strip()
        while token and token.lower().startswith("bearer "):
            token = token[7:].strip()
        if not token:
            return JSONResponse(
                {"detail": "Missing Authorization: Bearer <token>"},
                status_code=401,
            )
        payload = decode_token(secret, token)
        if not payload:
            return JSONResponse(
                {"detail": "Invalid or expired token"},
                status_code=401,
            )
        request.state["user"] = {
            "id": payload.get("sub", ""),
            "username": payload.get("username", ""),
            "role": payload.get("role", "user"),
        }
        return None

    return middleware
