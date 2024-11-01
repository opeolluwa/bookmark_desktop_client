use super::handlers::{
    root::{base, health_check_handler},
    // user_information::{login, register_user},
    // vault::some_protected_resources,
};
use axum::routing::{get, Router};

pub fn router() -> Router {
    Router::new()
        .route("/", get(base))
        .route("/health", get(health_check_handler))
    // .nest("/users", user_route())
    // .nest("/api/v1", protected_routes())
}

// pub fn user_route() -> Router<AppState> {
//     Router::new()
//         .route("/register", post(register_user))
//         .route("/login", post(login))
// }

// pub fn protected_routes() -> Router<AppState> {
//     Router::new().route("/protected", get(some_protected_resources))
// }
