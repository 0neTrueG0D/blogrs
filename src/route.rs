use std::sync::Arc;

use axum::{
    middleware,
    routing::{delete, get, patch, post},
    Router,
};

use crate::{
    guard::auth_guard_middleware,
    handlers::{
        auth::{
            current_user_handler, login_user_handler, logout_user_handler, register_user_handler,
        },
        post::{
            create_post_handler, delete_post_handler, fetch_post_detail_handler,
            fetch_post_handler, update_post_handler,
        },
    },
    AppState,
};

pub fn api_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/post", get(fetch_post_handler))
        .route("/post/:slug", get(fetch_post_detail_handler))
        .route(
            "/post/create",
            post(create_post_handler).route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                auth_guard_middleware,
            )),
        )
        .route(
            "/post/update/:slug",
            patch(update_post_handler).route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                auth_guard_middleware,
            )),
        )
        .route(
            "/post/delete/:slug",
            delete(delete_post_handler).route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                auth_guard_middleware,
            )),
        )
        .route("/auth/register", post(register_user_handler))
        .route("/auth/login", post(login_user_handler))
        .route(
            "/auth/logout",
            post(logout_user_handler).route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                auth_guard_middleware,
            )),
        )
        .route(
            "/auth/current_user",
            get(current_user_handler).route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                auth_guard_middleware,
            )),
        )
        .with_state(app_state)
}
