"""Bounded context: tasks."""
from urich.ddd import DomainModule

from .domain import Task, TaskCreated, TaskAssigned, TaskCompleted
from .application import (
    CreateTask,
    CreateTaskHandler,
    AssignTask,
    AssignTaskHandler,
    CompleteTask,
    CompleteTaskHandler,
    GetTask,
    GetTaskHandler,
    ListTasksByEmployee,
    ListTasksByEmployeeHandler,
)
from .infrastructure import ITaskRepository, TaskRepositoryImpl


def on_task_created(event: TaskCreated) -> None:
    """Domain event handler (e.g. notify assignee)."""
    ...


def on_task_assigned(event: TaskAssigned) -> None:
    """Domain event handler (e.g. send assignment notification)."""
    ...


tasks_module = (
    DomainModule("tasks")
    .aggregate(Task)
    .repository(ITaskRepository, TaskRepositoryImpl)
    .command(CreateTask, CreateTaskHandler)
    .command(AssignTask, AssignTaskHandler)
    .command(CompleteTask, CompleteTaskHandler)
    .query(GetTask, GetTaskHandler)
    .query(ListTasksByEmployee, ListTasksByEmployeeHandler)
    .on_event(TaskCreated, on_task_created)
    .on_event(TaskAssigned, on_task_assigned)
    .on_event(TaskCompleted, lambda e: None)
)
