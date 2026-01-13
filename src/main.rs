use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    //create an app with a single route
    let app = Router::new().route("/", get(|| async { "hello world" }));

    //create a listener to listen to the port for which the router is configured
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    //serve for that port
    axum::serve(listener, app).await.unwrap();
}
