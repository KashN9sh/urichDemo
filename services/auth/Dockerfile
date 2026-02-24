# Сборка из корня: docker build -f services/auth/Dockerfile .
FROM python:3.12-slim
WORKDIR /app

COPY pyproject.toml ./
RUN pip install --no-cache-dir urich "sqlalchemy[asyncio]>=2.0" asyncpg "pyjwt>=2.0" "bcrypt>=4.0"

COPY . .
COPY services/auth/main.py main.py

EXPOSE 8000
CMD ["urich", "main:app", "--host", "0.0.0.0", "--port", "8000"]
