//! Точка входа сервиса Employees. DI: Db в контейнере (как services/employees).

use urich_demo_rust::employees;
use urich_demo_rust::shared;
use urich_rs::{Application, RpcModule};

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let db = shared::Db::open()?;
    let mut app = Application::new();
    app.with_container_mut(|c| c.register_instance(db.clone()));
    app.add_middleware(shared::jwt_validation_middleware(&["docs", "openapi.json", "rpc"]));
    let mut employees_module = employees::employees_module();
    app.register(&mut employees_module)?;
    let mut rpc = RpcModule::new()
        .server("rpc", Box::new(employees::rpc_handler()))
        .methods(&["get_employee"]);
    app.register(&mut rpc)?;

    println!("Employees: http://127.0.0.1:8002");
    println!("  POST /employees/commands/create_employee  GET /employees/queries/get_employee  list_employees");
    println!("  RPC /rpc  method get_employee");
    app.run("127.0.0.1", 8002, "Employees Service", "0.1.0")
}
