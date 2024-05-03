use axum::body::Body;
use axum::extract::State;
use axum::http::{HeaderMap, HeaderName, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use reqwest::Client;

pub async fn get_image_linksaya(State(client): State<Client>) -> Response {
    let url = "https://linksaya.com/images/m/milf-hunting-in-another-world/chapter-01/3.webp";
    let reqwest_response = match client.get(url)
        .header("Referer", "https://komikindo.link")
        .send().await {
        Ok(res) => res,
        Err(_) => {
            return (StatusCode::BAD_REQUEST, Body::empty()).into_response();
        }
    };

    let response_builder = Response::builder()
        .status(reqwest_response.status().as_u16());

    let mut header = HeaderMap::with_capacity(reqwest_response.headers().len());
    header.extend(reqwest_response.headers().into_iter().map(|(name, value)| {
        let name = HeaderName::from_bytes(name.as_ref()).unwrap();
        let value = HeaderValue::from_bytes(value.as_ref()).unwrap();
        (name, value)
    }));

    response_builder
        .body(Body::from_stream(reqwest_response.bytes_stream()))
        .unwrap()
}

#[tokio::test]
async fn get_image_test() {
    let url = "https://linksaya.com/images/m/milf-hunting-in-another-world/chapter-01/3.webp";
    let client = Client::new();

    let resp = client.get(url)
        .header("Referer", "https://komikindo.link")
        .send().await;

    println!("{:?}", resp.unwrap().status());
}