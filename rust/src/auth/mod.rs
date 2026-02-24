//! Auth: логин, регистрация (DI: Db из контейнера). Как services/auth.

mod application;

use std::sync::{Arc, Mutex};
use urich_rs::{Application, Container, CoreError};

/// Регистрирует маршруты /auth/login, /auth/register. Обработчики получают Db из app.container().
pub fn register_auth(app: &mut Application) -> Result<(), CoreError> {
    app.register_route(
        "POST",
        "auth/login",
        Some(serde_json::json!({
            "type": "object",
            "required": ["username", "password"],
            "properties": { "username": {"type": "string"}, "password": {"type": "string"} }
        })),
        Box::new(|body: serde_json::Value, container: Arc<Mutex<Container>>| {
            Box::pin(async move {
                let db = {
                    let mut guard = container.lock().unwrap();
                    guard.resolve::<crate::shared::Db>().map_err(|e| CoreError::Validation(e.to_string()))?.clone()
                };
                application::login_handler(body, &db).await
            })
        }),
        Some("Auth"),
    )?;
    app.register_route(
        "POST",
        "auth/register",
        Some(serde_json::json!({
            "type": "object",
            "required": ["username", "password"],
            "properties": {
                "username": {"type": "string"},
                "password": {"type": "string"},
                "role": {"type": "string", "default": "user"}
            }
        })),
        Box::new(|body: serde_json::Value, container: Arc<Mutex<Container>>| {
            Box::pin(async move {
                let db = {
                    let mut guard = container.lock().unwrap();
                    guard.resolve::<crate::shared::Db>().map_err(|e| CoreError::Validation(e.to_string()))?.clone()
                };
                application::register_handler(body, &db).await
            })
        }),
        Some("Auth"),
    )?;
    Ok(())
}
