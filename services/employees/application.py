"""Application layer: commands, queries, handlers."""
from __future__ import annotations

from dataclasses import dataclass
from urich.ddd import Command, Query
from urich.domain import EventBus

from .domain import Employee, EmployeeCreated
from .infrastructure import IEmployeeRepository


@dataclass
class CreateEmployee(Command):
    employee_id: str
    name: str
    role: str


@dataclass
class GetEmployee(Query):
    employee_id: str


@dataclass
class ListEmployees(Query):
    search: str = ""


async def create_employee(
    cmd: CreateEmployee,
    employee_repository: IEmployeeRepository,
    event_bus: EventBus,
) -> str:
    """Хендлер-функция: зависимости инжектятся по типу из контейнера (Urich DI)."""
    employee = Employee(id=cmd.employee_id, name=cmd.name, role=cmd.role)
    await employee_repository.add(employee)
    await event_bus.publish(
        EmployeeCreated(employee_id=employee.id, name=employee.name, role=employee.role)
    )
    return employee.id


async def get_employee(
    query: GetEmployee,
    employee_repository: IEmployeeRepository,
):
    employee = await employee_repository.get(query.employee_id)
    if employee is None:
        return None
    return {"id": employee.id, "name": employee.name, "role": employee.role}


async def list_employees(
    query: ListEmployees,
    employee_repository: IEmployeeRepository,
):
    employees = await employee_repository.list_all()
    result = [{"id": e.id, "name": e.name, "role": e.role} for e in employees]
    if query.search:
        q = query.search.lower()
        result = [r for r in result if q in r["name"].lower() or q in r["role"].lower()]
    return result
