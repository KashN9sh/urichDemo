"""Auth domain: события (опционально для единообразия с другими контекстами)."""
from dataclasses import dataclass

from urich.domain import DomainEvent


@dataclass
class UserRegistered(DomainEvent):
    """Событие после успешной регистрации пользователя."""
    user_id: str
    username: str
    role: str
