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


class CreateEmployeeHandler:
    def __init__(self, employee_repository: IEmployeeRepository, event_bus: EventBus):
        self._repo = employee_repository
        self._event_bus = event_bus

    async def __call__(self, cmd: CreateEmployee) -> str:
        employee = Employee(id=cmd.employee_id, name=cmd.name, role=cmd.role)
        await self._repo.add(employee)
        for event in employee.collect_pending_events():
            await self._event_bus.publish(event)
        return employee.id


class GetEmployeeHandler:
    def __init__(self, employee_repository: IEmployeeRepository):
        self._repo = employee_repository

    async def __call__(self, query: GetEmployee):
        employee = await self._repo.get(query.employee_id)
        if employee is None:
            return None
        return {"id": employee.id, "name": employee.name, "role": employee.role}


class ListEmployeesHandler:
    def __init__(self, employee_repository: IEmployeeRepository):
        self._repo = employee_repository

    async def __call__(self, query: ListEmployees):
        employees = await self._repo.list_all()
        result = [{"id": e.id, "name": e.name, "role": e.role} for e in employees]
        if query.search:
            q = query.search.lower()
            result = [r for r in result if q in r["name"].lower() or q in r["role"].lower()]
        return result
