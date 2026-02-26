"""Tasks domain: aggregate and events."""
from dataclasses import dataclass
from urich.domain import DomainEvent


@dataclass
class TaskCreated(DomainEvent):
    task_id: str
    title: str
    assignee_id: str


@dataclass
class TaskAssigned(DomainEvent):
    task_id: str
    assignee_id: str


@dataclass
class TaskCompleted(DomainEvent):
    task_id: str


@dataclass
class Task:
    id: str
    title: str
    assignee_id: str
    status: str = "open"

    @classmethod
    def from_db(cls, id: str, title: str, assignee_id: str, status: str) -> "Task":
        return cls(id=id, title=title, assignee_id=assignee_id, status=status)

    def assign(self, assignee_id: str) -> None:
        self.assignee_id = assignee_id

    def complete(self) -> None:
        self.status = "done"
