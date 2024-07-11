use std::{net::SocketAddr, sync::Arc};

use axum::{extract::Request, http::header::CONTENT_TYPE, routing::get, Router};
use backend::{services::deck::deck_service, AppContext};
use dashmap::DashMap;
use tonic::transport::Server as TonicServer;
use tower::{make::Shared, steer::Steer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

async fn web_root() -> &'static str {
    tracing::info!("Got a REST request");

    "Hello, World!"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let ctx = AppContext {
        game_states: Arc::new(DashMap::new()),
    };

    let addr = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], 8080));

    let deck_svc = deck_service(ctx);

    let layer = tower::ServiceBuilder::new()
        .layer(TraceLayer::new_for_grpc())
        .into_inner();

    let grpc = TonicServer::builder()
        .layer(layer)
        .add_service(deck_svc)
        .into_router();

    let rest = Router::new().route("/", get(web_root));
    let service = Steer::new(vec![rest, grpc], |req: &Request, _services: &[_]| {
        if req
            .headers()
            .get(CONTENT_TYPE)
            .map(|content_type| content_type.as_bytes())
            .filter(|content_type| content_type.starts_with(b"application/grpc"))
            .is_some()
        {
            1
        } else {
            0
        }
    });

    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::debug!("listening on {}", addr);
    axum::serve(listener, Shared::new(service)).await.unwrap();
}
