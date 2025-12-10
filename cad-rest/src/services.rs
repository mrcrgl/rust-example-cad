use axum::{extract::Query, extract::State, routing::get};
use cad_geometry::figures::Circle;
use serde::Deserialize;

use crate::{
    AppState,
    error::{ApiError, ApiResult},
};

#[derive(Deserialize)]
struct GetParams {
    amount: Option<u32>,
}

pub async fn build_service() -> axum::Router<AppState> {
    axum::Router::new().route("/get_circle", get(handler))
}

async fn handler(
    get_params: Query<GetParams>,
    State(app_state): State<AppState>,
) -> ApiResult<axum::Json<Vec<Circle>>> {
    if get_params.amount.is_some_and(|v| v > 1000) {
        return Err(ApiError::User("amount too big. max 1000".to_string()));
    }

    let params = get_params.amount.unwrap_or(1);
    let prod_circles = app_state.producer.produce_circles(params);

    Ok(axum::Json(prod_circles))
}
