//! Bounded context: tasks. Handler как тип (Db + RpcClient из контейнера), как services/tasks.

use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::Arc;
use urich_rs::{Command, CommandHandler, DomainModule, IntoCoreError, Query, QueryHandler, RpcClient};
use rusqlite::Connection;

use crate::shared::Db;

#[derive(Debug, Deserialize, Command)]
pub struct CreateTask {
    pub task_id: String,
    pub title: String,
    pub assignee_id: String,
}

#[derive(Debug, Deserialize, Command)]
pub struct AssignTask {
    pub task_id: String,
    pub assignee_id: String,
}

#[derive(Debug, Deserialize, Command)]
pub struct CompleteTask {
    pub task_id: String,
}

#[derive(Debug, Deserialize, Query)]
pub struct GetTask {
    pub task_id: String,
}

#[derive(Debug, Deserialize, Query)]
pub struct ListTasksByEmployee {
    pub employee_id: String,
}

async fn create_task_with_rpc(cmd: CreateTask, client: &RpcClient, db: &Db) -> Result<Value, urich_rs::CoreError> {
    let emp = client
        .call("employees", "get_employee", json!({ "employee_id": cmd.assignee_id }))
        .await
        .map_err(IntoCoreError::into_core_error)?;
    if emp.is_null() || (emp.get("id").is_none() && emp.get("name").is_none()) {
        return Err(urich_rs::CoreError::Validation(format!(
            "Assignee '{}' not found",
            cmd.assignee_id
        )));
    }
    let cmd = cmd;
    db.run(move |conn| {
        conn.execute(
            "INSERT INTO tasks (id, title, assignee_id, status) VALUES (?1, ?2, ?3, 'open')",
            rusqlite::params![cmd.task_id, cmd.title, cmd.assignee_id],
        )
        .map_err(IntoCoreError::into_core_error)?;
        Ok(json!({ "ok": true, "task_id": cmd.task_id }))
    })
    .await
}

async fn assign_task_with_rpc(cmd: AssignTask, client: &RpcClient, db: &Db) -> Result<Value, urich_rs::CoreError> {
    let emp = client
        .call("employees", "get_employee", json!({ "employee_id": cmd.assignee_id }))
        .await
        .map_err(IntoCoreError::into_core_error)?;
    if emp.is_null() || (emp.get("id").is_none() && emp.get("name").is_none()) {
        return Err(urich_rs::CoreError::Validation(format!(
            "Assignee '{}' not found",
            cmd.assignee_id
        )));
    }
    let task_id = cmd.task_id.clone();
    let n = db
        .run(move |conn| {
            conn.execute("UPDATE tasks SET assignee_id = ?1 WHERE id = ?2", rusqlite::params![cmd.assignee_id, cmd.task_id])
                .map_err(IntoCoreError::into_core_error)
        })
        .await?;
    if n == 0 {
        return Err(urich_rs::CoreError::Validation(format!("Task '{}' not found", task_id)));
    }
    Ok(json!({ "ok": true }))
}

fn complete_task(cmd: CompleteTask, conn: &Connection) -> Result<Value, urich_rs::CoreError> {
    conn.execute("UPDATE tasks SET status = 'done' WHERE id = ?1", [&cmd.task_id])
        .map_err(IntoCoreError::into_core_error)?;
    Ok(json!({ "ok": true }))
}

fn get_task(query: GetTask, conn: &Connection) -> Result<Value, urich_rs::CoreError> {
    let row = conn.query_row(
        "SELECT id, title, assignee_id, status FROM tasks WHERE id = ?1",
        [&query.task_id],
        |r| {
            Ok(json!({
                "task_id": r.get::<_, String>(0)?,
                "title": r.get::<_, String>(1)?,
                "assignee_id": r.get::<_, String>(2)?,
                "status": r.get::<_, String>(3)?,
            }))
        },
    );
    match row {
        Ok(v) => Ok(v),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(Value::Null),
        Err(e) => Err(urich_rs::CoreError::Validation(e.to_string())),
    }
}

fn list_tasks_by_employee(query: ListTasksByEmployee, conn: &Connection) -> Result<Value, urich_rs::CoreError> {
    let mut stmt = conn
        .prepare("SELECT id, title, assignee_id, status FROM tasks WHERE assignee_id = ?1")
        .map_err(IntoCoreError::into_core_error)?;
    let rows = stmt
        .query_map([&query.employee_id], |r| {
            Ok(json!({
                "task_id": r.get::<_, String>(0)?,
                "title": r.get::<_, String>(1)?,
                "assignee_id": r.get::<_, String>(2)?,
                "status": r.get::<_, String>(3)?,
            }))
        })
        .map_err(IntoCoreError::into_core_error)?;
    let list: Vec<Value> = rows.filter_map(|r| r.ok()).collect();
    Ok(Value::Array(list))
}

#[derive(Clone)]
pub struct CreateTaskHandler {
    pub db: Db,
    pub rpc_client: Arc<RpcClient>,
}

#[async_trait::async_trait]
impl CommandHandler<CreateTask> for CreateTaskHandler {
    async fn handle(&self, cmd: CreateTask) -> Result<Value, urich_rs::CoreError> {
        create_task_with_rpc(cmd, self.rpc_client.as_ref(), &self.db).await
    }
}

#[derive(Clone)]
pub struct AssignTaskHandler {
    pub db: Db,
    pub rpc_client: Arc<RpcClient>,
}

#[async_trait::async_trait]
impl CommandHandler<AssignTask> for AssignTaskHandler {
    async fn handle(&self, cmd: AssignTask) -> Result<Value, urich_rs::CoreError> {
        assign_task_with_rpc(cmd, self.rpc_client.as_ref(), &self.db).await
    }
}

#[derive(Clone)]
pub struct CompleteTaskHandler {
    pub db: Db,
}

#[async_trait::async_trait]
impl CommandHandler<CompleteTask> for CompleteTaskHandler {
    async fn handle(&self, cmd: CompleteTask) -> Result<Value, urich_rs::CoreError> {
        self.db.run(move |conn| complete_task(cmd, conn)).await
    }
}

#[derive(Clone)]
pub struct GetTaskHandler {
    pub db: Db,
}

#[async_trait::async_trait]
impl QueryHandler<GetTask> for GetTaskHandler {
    async fn handle(&self, query: GetTask) -> Result<Value, urich_rs::CoreError> {
        self.db.run(move |conn| get_task(query, conn)).await
    }
}

#[derive(Clone)]
pub struct ListTasksByEmployeeHandler {
    pub db: Db,
}

#[async_trait::async_trait]
impl QueryHandler<ListTasksByEmployee> for ListTasksByEmployeeHandler {
    async fn handle(&self, query: ListTasksByEmployee) -> Result<Value, urich_rs::CoreError> {
        self.db.run(move |conn| list_tasks_by_employee(query, conn)).await
    }
}

/// Модуль tasks: handler как тип, резолвится из контейнера. Как Python DomainModule.
pub fn tasks_module() -> DomainModule {
    DomainModule::new("tasks")
        .command_with_handler::<CreateTask, CreateTaskHandler>()
        .command_with_handler::<AssignTask, AssignTaskHandler>()
        .command_with_handler::<CompleteTask, CompleteTaskHandler>()
        .query_with_handler::<GetTask, GetTaskHandler>()
        .query_with_handler::<ListTasksByEmployee, ListTasksByEmployeeHandler>()
}
