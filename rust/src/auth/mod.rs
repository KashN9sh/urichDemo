//! Auth: логин, регистрация. Handler как тип (DI из контейнера), без lock/resolve в модуле. Как services/auth.

mod application;

use serde::Deserialize;
use urich_rs::{Command, CommandHandler, DomainModule};

use crate::shared::Db;

#[derive(Debug, Deserialize, Command)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Command)]
pub struct Register {
    pub username: String,
    pub password: String,
    #[serde(default = "default_role")]
    pub role: String,
}

fn default_role() -> String {
    "user".to_string()
}

/// Handler с DI: Db подставляется из контейнера при register_factory. В модуле только .command_with_handler::<Login, LoginHandler>().
#[derive(Clone)]
pub struct LoginHandler {
    pub db: Db,
}

#[async_trait::async_trait]
impl CommandHandler<Login> for LoginHandler {
    async fn handle(&self, cmd: Login) -> Result<serde_json::Value, urich_rs::CoreError> {
        application::login_handler(cmd, &self.db).await
    }
}

/// Handler с DI: Db из контейнера.
#[derive(Clone)]
pub struct RegisterHandler {
    pub db: Db,
}

#[async_trait::async_trait]
impl CommandHandler<Register> for RegisterHandler {
    async fn handle(&self, cmd: Register) -> Result<serde_json::Value, urich_rs::CoreError> {
        application::register_handler(cmd, &self.db).await
    }
}

/// Модуль auth: handler как тип, резолвится из контейнера (register_factory в main). Никакого lock/resolve здесь.
pub fn auth_module() -> DomainModule {
    DomainModule::new("auth")
        .command_with_handler::<Login, LoginHandler>()
        .command_with_handler::<Register, RegisterHandler>()
}
