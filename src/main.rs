use axum::{
    Router,
    routing::{get, post},
};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing_subscriber::{EnvFilter, fmt};

mod adapters;
mod domain;
mod infrastructure;
mod usecase;

use rust_api::Config;

use adapters::db::user_repo_sqlx::SqlxUserRepo;
use adapters::http::user_handlers::HttpState;
use adapters::http::user_handlers::*;
use usecase::user_service::UserService;

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    fmt()
        .with_env_filter(filter)
        .with_target(false)
        .compact()
        .init();
    tracing::info!("tracing initialized");
}

#[tokio::main]
async fn main() {
    init_tracing();

    let config = Config::from_env().expect("failed to load config");
    let pool = infrastructure::db::new_pool(&config.db_url).await;

    // Infrastructure -> Adapter(DB) -> Usecase -> Adapter(HTTP)
    let repo = SqlxUserRepo::new(pool);
    let service = UserService::new(repo);
    let http_state = HttpState {
        user_service: service.into(),
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any);

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .route(
            "/users",
            post(create_user::<SqlxUserRepo>).get(list_users::<SqlxUserRepo>),
        )
        .route(
            "/users/:id",
            get(get_user::<SqlxUserRepo>)
                .put(update_user::<SqlxUserRepo>)
                .delete(delete_user::<SqlxUserRepo>),
        )
        .with_state(http_state)
        .layer(cors)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().include_headers(false))
                .on_response(
                    DefaultOnResponse::new()
                        .include_headers(true)
                        .latency_unit(tower_http::LatencyUnit::Millis),
                ),
        );

    let addr = "0.0.0.0:3000";
    println!("Listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
