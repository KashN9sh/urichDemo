"""Bounded context: employees."""
from urich.ddd import DomainModule

from .domain import Employee, EmployeeCreated
from .application import (
    CreateEmployee,
    CreateEmployeeHandler,
    GetEmployee,
    GetEmployeeHandler,
    ListEmployees,
    ListEmployeesHandler,
)
from .infrastructure import IEmployeeRepository, EmployeeRepositoryImpl


def on_employee_created(event: EmployeeCreated) -> None:
    """Domain event handler (e.g. sync to HR system)."""
    ...


employees_module = (
    DomainModule("employees")
    .aggregate(Employee)
    .repository(IEmployeeRepository, EmployeeRepositoryImpl)
    .command(CreateEmployee, CreateEmployeeHandler)
    .query(GetEmployee, GetEmployeeHandler)
    .query(ListEmployees, ListEmployeesHandler)
    .on_event(EmployeeCreated, on_employee_created)
)
