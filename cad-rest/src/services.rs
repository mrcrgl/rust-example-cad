use axum::{response::IntoResponse,extract::State, response::Response, extract::Query, routing::get};
use cad_geometry::figures::Circle;
use serde::Deserialize;

use crate::AppState;

#[derive(Deserialize)]
struct GetParams {
    amount: Option<u32>,
}


pub async fn build_service() -> axum::Router<AppState> {

    axum::Router::new()
        .route("/get_circle", get(handler))


}

async fn handler(get_params: Query<GetParams>, State(app_state): State<AppState>) -> axum::Json<Vec<Circle>> {
    let params = get_params.amount.unwrap_or_else(||1);
    let prod_circles = app_state.producer.produce_circles(params);

    axum::Json(prod_circles)

}
