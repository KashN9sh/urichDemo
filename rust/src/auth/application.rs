//! Обработчики login и register: Db из контейнера (DI, как services/auth). Свои ошибки — через .map_err(IntoCoreError::into_core_error).

use serde_json::{json, Value};
use urich_rs::{CoreError, IntoCoreError};
use uuid::Uuid;

use crate::auth::{Login, Register};
use crate::shared::Db;

fn hash_password(password: &str) -> String {
    format!("hash:{}", password.len())
}

fn verify_password(password: &str, hash: &str) -> bool {
    hash_password(password) == hash
}

pub async fn login_handler(cmd: Login, db: &Db) -> Result<Value, CoreError> {
    let username = cmd.username;
    let password = cmd.password;
    db.run(move |conn| {
        let mut stmt = conn
            .prepare("SELECT id, username, password_hash, role FROM users WHERE username = ?1")
            .map_err(IntoCoreError::into_core_error)?;
        let mut rows = stmt
            .query_map([&username], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                ))
            })
            .map_err(IntoCoreError::into_core_error)?;
        let row = rows.next();
        Ok(match row {
            None => json!({ "detail": "Invalid username or password" }),
            Some(Ok((id, username, password_hash, role))) if verify_password(&password, &password_hash) => {
                json!({
                    "user": { "id": id, "username": username, "role": role },
                    "token": format!("demo-token-{}", id)
                })
            }
            _ => json!({ "detail": "Invalid username or password" }),
        })
    })
    .await
}

pub async fn register_handler(cmd: Register, db: &Db) -> Result<Value, CoreError> {
    let username = cmd.username.trim().to_string();
    let password = cmd.password;
    let role = cmd.role.trim().to_string();
    if username.is_empty() || password.is_empty() {
        return Ok(json!({ "detail": "username and password required" }));
    }
    db.run(move |conn| {
        let exists: i64 = conn
            .query_row("SELECT COUNT(1) FROM users WHERE username = ?1", [&username], |r| r.get(0))
            .map_err(IntoCoreError::into_core_error)?;
        if exists > 0 {
            return Ok(json!({ "detail": "Username already exists" }));
        }
        let id = Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO users (id, username, password_hash, role) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![id, username.clone(), hash_password(&password), role.clone()],
        )
        .map_err(IntoCoreError::into_core_error)?;
        Ok(json!({ "id": id, "username": username, "role": role }))
    })
    .await
}
