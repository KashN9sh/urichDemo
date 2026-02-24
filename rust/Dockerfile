# Сборка из каталога, где лежат оба репозитория (urich и urichDemo):
#   docker build -f urichDemo/rust/Dockerfile --build-arg BINARY=auth -t urich-demo-auth .
#   docker build -f urichDemo/rust/Dockerfile --build-arg BINARY=employees -t urich-demo-employees .
#   docker build -f urichDemo/rust/Dockerfile --build-arg BINARY=tasks -t urich-demo-tasks .
#
# Запуск (в одном стиле с Python: urich main:app --host 0.0.0.0 --port 8000):
#   docker run -p 8001:8000 urich-demo-auth
#   docker run -p 8002:8000 urich-demo-employees
#   docker run -p 8003:8000 -e EMPLOYEES_SERVICE_URL=http://host.docker.internal:8002 urich-demo-tasks

ARG BINARY=auth
FROM rust:1-bookworm AS builder
ARG BINARY
WORKDIR /app

COPY urich /app/urich
COPY urichDemo /app/urichDemo

WORKDIR /app/urichDemo/rust
RUN cargo build --release --bin $BINARY

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates libssl3 \
  && rm -rf /var/lib/apt/lists/*
ARG BINARY
COPY --from=builder /app/urichDemo/rust/target/release/$BINARY /app/urich
WORKDIR /app
ENV PATH="/app:$PATH"
EXPOSE 8000
CMD ["urich", "--host", "0.0.0.0", "--port", "8000"]
