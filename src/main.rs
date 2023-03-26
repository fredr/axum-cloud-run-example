use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello));
    let addr = ([0, 0, 0, 0], 8080).into();

    println!("starting the server");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("failed starting the server");

    println!("server stopped");
}

async fn hello() -> Html<&'static str> {
    Html("<p>Hello there!!</p>")
}
