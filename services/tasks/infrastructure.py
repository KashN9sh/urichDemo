"""Infrastructure: task repository и RPC-клиент к Employees (urich.discovery + urich.rpc)."""
from __future__ import annotations

import json
import logging
from typing import Optional

logger = logging.getLogger(__name__)

from sqlalchemy import select

from urich.discovery.protocol import ServiceDiscovery
from urich.domain import Repository
from urich.rpc.protocol import RpcTransport

from services.shared.database.module import SessionFactory

from .domain import Task
from .models import TaskModel


class ITaskRepository(Repository[Task]):
    pass


class TaskRepositoryImpl(ITaskRepository):
    def __init__(self, session_factory: SessionFactory) -> None:
        self._session_factory = session_factory

    async def get(self, id: str) -> Optional[Task]:
        async with self._session_factory() as session:
            result = await session.execute(select(TaskModel).where(TaskModel.id == id))
            row = result.scalar_one_or_none()
            if row is None:
                return None
            return Task.from_db(row.id, row.title, row.assignee_id, row.status)

    async def add(self, aggregate: Task) -> None:
        async with self._session_factory() as session:
            session.add(
                TaskModel(
                    id=aggregate.id,
                    title=aggregate.title,
                    assignee_id=aggregate.assignee_id,
                    status=aggregate.status,
                )
            )
            await session.commit()

    async def save(self, aggregate: Task) -> None:
        async with self._session_factory() as session:
            result = await session.execute(select(TaskModel).where(TaskModel.id == aggregate.id))
            row = result.scalar_one_or_none()
            if row:
                row.title = aggregate.title
                row.assignee_id = aggregate.assignee_id
                row.status = aggregate.status
            else:
                session.add(
                    TaskModel(
                        id=aggregate.id,
                        title=aggregate.title,
                        assignee_id=aggregate.assignee_id,
                        status=aggregate.status,
                    )
                )
            await session.commit()

    async def list_by_assignee(self, assignee_id: str) -> list[Task]:
        async with self._session_factory() as session:
            result = await session.execute(
                select(TaskModel).where(TaskModel.assignee_id == assignee_id)
            )
            rows = result.scalars().all()
            return [self._row_to_task(r) for r in rows]

    def _row_to_task(self, row: TaskModel) -> Task:
        return Task.from_db(row.id, row.title, row.assignee_id, row.status)


class RpcEmployeeService:
    """Вызов Employees через urich RPC (Discovery + RpcTransport)."""

    def __init__(self, discovery: ServiceDiscovery, transport: RpcTransport) -> None:
        self._discovery = discovery
        self._transport = transport

    async def get_employee(self, employee_id: str) -> dict | None:
        urls = self._discovery.resolve("employees")
        if not urls:
            logger.warning("RPC employees: discovery returned no URLs")
            return None
        try:
            payload = json.dumps({"employee_id": employee_id}).encode()
            result = await self._transport.call(urls[0], "get_employee", payload)
            data = json.loads(result.decode()) if result else None
            if isinstance(data, dict) and data.get("id"):
                return data
            logger.warning("RPC get_employee(%r): unexpected response %s", employee_id, (result.decode(errors="replace")[:200] if result else None))
            return None
        except Exception as e:
            logger.warning("RPC get_employee(%r) failed: %s", employee_id, e, exc_info=True)
            return None
