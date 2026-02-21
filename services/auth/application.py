"""Application layer: команды и хендлеры для логина и регистрации."""
from __future__ import annotations

from dataclasses import dataclass

from urich.ddd import Command

from .infrastructure import AuthService


@dataclass
class Login(Command):
    username: str
    password: str


@dataclass
class Register(Command):
    username: str
    password: str
    role: str = "user"


class LoginHandler:
    def __init__(self, auth_service: AuthService) -> None:
        self._auth = auth_service

    async def __call__(self, cmd: Login) -> tuple[int, dict]:
        user = await self._auth.verify_user(cmd.username, cmd.password)
        if user is None:
            return 401, {"detail": "Invalid username or password"}
        return 200, {"user": user}  # токен добавляется в эндпоинте (нужен JWT_SECRET)


class RegisterHandler:
    def __init__(self, auth_service: AuthService) -> None:
        self._auth = auth_service

    async def __call__(self, cmd: Register) -> tuple[int, dict]:
        username = (cmd.username or "").strip()
        password = cmd.password or ""
        role = (cmd.role or "user").strip()
        if not username or not password:
            return 400, {"detail": "username and password required"}
        existing = await self._auth.get_by_username(username)
        if existing is not None:
            return 400, {"detail": "Username already exists"}
        import uuid
        user_id = str(uuid.uuid4())
        await self._auth.create_user(user_id, username, password, role)
        return 201, {"id": user_id, "username": username, "role": role}
