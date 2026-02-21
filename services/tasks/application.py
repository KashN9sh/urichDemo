"""Application layer: commands, queries, handlers."""
from __future__ import annotations

from dataclasses import dataclass
from urich.ddd import Command, Query
from urich.domain import EventBus

from .domain import Task, TaskCreated, TaskAssigned, TaskCompleted
from .infrastructure import ITaskRepository
from .ports import IEmployeeService


@dataclass
class CreateTask(Command):
    task_id: str
    title: str
    assignee_id: str


@dataclass
class AssignTask(Command):
    task_id: str
    assignee_id: str


@dataclass
class CompleteTask(Command):
    task_id: str


@dataclass
class GetTask(Query):
    task_id: str


@dataclass
class ListTasksByEmployee(Query):
    employee_id: str


class CreateTaskHandler:
    def __init__(
        self,
        task_repository: ITaskRepository,
        event_bus: EventBus,
        employee_service: IEmployeeService,
    ):
        self._repo = task_repository
        self._event_bus = event_bus
        self._employee_service = employee_service

    async def __call__(self, cmd: CreateTask) -> str:
        assignee = await self._employee_service.get_employee(cmd.assignee_id)
        if not assignee:
            raise ValueError(f"Assignee '{cmd.assignee_id}' not found")
        task = Task(id=cmd.task_id, title=cmd.title, assignee_id=cmd.assignee_id)
        await self._repo.add(task)
        for event in task.collect_pending_events():
            await self._event_bus.publish(event)
        return task.id


class AssignTaskHandler:
    def __init__(
        self,
        task_repository: ITaskRepository,
        event_bus: EventBus,
        employee_service: IEmployeeService,
    ):
        self._repo = task_repository
        self._event_bus = event_bus
        self._employee_service = employee_service

    async def __call__(self, cmd: AssignTask) -> None:
        assignee = await self._employee_service.get_employee(cmd.assignee_id)
        if not assignee:
            raise ValueError(f"Assignee '{cmd.assignee_id}' not found")
        task = await self._repo.get(cmd.task_id)
        if task is None:
            raise ValueError(f"Task {cmd.task_id} not found")
        task.assign(cmd.assignee_id)
        await self._repo.save(task)
        for event in task.collect_pending_events():
            await self._event_bus.publish(event)


class CompleteTaskHandler:
    def __init__(self, task_repository: ITaskRepository, event_bus: EventBus):
        self._repo = task_repository
        self._event_bus = event_bus

    async def __call__(self, cmd: CompleteTask) -> None:
        task = await self._repo.get(cmd.task_id)
        if task is None:
            raise ValueError(f"Task {cmd.task_id} not found")
        task.complete()
        await self._repo.save(task)
        for event in task.collect_pending_events():
            await self._event_bus.publish(event)


class GetTaskHandler:
    def __init__(self, task_repository: ITaskRepository):
        self._repo = task_repository

    async def __call__(self, query: GetTask):
        task = await self._repo.get(query.task_id)
        if task is None:
            return None
        return {
            "id": task.id,
            "title": task.title,
            "assignee_id": task.assignee_id,
            "status": task.status,
        }


class ListTasksByEmployeeHandler:
    def __init__(self, task_repository: ITaskRepository):
        self._repo = task_repository

    async def __call__(self, query: ListTasksByEmployee):
        tasks = await self._repo.list_by_assignee(query.employee_id)
        return [
            {"id": t.id, "title": t.title, "assignee_id": t.assignee_id, "status": t.status}
            for t in tasks
        ]
