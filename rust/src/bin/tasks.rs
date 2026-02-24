//! Точка входа сервиса Tasks. DI: Db и Arc<RpcClient> в контейнере (как services/tasks).

use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use urich_demo_rust::shared;
use urich_demo_rust::tasks;
use urich_rs::{host_port_from_env_and_args, Application, RpcClient, StaticDiscovery};

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
