//! Общее для сервисов (как services/shared).

pub mod database;
pub mod http_transport;
pub mod jwt_middleware;
pub use database::Db;
pub use http_transport::HttpRpcTransport;
pub use jwt_middleware::{jwt_validation_middleware, require_demo_key};
