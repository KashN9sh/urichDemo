//! Точка входа сервиса Auth. DI: Db в контейнере (как services/auth). DomainModule: POST /auth/commands/login, /auth/commands/register.

use urich_demo_rust::auth;
use urich_demo_rust::shared::Db;
use urich_rs::{host_port_from_env_and_args, Application};

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let db = Db::open()?;
    let mut app = Application::new();
    app.with_container_mut(|c| {
        c.register_instance(db);
        c.register_factory(|c| auth::LoginHandler {
            db: c.resolve::<Db>().unwrap().clone(),
        });
        c.register_factory(|c| auth::RegisterHandler {
            db: c.resolve::<Db>().unwrap().clone(),
        });
    });
    let mut auth_mod = auth::auth_module();
    app.register(&mut auth_mod)?;
    let (host, port) = host_port_from_env_and_args("127.0.0.1", 8001);
    println!("Auth: http://{}:{}", host, port);
    println!("  POST /auth/commands/login  POST /auth/commands/register");
    app.run_from_env("127.0.0.1", 8001, "Auth Service", "0.1.0")
}
