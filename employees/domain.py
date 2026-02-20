"""Employees domain: aggregate and events."""
from dataclasses import dataclass
from urich.domain import AggregateRoot, DomainEvent


@dataclass
class EmployeeCreated(DomainEvent):
    employee_id: str
    name: str
    role: str


class Employee(AggregateRoot):
    def __init__(self, id: str, name: str, role: str):
        super().__init__(id=id)
        self.name = name
        self.role = role
        self.raise_event(EmployeeCreated(employee_id=id, name=name, role=role))

    @classmethod
    def from_db(cls, id: str, name: str, role: str) -> "Employee":
        """Reconstitute from persistence without raising domain events."""
        instance = object.__new__(cls)
        AggregateRoot.__init__(instance, id=id)
        instance.name = name
        instance.role = role
        return instance
