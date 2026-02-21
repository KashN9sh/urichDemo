"""RPC-обработчик для вызовов из других сервисов (например Tasks)."""
from urich.rpc import RpcServer

from .infrastructure import IEmployeeRepository


class EmployeesRpcHandler(RpcServer):
    """RPC-методы: get_employee(employee_id) → {id, name, role} или null."""

    def __init__(self, employee_repository: IEmployeeRepository) -> None:
        self._repo = employee_repository

    async def get_employee(self, employee_id: str = "") -> dict | None:
        employee = await self._repo.get(employee_id)
        if employee is None:
            return None
        return {"id": employee.id, "name": employee.name, "role": employee.role}
