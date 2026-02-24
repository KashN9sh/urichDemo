//! Bounded context: employees. DI: Db из контейнера (как services/employees).

mod rpc_handler;

use serde::Deserialize;
use serde_json::Value;
use urich_rs::{Command, ContainerError, CoreError, DomainModule, Query};
use rusqlite::Connection;

use crate::shared::Db;

#[derive(Debug, Deserialize, Command)]
pub struct CreateEmployee {
    pub employee_id: String,
    pub name: String,
    pub role: String,
}

#[derive(Debug, Deserialize, Query)]
pub struct GetEmployee {
    pub employee_id: String,
}

#[derive(Debug, Deserialize, Query)]
pub struct ListEmployees {
    pub search: String,
}

fn container_err(e: ContainerError) -> CoreError {
    CoreError::Validation(e.to_string())
}

fn create_employee(cmd: CreateEmployee, conn: &Connection) -> Result<Value, CoreError> {
    conn.execute(
        "INSERT INTO employees (id, name, role) VALUES (?1, ?2, ?3)",
        rusqlite::params![cmd.employee_id, cmd.name, cmd.role],
    )
    .map_err(|e| CoreError::Validation(e.to_string()))?;
    Ok(serde_json::json!({ "ok": true, "employee_id": cmd.employee_id }))
}

pub(crate) fn get_employee(query: GetEmployee, conn: &Connection) -> Result<Value, CoreError> {
    let mut stmt = conn
        .prepare("SELECT id, name, role FROM employees WHERE id = ?1")
        .map_err(|e| CoreError::Validation(e.to_string()))?;
    let row = stmt
        .query_row([&query.employee_id], |r| {
            Ok(serde_json::json!({
                "id": r.get::<_, String>(0)?,
                "name": r.get::<_, String>(1)?,
                "role": r.get::<_, String>(2)?,
            }))
        });
    drop(stmt);
    match row {
        Ok(v) => Ok(v),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(serde_json::Value::Null),
        Err(e) => Err(CoreError::Validation(e.to_string())),
    }
}

fn list_employees(query: ListEmployees, conn: &Connection) -> Result<Value, CoreError> {
    let list: Vec<Value> = if query.search.is_empty() {
        let mut stmt = conn
            .prepare("SELECT id, name, role FROM employees")
            .map_err(|e| CoreError::Validation(e.to_string()))?;
        let rows = stmt
            .query_map([], |r| {
                Ok(serde_json::json!({
                    "id": r.get::<_, String>(0)?,
                    "name": r.get::<_, String>(1)?,
                    "role": r.get::<_, String>(2)?,
                }))
            })
            .map_err(|e| CoreError::Validation(e.to_string()))?;
        rows.filter_map(|r| r.ok()).collect()
    } else {
        let pattern = format!("%{}%", query.search);
        let mut stmt = conn
            .prepare("SELECT id, name, role FROM employees WHERE LOWER(name) LIKE LOWER(?1) OR LOWER(role) LIKE LOWER(?1)")
            .map_err(|e| CoreError::Validation(e.to_string()))?;
        let rows = stmt
            .query_map([&pattern], |r| {
                Ok(serde_json::json!({
                    "id": r.get::<_, String>(0)?,
                    "name": r.get::<_, String>(1)?,
                    "role": r.get::<_, String>(2)?,
                }))
            })
            .map_err(|e| CoreError::Validation(e.to_string()))?;
        rows.filter_map(|r| r.ok()).collect()
    };
    Ok(serde_json::Value::Array(list))
}

/// Модуль employees: обработчики получают Db из контейнера (DI). Доступ к БД через spawn_blocking.
pub fn employees_module() -> DomainModule {
    DomainModule::new("employees")
        .command("CreateEmployee", |body, container| {
            let container = std::sync::Arc::clone(&container);
            Box::pin(async move {
                let (db, cmd) = {
                    let mut guard = container.lock().unwrap();
                    let db = guard.resolve::<Db>().map_err(container_err)?.clone();
                    let cmd: CreateEmployee = serde_json::from_value(body).map_err(|e| CoreError::Validation(e.to_string()))?;
                    (db, cmd)
                };
                db.run(move |conn| create_employee(cmd, conn)).await
            })
        })
        .query("GetEmployee", |body, container| {
            let container = std::sync::Arc::clone(&container);
            Box::pin(async move {
                let (db, q) = {
                    let mut guard = container.lock().unwrap();
                    let db = guard.resolve::<Db>().map_err(container_err)?.clone();
                    let q: GetEmployee = serde_json::from_value(body).map_err(|e| CoreError::Validation(e.to_string()))?;
                    (db, q)
                };
                db.run(move |conn| get_employee(q, conn)).await
            })
        })
        .query("ListEmployees", |body, container| {
            let container = std::sync::Arc::clone(&container);
            Box::pin(async move {
                let (db, q) = {
                    let mut guard = container.lock().unwrap();
                    let db = guard.resolve::<Db>().map_err(container_err)?.clone();
                    let q: ListEmployees = serde_json::from_value(body).map_err(|e| CoreError::Validation(e.to_string()))?;
                    (db, q)
                };
                db.run(move |conn| list_employees(q, conn)).await
            })
        })
}

/// RPC handler: Db резолвится из контейнера при каждом запросе (как в Python).
pub fn rpc_handler() -> rpc_handler::EmployeesRpcHandler {
    rpc_handler::EmployeesRpcHandler
}
