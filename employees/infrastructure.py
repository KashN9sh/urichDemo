"""Infrastructure: employee repository (SQLAlchemy, session from DI)."""
from typing import Optional
from sqlalchemy import select

from urich.domain import Repository

from database.module import SessionFactory
from .domain import Employee
from .models import EmployeeModel


class IEmployeeRepository(Repository[Employee]):
    pass


class EmployeeRepositoryImpl(IEmployeeRepository):
    def __init__(self, session_factory: SessionFactory) -> None:
        self._session_factory = session_factory

    async def get(self, id: str) -> Optional[Employee]:
        async with self._session_factory() as session:
            result = await session.execute(select(EmployeeModel).where(EmployeeModel.id == id))
            row = result.scalar_one_or_none()
            if row is None:
                return None
            return Employee.from_db(row.id, row.name, row.role)

    async def add(self, aggregate: Employee) -> None:
        async with self._session_factory() as session:
            session.add(
                EmployeeModel(id=aggregate.id, name=aggregate.name, role=aggregate.role)
            )
            await session.commit()

    async def save(self, aggregate: Employee) -> None:
        async with self._session_factory() as session:
            result = await session.execute(select(EmployeeModel).where(EmployeeModel.id == aggregate.id))
            row = result.scalar_one_or_none()
            if row:
                row.name = aggregate.name
                row.role = aggregate.role
            else:
                session.add(
                    EmployeeModel(id=aggregate.id, name=aggregate.name, role=aggregate.role)
                )
            await session.commit()

    async def list_all(self) -> list[Employee]:
        async with self._session_factory() as session:
            result = await session.execute(select(EmployeeModel))
            rows = result.scalars().all()
            return [Employee.from_db(r.id, r.name, r.role) for r in rows]
