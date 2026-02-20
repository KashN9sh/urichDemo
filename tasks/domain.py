"""Tasks domain: aggregate and events."""
from dataclasses import dataclass
from urich.domain import AggregateRoot, DomainEvent


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


class Task(AggregateRoot):
    def __init__(self, id: str, title: str, assignee_id: str):
        super().__init__(id=id)
        self.title = title
        self.assignee_id = assignee_id
        self.status = "open"
        self.raise_event(TaskCreated(task_id=id, title=title, assignee_id=assignee_id))

    @classmethod
    def from_db(cls, id: str, title: str, assignee_id: str, status: str) -> "Task":
        """Reconstitute from persistence without raising domain events."""
        instance = object.__new__(cls)
        AggregateRoot.__init__(instance, id=id)
        instance.title = title
        instance.assignee_id = assignee_id
        instance.status = status
        return instance

    def assign(self, assignee_id: str) -> None:
        self.assignee_id = assignee_id
        self.raise_event(TaskAssigned(task_id=self.id, assignee_id=assignee_id))

    def complete(self) -> None:
        self.status = "done"
        self.raise_event(TaskCompleted(task_id=self.id))
