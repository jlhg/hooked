use std::sync::Arc;
use axum::extract::{Extension, Json};
use axum::http::StatusCode;
use http::HeaderMap;
use serde_json::{Value};
use crate::crypto::verify_signature;
use tracing::{error};

use crate::web::app::AppState;
use crate::web::object::*;

pub async fn post_webhooks_github(
    Extension(state): Extension<Arc<AppState>>,
    headers: HeaderMap,
    body: String,
    // Json(payload): Json<serde_json::Value>
) -> (StatusCode, Json<Value>) {
    match headers.get("X-Hub-Signature-256") {
        Some(v) => {
            let signature = v.to_str().unwrap_or("");

            match verify_signature(
                state.config.github_webhook_secret.as_bytes(),
                body.as_bytes(),
                signature.strip_prefix("sha256=").unwrap_or_default().as_bytes(),
            ) {
                Ok(matched) => {
                    if !matched {
                        return render_forbidden("invalid signature");
                    }
                }
                Err(e) => {
                    error!("{}", e);
                    return render_internal_server_error("can't verify signature");
                }
            }
        }
        None => return render_forbidden("missing signature")
    }

    match headers.get("X-GitHub-Event") {
        Some(v) => {
            let event = v.to_str().unwrap_or("");
            match event {
                "ping" => {
                    // TODO
                    return render_success(StatusCode::OK, "ping")
                }
                "push" => {
                    // TODO
                    return render_success(StatusCode::OK, "push")
                }
                _ => return render_success(StatusCode::OK, "unhandled event")
            }
        }
        None => return render_success(StatusCode::OK, "no event")
    }
}
