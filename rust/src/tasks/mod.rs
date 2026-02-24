//! Bounded context: tasks. DI: Db и Arc<RpcClient> из контейнера (как services/tasks).

use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::Arc;
use urich_rs::{Command, ContainerError, CoreError, DomainModule, Query, RpcClient};

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

fn container_err(e: ContainerError) -> CoreError {
    CoreError::Validation(e.to_string())
}

async fn create_task_with_rpc(cmd: CreateTask, client: &RpcClient, db: &Db) -> Result<Value, CoreError> {
    let emp = client
        .call("employees", "get_employee", json!({ "employee_id": cmd.assignee_id }))
        .await
        .map_err(|e| CoreError::Validation(e.to_string()))?;
    if emp.is_null() || (emp.get("id").is_none() && emp.get("name").is_none()) {
        return Err(CoreError::Validation(format!(
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
        .map_err(|e| CoreError::Validation(e.to_string()))?;
        Ok(json!({ "ok": true, "task_id": cmd.task_id }))
    })
    .await
}

async fn assign_task_with_rpc(cmd: AssignTask, client: &RpcClient, db: &Db) -> Result<Value, CoreError> {
    let emp = client
        .call("employees", "get_employee", json!({ "employee_id": cmd.assignee_id }))
        .await
        .map_err(|e| CoreError::Validation(e.to_string()))?;
    if emp.is_null() || (emp.get("id").is_none() && emp.get("name").is_none()) {
        return Err(CoreError::Validation(format!(
            "Assignee '{}' not found",
            cmd.assignee_id
        )));
    }
    let task_id = cmd.task_id.clone();
    let n = db.run(move |conn| {
        conn.execute("UPDATE tasks SET assignee_id = ?1 WHERE id = ?2", rusqlite::params![cmd.assignee_id, cmd.task_id])
            .map_err(|e| CoreError::Validation(e.to_string()))
    }).await?;
    if n == 0 {
        return Err(CoreError::Validation(format!("Task '{}' not found", task_id)));
    }
    Ok(json!({ "ok": true }))
}

fn complete_task(cmd: CompleteTask, conn: &Connection) -> Result<Value, CoreError> {
    conn.execute("UPDATE tasks SET status = 'done' WHERE id = ?1", [&cmd.task_id])
        .map_err(|e| CoreError::Validation(e.to_string()))?;
    Ok(json!({ "ok": true }))
}

fn get_task(query: GetTask, conn: &Connection) -> Result<Value, CoreError> {
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
        Err(e) => Err(CoreError::Validation(e.to_string())),
    }
}

fn list_tasks_by_employee(query: ListTasksByEmployee, conn: &Connection) -> Result<Value, CoreError> {
    let mut stmt = conn
        .prepare("SELECT id, title, assignee_id, status FROM tasks WHERE assignee_id = ?1")
        .map_err(|e| CoreError::Validation(e.to_string()))?;
    let rows = stmt
        .query_map([&query.employee_id], |r| {
            Ok(json!({
                "task_id": r.get::<_, String>(0)?,
                "title": r.get::<_, String>(1)?,
                "assignee_id": r.get::<_, String>(2)?,
                "status": r.get::<_, String>(3)?,
            }))
        })
        .map_err(|e| CoreError::Validation(e.to_string()))?;
    let list: Vec<Value> = rows.filter_map(|r| r.ok()).collect();
    Ok(Value::Array(list))
}

/// Модуль tasks: Db и Arc<RpcClient> из контейнера (DI, как Python services/tasks).
pub fn tasks_module() -> DomainModule {
    DomainModule::new("tasks")
        .command("CreateTask", |body, container| {
            let container = std::sync::Arc::clone(&container);
            Box::pin(async move {
                let (client, db) = {
                    let mut guard = container.lock().unwrap();
                    let client = guard.resolve::<Arc<RpcClient>>().map_err(container_err)?.clone();
                    let db = guard.resolve::<Db>().map_err(container_err)?.clone();
                    (client, db)
                };
                let cmd: CreateTask = serde_json::from_value(body).map_err(|e| CoreError::Validation(e.to_string()))?;
                create_task_with_rpc(cmd, client.as_ref(), &db).await
            })
        })
        .command("AssignTask", |body, container| {
            let container = std::sync::Arc::clone(&container);
            Box::pin(async move {
                let (client, db) = {
                    let mut guard = container.lock().unwrap();
                    let client = guard.resolve::<Arc<RpcClient>>().map_err(container_err)?.clone();
                    let db = guard.resolve::<Db>().map_err(container_err)?.clone();
                    (client, db)
                };
                let cmd: AssignTask = serde_json::from_value(body).map_err(|e| CoreError::Validation(e.to_string()))?;
                assign_task_with_rpc(cmd, client.as_ref(), &db).await
            })
        })
        .command("CompleteTask", |body, container| {
            let container = std::sync::Arc::clone(&container);
            Box::pin(async move {
                let (db, cmd) = {
                    let mut guard = container.lock().unwrap();
                    let db = guard.resolve::<Db>().map_err(container_err)?.clone();
                    let cmd: CompleteTask = serde_json::from_value(body).map_err(|e| CoreError::Validation(e.to_string()))?;
                    (db, cmd)
                };
                db.run(move |conn| complete_task(cmd, conn)).await
            })
        })
        .query("GetTask", |body, container| {
            let container = std::sync::Arc::clone(&container);
            Box::pin(async move {
                let (db, q) = {
                    let mut guard = container.lock().unwrap();
                    let db = guard.resolve::<Db>().map_err(container_err)?.clone();
                    let q: GetTask = serde_json::from_value(body).map_err(|e| CoreError::Validation(e.to_string()))?;
                    (db, q)
                };
                db.run(move |conn| get_task(q, conn)).await
            })
        })
        .query("ListTasksByEmployee", |body, container| {
            let container = std::sync::Arc::clone(&container);
            Box::pin(async move {
                let (db, q) = {
                    let mut guard = container.lock().unwrap();
                    let db = guard.resolve::<Db>().map_err(container_err)?.clone();
                    let q: ListTasksByEmployee = serde_json::from_value(body).map_err(|e| CoreError::Validation(e.to_string()))?;
                    (db, q)
                };
                db.run(move |conn| list_tasks_by_employee(q, conn)).await
            })
        })
}
