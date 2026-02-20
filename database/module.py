"""Database as a module: engine and session factory in container, tables on first request."""
import asyncio
import os
from typing import Any

from sqlalchemy.ext.asyncio import AsyncEngine, AsyncSession, create_async_engine, async_sessionmaker
from starlette.middleware.base import BaseHTTPMiddleware

from urich.core.app import Application
from urich.core.module import Module

from .base import Base

# PostgreSQL async URL
DATABASE_URL = os.environ.get(
    "DATABASE_URL",
    "postgresql+asyncpg://urich:urich@localhost:5432/urichdemo",
)
if DATABASE_URL.startswith("postgresql://") and "+asyncpg" not in DATABASE_URL:
    DATABASE_URL = DATABASE_URL.replace("postgresql://", "postgresql+asyncpg://", 1)


class SessionFactory:
    """Injectable session factory: call it to get an async context manager for AsyncSession."""

    def __init__(self, maker: Any) -> None:
        self._maker = maker

    def __call__(self) -> Any:
        return self._maker()


_tables_created = False
_tables_lock = asyncio.Lock()


class EnsureTablesMiddleware(BaseHTTPMiddleware):
    """Runs create_all once on first request, using engine from app container."""

    def __init__(self, app: Any, app_ref: Application | None = None) -> None:
        super().__init__(app)
        self._app_ref = app_ref

    async def dispatch(self, request: Any, call_next: Any) -> Any:
        global _tables_created
        if not _tables_created and self._app_ref is not None:
            async with _tables_lock:
                if not _tables_created:
                    engine = self._app_ref.container.resolve(AsyncEngine)
                    # ensure models are attached to Base.metadata
                    from employees.models import EmployeeModel  # noqa: F401
                    from tasks.models import TaskModel  # noqa: F401

                    async with engine.begin() as conn:
                        await conn.run_sync(Base.metadata.create_all)
                    _tables_created = True
        return await call_next(request)


class DatabaseModule(Module):
    """Registers engine and session factory in the container; adds middleware to create tables on first request."""

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
        app.starlette.add_middleware(EnsureTablesMiddleware, app_ref=app)
