use std::{str::FromStr, sync::Arc};

use anyhow::Context;
use axum::{
    extract::{Path, State},
    Json,
};
use move_core_types::language_storage::ModuleId;
use serde_json::Value;

use crate::commands::describe::module;

use super::{error::AppError, state::ServerState};

pub async fn module_description(
    State(state): State<Arc<ServerState>>,
    Path((network, module_id)): Path<(String, String)>,
) -> Result<Json<Value>, AppError> {
    let module = ModuleId::from_str(&module_id)?;
    Ok(Json(serde_json::to_value(
        module::get_or_describe(
            module,
            &mut *state.db.lock().await,
            state.sui_clients.get(&network).context("Unknown network")?,
            &state.ai,
        )
        .await?,
    )?))
}
