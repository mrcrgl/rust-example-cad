use tracing_subscriber::{
    fmt::format::FmtSpan, layer::SubscriberExt as _, util::SubscriberInitExt as _,
};
use std::sync::Arc;
use cad_geometry::application::figure_producer::GeometricFigureProducer;

use crate::services::build_service;

mod services;

#[derive(Clone)]
pub struct AppState {
   producer: Arc<GeometricFigureProducer>,
   is_online: bool,
}

#[tokio::main]
async fn main() {

    let app_state = AppState {
        producer: Arc::new(GeometricFigureProducer::new()),
        is_online: false
    };

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_span_events(FmtSpan::NONE)
                .compact()
                .without_time()
                .with_target(false),
        )
        .init();

    tracing::info!("Im Alive");
    
    let routes = axum::Router::new()
        .nest("/api", build_service().await)
        .with_state(app_state);
        
    let listener = tokio::net::TcpListener::bind("127.0.0.1:1337").await.unwrap();
    axum::serve(listener, routes).await.unwrap();



}

