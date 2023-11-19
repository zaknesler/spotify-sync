use super::view::AuthTemplate;
use crate::{context::AppContext, util::client::create_oauth_client};
use axum::{extract::State, response::IntoResponse, routing::get, Router};
use std::sync::Arc;

mod auth;
mod user;
mod watcher;

pub const JWT_COOKIE: &str = "spotify_sync_jwt";

pub fn router(ctx: Arc<AppContext>) -> Router {
    Router::new()
        .route("/", get(root))
        .with_state(ctx.clone())
        .merge(auth::router(ctx.clone()))
        .merge(watcher::router(ctx.clone()))
        .merge(user::router(ctx))
}

async fn root(State(ctx): State<Arc<AppContext>>) -> crate::Result<impl IntoResponse> {
    let client = create_oauth_client(&ctx.config);
    let url = client.get_authorize_url(true)?;

    Ok(AuthTemplate { url })
}
