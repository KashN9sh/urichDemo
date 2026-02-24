//! База данных: engine и таблицы при старте (как services/shared/database).
//! SQLite для демо; один общий Connection в Mutex. Доступ через Db::run (spawn_blocking) — «сессия на запрос»:
//! каждый вызов run() получает соединение на время замыкания, не блокируя async runtime.

use std::path::Path;
use std::sync::{Arc, Mutex};
use rusqlite::Connection;

/// Обёртка над соединением SQLite для использования в нескольких потоках (сериализованный доступ).
/// Используйте .run() в async handler'ах, чтобы не блокировать runtime.
#[derive(Clone)]
pub struct Db(Arc<Mutex<Connection>>);

impl Db {
    /// Открывает БД по пути (или in-memory). Таблицы создаются при первом открытии.
    pub fn open() -> Result<Self, rusqlite::Error> {
        let path = std::env::var("DATABASE_URL").unwrap_or_else(|_| "urich_demo.db".to_string());
        let conn = if path == ":memory:" || path.is_empty() {
            Connection::open_in_memory()?
        } else {
            Connection::open(Path::new(&path))?
        };
        Self::create_tables(&conn)?;
        Ok(Self(Arc::new(Mutex::new(conn))))
    }

    /// Создаёт таблицы users, employees, tasks (как в services).
    fn create_tables(conn: &Connection) -> Result<(), rusqlite::Error> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                username TEXT NOT NULL UNIQUE,
                password_hash TEXT NOT NULL,
                role TEXT NOT NULL DEFAULT 'user'
            )",
            [],
        )?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS employees (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                role TEXT NOT NULL
            )",
            [],
        )?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                assignee_id TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'open'
            )",
            [],
        )?;
        Ok(())
    }

    /// Синхронный доступ (блокирует поток). Для совместимости и кода вне async.
    pub fn conn(&self) -> std::sync::MutexGuard<'_, Connection> {
        self.0.lock().unwrap()
    }

    /// Выполняет замыкание с соединением в отдельном потоке (spawn_blocking), не блокируя async runtime.
    pub async fn run<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&Connection) -> R + Send + 'static,
        R: Send + 'static,
    {
        let inner = Arc::clone(&self.0);
        tokio::task::spawn_blocking(move || {
            let guard = inner.lock().unwrap();
            f(&*guard)
        })
        .await
        .expect("spawn_blocking join")
    }
}
