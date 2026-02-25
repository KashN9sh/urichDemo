//! Один dev-бинарник: сервис выбирается по URICH_SERVICE или первому аргументу (auth | employees | tasks).
//! Пример: URICH_SERVICE=employees cargo run --bin dev   или   cargo run --bin dev -- employees

use std::collections::HashMap;
use std::env;
use std::sync::Arc;

use urich_demo_rust::{auth, employees, shared, tasks};
use urich_rs::{host_port_from_env_and_args, Application, RpcClient, RpcModule, StaticDiscovery};

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let service = env::var("URICH_SERVICE")
        .or_else(|_| env::var("BINARY"))
        .unwrap_or_else(|_| {
            env::args()
                .nth(1)
                .unwrap_or_else(|| "auth".to_string())
        });

    match service.as_str() {
        "auth" => run_auth(),
        "employees" => run_employees(),
        "tasks" => run_tasks(),
        _ => {
            eprintln!("Unknown URICH_SERVICE={}. Use: auth, employees, tasks", service);
            std::process::exit(1);
        }
    }
}

fn run_auth() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let db = shared::Db::open()?;
    let mut app = Application::new();
    app.with_container_mut(|c| {
        c.register_instance(db);
        c.register_factory(|c| auth::LoginHandler {
            db: c.resolve::<shared::Db>().unwrap().clone(),
        });
        c.register_factory(|c| auth::RegisterHandler {
            db: c.resolve::<shared::Db>().unwrap().clone(),
        });
    });
    let mut auth_mod = auth::auth_module();
    app.register(&mut auth_mod)?;
    let (host, port) = host_port_from_env_and_args("127.0.0.1", 8001);
    println!("Auth: http://{}:{}", host, port);
    println!("  POST /auth/commands/login  POST /auth/commands/register");
    app.run_from_env("127.0.0.1", 8001, "Auth Service", "0.1.0")
}

fn run_employees() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let db = shared::Db::open()?;
    let mut app = Application::new();
    app.with_container_mut(|c| {
        c.register_instance(db.clone());
        c.register_factory(|c| employees::CreateEmployeeHandler {
            db: c.resolve::<shared::Db>().unwrap().clone(),
        });
        c.register_factory(|c| employees::GetEmployeeHandler {
            db: c.resolve::<shared::Db>().unwrap().clone(),
        });
        c.register_factory(|c| employees::ListEmployeesHandler {
            db: c.resolve::<shared::Db>().unwrap().clone(),
        });
    });
    app.add_middleware(shared::jwt_validation_middleware(&["docs", "openapi.json", "rpc"]));
    let mut employees_module = employees::employees_module();
    app.register(&mut employees_module)?;
    let mut rpc = RpcModule::new()
        .server("rpc", Box::new(employees::rpc_handler()))
        .methods(&["get_employee"]);
    app.register(&mut rpc)?;

    let (host, port) = host_port_from_env_and_args("127.0.0.1", 8002);
    println!("Employees: http://{}:{}", host, port);
    println!("  POST /employees/commands/create_employee  GET /employees/queries/get_employee  list_employees");
    println!("  RPC /rpc  method get_employee");
    app.run_from_env("127.0.0.1", 8002, "Employees Service", "0.1.0")
}

fn run_tasks() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let employees_url = env::var("EMPLOYEES_SERVICE_URL").unwrap_or_else(|_| "http://127.0.0.1:8002".to_string());
    let mut services = HashMap::new();
    services.insert("employees".to_string(), employees_url.clone());

    let discovery = Box::new(StaticDiscovery::new(services));
    let transport = Box::new(shared::HttpRpcTransport::new("rpc"));
    let client = Arc::new(RpcClient::new(discovery, transport));
    let db = shared::Db::open()?;

    let mut app = Application::new();
    app.with_container_mut(|c| {
        c.register_instance(db);
        c.register_instance(client);
        c.register_factory(|c| tasks::CreateTaskHandler {
            db: c.resolve::<shared::Db>().unwrap().clone(),
            rpc_client: c.resolve::<Arc<RpcClient>>().unwrap().clone(),
        });
        c.register_factory(|c| tasks::AssignTaskHandler {
            db: c.resolve::<shared::Db>().unwrap().clone(),
            rpc_client: c.resolve::<Arc<RpcClient>>().unwrap().clone(),
        });
        c.register_factory(|c| tasks::CompleteTaskHandler {
            db: c.resolve::<shared::Db>().unwrap().clone(),
        });
        c.register_factory(|c| tasks::GetTaskHandler {
            db: c.resolve::<shared::Db>().unwrap().clone(),
        });
        c.register_factory(|c| tasks::ListTasksByEmployeeHandler {
            db: c.resolve::<shared::Db>().unwrap().clone(),
        });
    });
    app.add_middleware(shared::jwt_validation_middleware(&["docs", "openapi.json"]));
    let mut tasks_module = tasks::tasks_module();
    app.register(&mut tasks_module)?;

    let (host, port) = host_port_from_env_and_args("127.0.0.1", 8003);
    println!("Tasks: http://{}:{}  (EMPLOYEES_SERVICE_URL={})", host, port, employees_url);
    println!("  POST /tasks/commands/create_task  assign_task  complete_task");
    println!("  GET  /tasks/queries/get_task  list_tasks_by_employee");
    app.run_from_env("127.0.0.1", 8003, "Tasks Service", "0.1.0")
}
