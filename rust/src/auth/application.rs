//! Обработчики login и register: Db из контейнера (DI, как services/auth).

use serde_json::{json, Value};
use urich_rs::CoreError;
use uuid::Uuid;

use crate::shared::Db;

fn hash_password(password: &str) -> String {
    format!("hash:{}", password.len())
}

fn verify_password(password: &str, hash: &str) -> bool {
    hash_password(password) == hash
}

pub async fn login_handler(body: Value, db: &Db) -> Result<Value, CoreError> {
    let username = body.get("username").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let password = body.get("password").and_then(|v| v.as_str()).unwrap_or("").to_string();
    db.run(move |conn| {
        let mut stmt = conn
            .prepare("SELECT id, username, password_hash, role FROM users WHERE username = ?1")
            .map_err(|e| CoreError::Validation(e.to_string()))?;
        let mut rows = stmt
            .query_map([&username], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                ))
            })
            .map_err(|e| CoreError::Validation(e.to_string()))?;
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

pub async fn register_handler(body: Value, db: &Db) -> Result<Value, CoreError> {
    let username = body
        .get("username")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_string();
    let password = body.get("password").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let role = body
        .get("role")
        .and_then(|v| v.as_str())
        .unwrap_or("user")
        .trim()
        .to_string();
    if username.is_empty() || password.is_empty() {
        return Ok(json!({ "detail": "username and password required" }));
    }
    db.run(move |conn| {
        let exists: i64 = conn
            .query_row("SELECT COUNT(1) FROM users WHERE username = ?1", [&username], |r| r.get(0))
            .map_err(|e| CoreError::Validation(e.to_string()))?;
        if exists > 0 {
            return Ok(json!({ "detail": "Username already exists" }));
        }
        let id = Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO users (id, username, password_hash, role) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![id, username.clone(), hash_password(&password), role.clone()],
        )
        .map_err(|e| CoreError::Validation(e.to_string()))?;
        Ok(json!({ "id": id, "username": username, "role": role }))
    })
    .await
}
