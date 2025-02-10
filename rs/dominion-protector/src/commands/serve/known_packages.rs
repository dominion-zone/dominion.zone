use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use serde_json::{json, Value};
use tokio::sync::Mutex;

use super::{error::AppError, state::ServerState};

pub async fn known_packages(
    State(state): State<Arc<ServerState>>,
    Path(network): Path<String>,
) -> Result<Json<Value>, AppError> {
    let packages = state
        .db
        .lock()
        .await
        .query(
            "SELECT package_id FROM module_descriptions
        WHERE network = $1
        GROUP BY package_id",
            &[&network],
        )
        .await?;
    let packages = packages
        .iter()
        .map(|row| {
            let package_id: String = row.get(0);
            package_id
        })
        .collect::<Vec<String>>();
    Ok(Json(json!(packages)))
}
