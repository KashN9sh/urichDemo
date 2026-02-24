//! Точка входа по умолчанию: запуск сервиса tasks (как services/tasks).
//! Отдельные сервисы: cargo run --bin auth (8001), --bin employees (8002), --bin tasks (8003).

fn main() {
    eprintln!("Usage: cargo run --bin auth | --bin employees | --bin tasks");
    eprintln!("  auth:      127.0.0.1:8001  POST /auth/login /auth/register");
    eprintln!("  employees: 127.0.0.1:8002  commands/queries + RPC /rpc");
    eprintln!("  tasks:     127.0.0.1:8003  commands/queries, RPC to employees");
    std::process::exit(1);
}
