# Сборка из корня: docker build -f services/employees/Dockerfile .
FROM python:3.12-slim
WORKDIR /app

COPY pyproject.toml ./
RUN pip install --no-cache-dir urich uvicorn "sqlalchemy[asyncio]>=2.0" asyncpg "pyjwt>=2.0" "bcrypt>=4.0"

COPY . .
COPY services/employees/main.py main.py

EXPOSE 8000
CMD ["uvicorn", "main:app", "--host", "0.0.0.0", "--port", "8000"]
