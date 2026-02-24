//! HTTP-транспорт для RPC (как JsonHttpRpcTransport в Python). Async, не блокирует runtime.

use async_trait::async_trait;
use serde_json::{json, Value};
use std::sync::Arc;
use urich_rs::rpc::{RpcError, RpcTransport};

/// Транспорт: POST url + base_path с телом { "method", "params" }. Для вызовов Tasks → Employees.
pub struct HttpRpcTransport {
    base_path: String,
    client: Arc<reqwest::Client>,
}

impl HttpRpcTransport {
    pub fn new(base_path: &str) -> Self {
        Self {
            base_path: base_path.trim_matches('/').to_string(),
            client: Arc::new(reqwest::Client::new()),
        }
    }
}

#[async_trait]
impl RpcTransport for HttpRpcTransport {
    async fn call(&self, url: &str, method: &str, payload: &[u8]) -> Result<Vec<u8>, RpcError> {
        let params: Value = serde_json::from_slice(payload).unwrap_or(Value::Null);
        let body = json!({ "method": method, "params": params });
        let path = format!("{}/{}/{}", url.trim_end_matches('/'), self.base_path, method);
        let res = self
            .client
            .post(&path)
            .json(&body)
            .send()
            .await
            .map_err(|e| RpcError::Transport(e.to_string()))?;
        let bytes = res
            .bytes()
            .await
            .map_err(|e| RpcError::Transport(e.to_string()))?
            .to_vec();
        Ok(bytes)
    }
}
