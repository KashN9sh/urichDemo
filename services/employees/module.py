"""Bounded context: employees."""
from urich.ddd import DomainModule

from .domain import Employee, EmployeeCreated
from .application import (
    CreateEmployee,
    create_employee,
    GetEmployee,
    get_employee,
    ListEmployees,
    list_employees,
)
from .infrastructure import IEmployeeRepository, EmployeeRepositoryImpl


def on_employee_created(event: EmployeeCreated) -> None:
    """Domain event handler (e.g. sync to HR system)."""
    ...


employees_module = (
    DomainModule("employees")
    .aggregate(Employee)
    .repository(IEmployeeRepository, EmployeeRepositoryImpl)
    .command(CreateEmployee, create_employee)
    .query(GetEmployee, get_employee)
    .query(ListEmployees, list_employees)
    .on_event(EmployeeCreated, on_employee_created)
)
