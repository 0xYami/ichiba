mod articles;
mod health;
mod users;

use axum::Router;
use std::sync::Arc;

use crate::AppState;

pub use articles::Article;
pub use users::User;

pub fn router(pool: Arc<AppState>) -> Router {
    Router::new()
        .merge(health::router())
        .merge(users::router(pool.clone()))
        .merge(articles::router(pool))
}
