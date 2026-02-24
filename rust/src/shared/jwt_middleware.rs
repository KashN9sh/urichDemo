//! JWT-проверка для сервисов employees и tasks (как services/shared/jwt_middleware.py).
//! Для путей не из public_path_prefixes требует заголовок Authorization: Bearer <token>; иначе 401.

use std::future::Future;
use std::pin::Pin;
use serde_json::json;
use urich_rs::{CoreResponse, RequestContext};

/// Возвращает middleware: для путей не из public_path_prefixes требует Bearer; иначе 401.
/// В демо без реального JWT можно использовать X-Demo-Key (см. require_demo_key ниже).
pub fn jwt_validation_middleware(
    public_path_prefixes: &'static [&'static str],
) -> impl Fn(&RequestContext) -> Pin<Box<dyn Future<Output = Option<CoreResponse>> + Send>> + Send + Sync + 'static {
    move |ctx: &RequestContext| {
        let path = ctx.path.trim_start_matches('/');
        if public_path_prefixes
            .iter()
            .any(|p| path == *p || path.starts_with(&format!("{}/", p)))
        {
            return Box::pin(std::future::ready(None));
        }
        let auth = ctx
            .headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case("authorization"))
            .map(|(_, v)| v.as_str())
            .unwrap_or("");
        let token = auth
            .trim()
            .strip_prefix("Bearer ")
            .or_else(|| auth.trim().strip_prefix("bearer "))
            .map(|s| s.trim())
            .unwrap_or("");
        if token.is_empty() {
            let body = serde_json::to_vec(&json!({ "detail": "Missing Authorization: Bearer <token>" }))
                .unwrap_or_default();
            return Box::pin(std::future::ready(Some(CoreResponse {
                status_code: 401,
                body,
            })));
        }
        Box::pin(std::future::ready(None))
    }
}

/// Демо-вариант: требует заголовок X-Demo-Key (без реального JWT). Как в минимальном примере.
pub fn require_demo_key(
    public_path_prefixes: &'static [&'static str],
) -> impl Fn(&RequestContext) -> Pin<Box<dyn Future<Output = Option<CoreResponse>> + Send>> + Send + Sync + 'static {
    move |ctx: &RequestContext| {
        let path = ctx.path.trim_start_matches('/');
        if public_path_prefixes
            .iter()
            .any(|p| path == *p || path.starts_with(&format!("{}/", p)))
        {
            return Box::pin(std::future::ready(None));
        }
        let key = ctx
            .headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case("x-demo-key"))
            .map(|(_, v)| v.as_str());
        if key.is_some() && key != Some("") {
            return Box::pin(std::future::ready(None));
        }
        let body = serde_json::to_vec(&json!({ "detail": "Missing or invalid X-Demo-Key header" }))
            .unwrap_or_default();
        Box::pin(std::future::ready(Some(CoreResponse {
            status_code: 401,
            body,
        })))
    }
}
