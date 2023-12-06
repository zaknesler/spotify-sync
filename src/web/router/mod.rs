use super::{middleware::guest, view::ConnectTemplate};
use crate::{api::client2, context::AppContext};
use axum::{middleware, response::IntoResponse, routing::get, Router};
use tower_cookies::{
    cookie::{
        time::{Duration, OffsetDateTime},
        CookieBuilder,
    },
    Cookies,
};

mod connect;
mod user;
mod watcher;

pub const JWT_COOKIE: &str = "modulate_jwt";
pub const CSRF_COOKIE: &str = "modulate_csrf";

pub fn router(ctx: AppContext) -> Router {
    Router::new()
        .route("/", get(root))
        .route_layer(middleware::from_fn(guest::middleware))
        .with_state(ctx.clone())
        .merge(connect::router(ctx.clone()))
        .merge(watcher::router(ctx.clone()))
        .merge(user::router(ctx))
}

async fn root(cookies: Cookies) -> crate::Result<impl IntoResponse> {
    let (url, csrf) = client2::Client::new()?.new_authorize_url();

    // Set CSRF cookie to verify once user is redirected back
    cookies.add(
        CookieBuilder::new(CSRF_COOKIE, csrf.secret().clone())
            .path("/")
            .expires(OffsetDateTime::now_utc().checked_add(Duration::hours(1)))
            .http_only(true)
            .same_site(tower_cookies::cookie::SameSite::Strict)
            .build(),
    );

    Ok(ConnectTemplate {
        url: url.to_string(),
    })
}
