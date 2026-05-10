use warp::{Filter, Rejection, Reply, http::StatusCode};
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;

pub fn crucible_routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    // 既存のモック用ルート（GET /api/mock/{data_type}）
    let mock_route = warp::path!("api" / "mock" / String)
        .and(warp::get())
        .and_then(handle_mock_data);

    // 新規追加：Crucibleカオスエンジニアリング用ルート（POST /api/crucible/test）
    let crucible_test_route = warp::path!("api" / "crucible" / "test")
        .and(warp::post())
        .and(warp::header::optional::<String>("x-crucible-scenario"))
        .and_then(handle_crucible_test);

    // 2つのルートを合成
    mock_route.or(crucible_test_route)
}

// 既存のモック処理（戻り値を warp::reply::Response に統一）
async fn handle_mock_data(data_type: String) -> Result<warp::reply::Response, Rejection> {
    match data_type.as_str() {
        "generic" => {
            Ok(warp::reply::json(&json!({
                "id": "test-123",
                "status": "success",
                "timestamp": chrono::Utc::now().to_rfc3339()
            })).into_response())
        },
        "medical" => {
            Ok(warp::reply::with_status(
                warp::reply::json(&json!({ "error": "Upgrade to Premium for Medical Mock" })),
                StatusCode::PAYMENT_REQUIRED
            ).into_response())
        },
        _ => Err(warp::reject::not_found())
    }
}

// 新規追加：Crucibleの異常系シミュレーション処理
async fn handle_crucible_test(scenario: Option<String>) -> Result<warp::reply::Response, Rejection> {
    if let Some(scen) = scenario {
        match scen.as_str() {
            "timeout" => {
                // 5秒間の通信スパイク（遅延）の再現
                sleep(Duration::from_secs(5)).await;
                Ok(warp::reply::json(&json!({
                    "status": "success",
                    "message": "Response delayed by 5 seconds (timeout scenario)"
                })).into_response())
            },
            "conflict" => {
                // データの書き換え競合（409 Conflict）の再現
                Ok(warp::reply::with_status(
                    warp::reply::json(&json!({ "error": "Data conflict detected" })),
                    StatusCode::CONFLICT
                ).into_response())
            },
            "data_loss" => {
                // DBの物理障害（500 Internal Server Error）の再現
                Ok(warp::reply::with_status(
                    warp::reply::json(&json!({ "error": "Internal Database Error" })),
                    StatusCode::INTERNAL_SERVER_ERROR
                ).into_response())
            },
            _ => {
                // 未知のシナリオ
                Ok(warp::reply::with_status(
                    warp::reply::json(&json!({ "error": "Unknown scenario" })),
                    StatusCode::BAD_REQUEST
                ).into_response())
            }
        }
    } else {
        // ヘッダーが指定されていない場合の正常なレスポンス
        Ok(warp::reply::json(&json!({
            "status": "success",
            "message": "Crucible test endpoint connected cleanly."
        })).into_response())
    }
}