use axum::{response::Html, routing::get, Router};
use tracing::{dispatcher, metadata::LevelFilter, Dispatch};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

#[tokio::main]
async fn main() {
    // Setup tracing subscriber with logfmt logging, and ability to set log level via RUST_LOG
    let subscriber = Registry::default().with(tracing_logfmt::layer()).with(
        EnvFilter::builder()
            .with_default_directive(LevelFilter::INFO.into())
            .from_env_lossy(),
    );

    dispatcher::set_global_default(Dispatch::new(subscriber))
        .expect("failed setting up tracing dispatcher");

    // Setup server and handlers
    let app = Router::new().route("/", get(hello));
    let addr = (
        [0, 0, 0, 0],
        std::env::var("PORT")
            .map(|p| p.parse().unwrap())
            .unwrap_or(8080),
    )
        .into();

    tracing::info!("starting the server, listening to {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("failed starting the server");

    tracing::info!("server stopped");
}

async fn hello() -> Html<&'static str> {
    Html("<p>Hello there!!</p>")
}
