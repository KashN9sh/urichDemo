"""Database as a module: engine and session factory in container, tables at startup (new urich, no Starlette)."""
import asyncio
import os
from typing import Any

from sqlalchemy.ext.asyncio import AsyncEngine, AsyncSession, create_async_engine, async_sessionmaker

from urich.core.app import Application
from urich.core.module import Module

from .base import Base

DATABASE_URL = os.environ.get(
    "DATABASE_URL",
    "postgresql+asyncpg://urich:urich@localhost:5432/urichdemo",
)
if DATABASE_URL.startswith("postgresql://") and "+asyncpg" not in DATABASE_URL:
    DATABASE_URL = DATABASE_URL.replace("postgresql://", "postgresql+asyncpg://", 1)


class SessionFactory:
    def __init__(self, maker: Any) -> None:
        self._maker = maker

    def __call__(self) -> Any:
        return self._maker()


async def _create_tables(engine: AsyncEngine) -> None:
    from services.auth.models import UserModel  # noqa: F401
    from services.employees.models import EmployeeModel  # noqa: F401
    from services.tasks.models import TaskModel  # noqa: F401

    async with engine.begin() as conn:
        await conn.run_sync(Base.metadata.create_all)


class DatabaseModule(Module):
    def register_into(self, app: Application) -> None:
        engine = create_async_engine(
            DATABASE_URL,
            echo=os.environ.get("SQL_ECHO", "").lower() in ("1", "true"),
        )
        session_factory = async_sessionmaker(
            engine, class_=AsyncSession, expire_on_commit=False
        )
        app.container.register_instance(AsyncEngine, engine)
        app.container.register_instance(SessionFactory, SessionFactory(session_factory))
        asyncio.run(_create_tables(engine))
