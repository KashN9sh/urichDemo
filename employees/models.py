"""SQLAlchemy model for employees table."""
from sqlalchemy import String
from sqlalchemy.orm import Mapped, mapped_column

from database.base import Base


class EmployeeModel(Base):
    __tablename__ = "employees"

    id: Mapped[str] = mapped_column(String(64), primary_key=True)
    name: Mapped[str] = mapped_column(String(256), nullable=False)
    role: Mapped[str] = mapped_column(String(128), nullable=False)
