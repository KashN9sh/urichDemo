//! Точка входа сервиса Auth. DI: Db в контейнере (как services/auth).

use urich_demo_rust::auth;
use urich_demo_rust::shared::Db;
use urich_rs::Application;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let db = Db::open()?;
    let mut app = Application::new();
    app.with_container_mut(|c| c.register_instance(db));
    auth::register_auth(&mut app)?;
    println!("Auth: http://127.0.0.1:8001");
    println!("  POST /auth/login  POST /auth/register");
    app.run("127.0.0.1", 8001, "Auth Service", "0.1.0")
}
