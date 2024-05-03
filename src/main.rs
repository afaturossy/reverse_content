use axum::Router;
use axum::routing::get;
use reqwest::Client;
use crate::reverse::{get_image_komikindo_link, test};

mod reverse;

#[tokio::main]
async fn main() {

   server().await
}

async fn server(){
    let client  =Client::new();

    let port = dotenvy::var("PORT").unwrap_or("3000".to_string());
    let app = Router::new()
        .route("/k_link/:url", get(get_image_komikindo_link)) // komikindo.link
        .with_state(client);

    // run our app with hyper, listening globally on port 3000
    let addr = format!("0.0.0.0:{}", port);
    println!("server run -> {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

