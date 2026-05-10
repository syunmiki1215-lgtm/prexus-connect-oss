use chrono::Utc;
use serde::Serialize;

// 修正ポイント: 使用されていない `use warp::Filter;` を削除し警告を解消

#[derive(Serialize)]
pub struct AuditLogEntry {
    pub timestamp: String,
    pub method: String,
    pub path: String,
    pub status: u16,
    pub latency_ms: u128,
}

pub fn audit_log_filter() -> warp::log::Log<impl Fn(warp::log::Info) + Clone> {
    warp::log::custom(|info| {
        let path = info.path();
        let safe_path = if path.contains("@") { "masked_path" } else { path };

        let entry = AuditLogEntry {
            timestamp: Utc::now().to_rfc3339(),
            method: info.method().to_string(),
            path: safe_path.to_string(),
            status: info.status().as_u16(),
            latency_ms: info.elapsed().as_millis(),
        };

        if let Ok(json) = serde_json::to_string(&entry) {
            println!("[AUDIT LOG] {}", json);
        }
    })
}