"""Employees domain: aggregate and events. No Urich imports. Aggregate is just data."""
from dataclasses import dataclass


@dataclass
class EmployeeCreated:
    employee_id: str
    name: str
    role: str


@dataclass
class Employee:
    id: str
    name: str
    role: str

    @classmethod
    def from_db(cls, id: str, name: str, role: str) -> "Employee":
        """Reconstitute from persistence."""
        return cls(id=id, name=name, role=role)
