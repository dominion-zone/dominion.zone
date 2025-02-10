use std::sync::Arc;

use axum::{extract::{Path, State}, Json};
use serde_json::{json, Value};

use super::state::ServerState;

async fn package_description(
    State(state): State<Arc<ServerState>>,
    Path((network, id)): Path<(String, String)>,
) -> Json<Value> {
    Json(json!({ "data": 42 }))
}
