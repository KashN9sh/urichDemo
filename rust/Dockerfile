# Сборка из каталога, где лежат оба репозитория (urich и urichDemo):
#   cd /path/to/projects   # родитель urich и urichDemo
#   docker build -f urichDemo/rust/Dockerfile --build-arg BINARY=auth -t urich-demo-auth .
#   docker build -f urichDemo/rust/Dockerfile --build-arg BINARY=employees -t urich-demo-employees .
#   docker build -f urichDemo/rust/Dockerfile --build-arg BINARY=tasks -t urich-demo-tasks .
#
# Запуск:
#   docker run -p 8001:8001 -e HOST=0.0.0.0 -e PORT=8001 urich-demo-auth
#   docker run -p 8002:8002 -e HOST=0.0.0.0 -e PORT=8002 urich-demo-employees
#   docker run -p 8003:8003 -e HOST=0.0.0.0 -e PORT=8003 -e EMPLOYEES_SERVICE_URL=http://host.docker.internal:8002 urich-demo-tasks

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
COPY --from=builder /app/urichDemo/rust/target/release/$BINARY /app/service
WORKDIR /app
ENV HOST=0.0.0.0
EXPOSE 8001 8002 8003
CMD ["/app/service"]
