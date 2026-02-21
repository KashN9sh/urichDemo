"""Auth service: find user by username, verify password, create user (uses DB via SessionFactory)."""
from sqlalchemy import select

from database.module import SessionFactory
from .models import UserModel
from .password import hash_password, verify_password


class AuthService:
    def __init__(self, session_factory: SessionFactory) -> None:
        self._session_factory = session_factory

    async def get_by_username(self, username: str) -> UserModel | None:
        async with self._session_factory() as session:
            result = await session.execute(
                select(UserModel).where(UserModel.username == username)
            )
            return result.scalar_one_or_none()

    async def verify_user(self, username: str, password: str) -> dict | None:
        user = await self.get_by_username(username)
        if user is None:
            return None
        if not verify_password(password, user.password_hash):
            return None
        return {"id": user.id, "username": user.username, "role": user.role}

    async def create_user(self, user_id: str, username: str, password: str, role: str = "user") -> None:
        async with self._session_factory() as session:
            session.add(
                UserModel(
                    id=user_id,
                    username=username,
                    password_hash=hash_password(password),
                    role=role,
                )
            )
            await session.commit()
