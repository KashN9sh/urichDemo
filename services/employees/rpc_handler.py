"""RPC-обработчик для вызовов из других сервисов (например Tasks)."""
import json

from urich.rpc.protocol import RpcServerHandler

from .infrastructure import IEmployeeRepository


class EmployeesRpcHandler:
    """Обрабатывает RPC-методы: get_employee(employee_id) → {id, name, role} или null."""

    def __init__(self, employee_repository: IEmployeeRepository) -> None:
        self._repo = employee_repository

    async def handle(self, method: str, payload: bytes) -> bytes:
        if method == "get_employee":
            try:
                params = json.loads(payload.decode() or "{}")
                employee_id = params.get("employee_id") or ""
            except Exception:
                return json.dumps(None).encode()
            employee = await self._repo.get(employee_id)
            if employee is None:
                return json.dumps(None).encode()
            return json.dumps({"id": employee.id, "name": employee.name, "role": employee.role}).encode()
        return json.dumps({"error": f"unknown method {method}"}).encode()
