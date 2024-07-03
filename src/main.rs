use std::{net::SocketAddr, sync::Arc};

use backend::{services::deck::deck_service, AppContext};
use dashmap::DashMap;
use tonic::transport::Server as TonicServer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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
        .layer(TraceLayer::new_for_http())
        .into_inner();

    tracing::info!("Listening on {}", addr);

    TonicServer::builder()
        .layer(layer)
        .add_service(deck_svc)
        .serve(addr)
        .await
        .unwrap();
}
