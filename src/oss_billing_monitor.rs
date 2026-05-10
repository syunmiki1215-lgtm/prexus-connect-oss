use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::{Filter, Rejection};

pub struct BillingMonitor {
    pub usage_store: Arc<RwLock<HashMap<String, u64>>>,
}

impl BillingMonitor {
    pub fn new() -> Self {
        Self { usage_store: Arc::new(RwLock::new(HashMap::new())) }
    }
}

pub fn with_rate_limiter(monitor: Arc<BillingMonitor>) -> impl Filter<Extract = (), Error = Rejection> + Clone {
    warp::header::optional::<String>("x-tenant-id")
        .and_then(move |tenant_id: Option<String>| {
            let m = monitor.clone();
            async move {
                let id = tenant_id.unwrap_or_else(|| "default".to_string());
                let mut store = m.usage_store.write().await;
                let count = store.entry(id).or_insert(0);
                *count += 1;
                
                if *count > 500_000 {
                    return Err(warp::reject::not_found());
                }
                Ok::<(), Rejection>(())
            }
        })
        .untuple_one() // 修正ポイント: 存在しない .unit() から、タプルを解除する .untuple_one() へ変更
}