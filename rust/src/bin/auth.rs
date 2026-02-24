//! Точка входа сервиса Auth. DI: Db в контейнере (как services/auth).
//! Запуск: как uvicorn — host/port из env (HOST, PORT) или из --host/--port.

use urich_demo_rust::auth;
use urich_demo_rust::shared::Db;
use urich_rs::{host_port_from_env_and_args, Application};

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let db = Db::open()?;
    let mut app = Application::new();
    app.with_container_mut(|c| c.register_instance(db));
    auth::register_auth(&mut app)?;
    let (host, port) = host_port_from_env_and_args("127.0.0.1", 8001);
    println!("Auth: http://{}:{}", host, port);
    println!("  POST /auth/login  POST /auth/register");
    app.run_from_env("127.0.0.1", 8001, "Auth Service", "0.1.0")
}
