//! Пример middleware для urich-rs (как JWT в Python services).

use serde_json::json;
use urich_rs::{CoreResponse, RequestContext};

/// Логирует метод и путь, передаёт управление дальше (возвращает None).
pub fn log_request(ctx: &RequestContext) -> Option<CoreResponse> {
    println!("  {} {}", ctx.method, ctx.path);
    None
}

/// Демо-проверка: для путей не из public_path_prefixes требует заголовок X-Demo-Key; иначе 401.
/// Аналог JWT middleware в Python (здесь без JWT — только пример цепочки).
pub fn require_demo_key(
    public_path_prefixes: &'static [&'static str],
) -> impl Fn(&RequestContext) -> Option<CoreResponse> + Send + Sync + 'static {
    move |ctx: &RequestContext| {
        let path = ctx.path.trim_start_matches('/');
        if public_path_prefixes
            .iter()
            .any(|p| path == *p || path.starts_with(&format!("{}/", p)))
        {
            return None;
        }
        let key = ctx
            .headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case("x-demo-key"))
            .map(|(_, v)| v.as_str());
        if key.is_some() && key != Some("") {
            return None;
        }
        let body =
            serde_json::to_vec(&json!({ "detail": "Missing or invalid X-Demo-Key header" }))
                .unwrap_or_default();
        Some(CoreResponse {
            status_code: 401,
            body,
        })
    }
}
