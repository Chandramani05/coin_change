use axum::extract::Path;
use axum::{routing::get, Json, Router};
use serde_json::json;
use coin_change::greed_coin_change;

async fn root() -> Json<serde_json::Value> {
    Json(json!({"message": "Hello World"}))
}

async fn change(Path((dollars, cents)): Path<(u32, u32)>) -> impl axum::response::IntoResponse {
    let amount =  dollars * 100 + cents;
    let change = greed_coin_change(amount);
    let json = json!({
        "dollars": dollars,
        "cents": cents,
        "change": change
    });

    Json(json)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/change/:dollars/:cents", get(change));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}



