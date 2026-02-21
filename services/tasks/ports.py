"""Порты для обмена с другими сервисами (Tasks вызывает Employees по RPC)."""
from typing import Protocol


class IEmployeeService(Protocol):
    """Проверка/получение сотрудника. Реализация — RpcEmployeeService (urich.discovery + urich.rpc)."""

    async def get_employee(self, employee_id: str) -> dict | None:
        """Возвращает {"id", "name", "role"} или None, если не найден."""
        ...
