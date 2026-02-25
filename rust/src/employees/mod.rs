//! Bounded context: employees. Handler как тип (DI из контейнера), как services/employees.

mod rpc_handler;

use serde::Deserialize;
use serde_json::Value;
use urich_rs::{Command, CommandHandler, DomainModule, IntoCoreError, Query, QueryHandler};
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

fn create_employee(cmd: CreateEmployee, conn: &Connection) -> Result<Value, urich_rs::CoreError> {
    conn.execute(
        "INSERT INTO employees (id, name, role) VALUES (?1, ?2, ?3)",
        rusqlite::params![cmd.employee_id, cmd.name, cmd.role],
    )
    .map_err(IntoCoreError::into_core_error)?;
    Ok(serde_json::json!({ "ok": true, "employee_id": cmd.employee_id }))
}

fn get_employee(query: GetEmployee, conn: &Connection) -> Result<Value, urich_rs::CoreError> {
    let mut stmt = conn
        .prepare("SELECT id, name, role FROM employees WHERE id = ?1")
        .map_err(IntoCoreError::into_core_error)?;
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
        Err(e) => Err(urich_rs::CoreError::Validation(e.to_string())),
    }
}

fn list_employees(query: ListEmployees, conn: &Connection) -> Result<Value, urich_rs::CoreError> {
    let list: Vec<Value> = if query.search.is_empty() {
        let mut stmt = conn
            .prepare("SELECT id, name, role FROM employees")
            .map_err(IntoCoreError::into_core_error)?;
        let rows = stmt
            .query_map([], |r| {
                Ok(serde_json::json!({
                    "id": r.get::<_, String>(0)?,
                    "name": r.get::<_, String>(1)?,
                    "role": r.get::<_, String>(2)?,
                }))
            })
            .map_err(IntoCoreError::into_core_error)?;
        rows.filter_map(|r| r.ok()).collect()
    } else {
        let pattern = format!("%{}%", query.search);
        let mut stmt = conn
            .prepare("SELECT id, name, role FROM employees WHERE LOWER(name) LIKE LOWER(?1) OR LOWER(role) LIKE LOWER(?1)")
            .map_err(IntoCoreError::into_core_error)?;
        let rows = stmt
            .query_map([&pattern], |r| {
                Ok(serde_json::json!({
                    "id": r.get::<_, String>(0)?,
                    "name": r.get::<_, String>(1)?,
                    "role": r.get::<_, String>(2)?,
                }))
            })
            .map_err(IntoCoreError::into_core_error)?;
        rows.filter_map(|r| r.ok()).collect()
    };
    Ok(serde_json::Value::Array(list))
}

#[derive(Clone)]
pub struct CreateEmployeeHandler {
    pub db: Db,
}

#[async_trait::async_trait]
impl CommandHandler<CreateEmployee> for CreateEmployeeHandler {
    async fn handle(&self, cmd: CreateEmployee) -> Result<Value, urich_rs::CoreError> {
        self.db.run(move |conn| create_employee(cmd, conn)).await
    }
}

#[derive(Clone)]
pub struct GetEmployeeHandler {
    pub db: Db,
}

#[async_trait::async_trait]
impl QueryHandler<GetEmployee> for GetEmployeeHandler {
    async fn handle(&self, query: GetEmployee) -> Result<Value, urich_rs::CoreError> {
        self.db.run(move |conn| get_employee(query, conn)).await
    }
}

#[derive(Clone)]
pub struct ListEmployeesHandler {
    pub db: Db,
}

#[async_trait::async_trait]
impl QueryHandler<ListEmployees> for ListEmployeesHandler {
    async fn handle(&self, query: ListEmployees) -> Result<Value, urich_rs::CoreError> {
        self.db.run(move |conn| list_employees(query, conn)).await
    }
}

/// Модуль employees: handler как тип, резолвится из контейнера. Как Python DomainModule.
pub fn employees_module() -> DomainModule {
    DomainModule::new("employees")
        .command_with_handler::<CreateEmployee, CreateEmployeeHandler>()
        .query_with_handler::<GetEmployee, GetEmployeeHandler>()
        .query_with_handler::<ListEmployees, ListEmployeesHandler>()
}

pub fn rpc_handler() -> rpc_handler::EmployeesRpcHandler {
    rpc_handler::EmployeesRpcHandler
}
