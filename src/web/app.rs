use std::sync::Arc;

use anyhow::Result;
use axum::{
    Router,
    extract::Extension,
    routing::post
};
use tower_http::{
    LatencyUnit,
    trace::{
        TraceLayer,
        DefaultMakeSpan,
        DefaultOnRequest,
        DefaultOnResponse
    },
};
use tower::ServiceBuilder;
use tracing::{
    Level
};

use crate::config::{Config};
use crate::web::handler::{
    post_webhooks_github
};

pub struct AppState {
    pub config: Config
}

fn new_router() -> Router {
    Router::new()
        .route("/webhooks/github", post(post_webhooks_github))
}

pub async fn start_server(cfg: Config) -> Result<()> {
    let bind_addr = cfg.host.clone() + ":" + &cfg.port.to_string();
    let state = Arc::new(AppState {
        config: cfg
    });

    let r = new_router()
        .layer(
            ServiceBuilder::new()
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(
                            DefaultMakeSpan::new().include_headers(true)
                        )
                        .on_request(
                            DefaultOnRequest::new().level(Level::INFO)
                        )
                        .on_response(
                            DefaultOnResponse::new()
                                .level(Level::INFO)
                                .latency_unit(LatencyUnit::Micros)
                        )
                )
                .layer(Extension(state))
        );

    axum::Server::bind(&bind_addr.parse()?)
        .serve(r.into_make_service())
        .await?;

    Ok(())
}
