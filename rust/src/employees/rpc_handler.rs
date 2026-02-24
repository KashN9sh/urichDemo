//! RPC-обработчик get_employee — читает из БД (как services/employees). Db из контейнера на каждый запрос.

use async_trait::async_trait;
use serde_json::{from_slice, to_vec, Value};
use std::sync::{Arc, Mutex};
use urich_rs::rpc::{RpcError, RpcServerHandler};
use urich_rs::Container;

use super::GetEmployee;

pub struct EmployeesRpcHandler;

#[async_trait]
impl RpcServerHandler for EmployeesRpcHandler {
    async fn handle(
        &self,
        method: &str,
        payload: &[u8],
        container: Arc<Mutex<Container>>,
    ) -> Result<Vec<u8>, RpcError> {
        if method != "get_employee" {
            return Err(RpcError::Server {
                code: "NOT_FOUND".to_string(),
                message: format!("unknown method {:?}", method),
            });
        }
        let params: Value = from_slice(payload).unwrap_or(Value::Null);
        let employee_id = params
            .get("employee_id")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let db = {
            let mut guard = container.lock().unwrap();
            guard.resolve::<crate::shared::Db>().map_err(|e| RpcError::Transport(e.to_string()))?.clone()
        };
        let out = db.run(|conn| super::get_employee(GetEmployee { employee_id }, conn)).await.unwrap_or(Value::Null);
        to_vec(&out).map_err(|e| RpcError::Transport(e.to_string()))
    }
}
