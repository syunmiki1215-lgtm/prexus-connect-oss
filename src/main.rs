use warp::Filter;
use std::sync::Arc;

mod oss_billing_monitor;
mod oss_audit_logger;
mod oss_crucible_env;

#[tokio::main]
async fn main() {
    let monitor = Arc::new(oss_billing_monitor::BillingMonitor::new());

    // 修正ポイント: wrap_fnでの複雑な合成を避け、.and()でシンプルかつ安全にフィルターを直列化
    let routes = oss_billing_monitor::with_rate_limiter(monitor.clone())
        .and(oss_crucible_env::crucible_routes())
        .with(oss_audit_logger::audit_log_filter());

    println!("🚀 Prexus Connect (OSS) started on port 8080...");
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}