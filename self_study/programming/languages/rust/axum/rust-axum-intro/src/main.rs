#![allow(unused)]
use std::net::SocketAddr;
use std::{thread, time};
use std::future::IntoFuture;

use axum;
use axum::extract::{Path, Query};
use axum::http::{StatusCode, Uri, header::{self, HeaderMap, HeaderName}};
use axum::response::{Html, IntoResponse, Json};
use axum::routing::{get, get_service};
use axum::Router;

use serde_json::{Value, json};
use serde::Deserialize;

use tokio;
use tokio::task;

use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127,0,0,1], 8080));

    println!("--> LISTENING on {addr}\n");

    let routes = Router::new()
        .merge(get_routes_dynamic())
        .fallback_service(get_routes_static());

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    let axum_worker = tokio::task::spawn(
        async move {
            axum::serve(
                listener,
                routes,
            ).await
        }
    );

    let some_other_worker = tokio::task::spawn(
        some_worker().into_future()
    );

    let (axum_res, some_worker_res) = tokio::join!(axum_worker, some_other_worker);

    axum_res.unwrap();
    some_worker_res.unwrap();
}


async fn some_worker() {
    let duration = time::Duration::from_millis(10000);
    let mut n: u32 = 0;
    loop {
        n += 1;
        println!("some_worker loop count: {}", n);
        thread::sleep(duration);
    }
}

fn get_routes_dynamic() -> Router {
    Router::new()
        .route(
            // http://localhost:8080/handler_query_param?name=Bob
            "/handler_query_param",
            get(handler_query_param)
        ).route(
            "/handler_path_param/:name",
            get(handler_path_param)
        ).route(
            // http://localhost:8080/handler_path_param/Bob
            "/handler_plain_text",
            get(handler_plain_text)
        ).route(
            "/handler_json",
            get(handler_json)
        ).route(
            "/handler_json_vec",
            get(handler_json_vec)
        ).route(
            "/handler_html",
            get(handler_html)
        ).route(
            "/handler_status",
            get(handler_status)
        )

}

fn get_routes_static() -> Router {
    Router::new()
        .nest_service(
            // http://localhost:8080 -> loads index.html from document_root
            "/",
            get_service(ServeDir::new("./document_root"))
        )
}

fn print_handler(handler: &str) {
    println!("--> HANDLER - {handler}\n");
}

#[derive(Debug, Deserialize)]
struct NameParams {
    name: Option<String>
}

async fn handler_query_param(Query(params): Query<NameParams>) -> impl IntoResponse {
    let name = params.name.as_deref().unwrap_or("World");
    print_handler("handler_query_param");
    Html(format!("<h1>Hello, <strong>{name}</strong> from handler!</h1>"))
}

async fn handler_path_param(Path(name): Path<String>) -> impl IntoResponse {
    print_handler("handler_path_param");
    Html(format!("<h1>Hello, <strong>{name}</strong> from handler!</h1>"))
}

async fn handler_plain_text(uri: Uri) -> String {
    print_handler("handler_plain_text");
    format!("Hi from {}", uri.path())
}

async fn handler_json_vec() -> Json<Vec<String>> {
    print_handler("handler_json_vec");
    Json(vec!["foo".to_owned(), "bar".to_owned()])
}

async fn handler_json() -> Json<Value> {
    print_handler("handler_json");
    Json(json!({ "data": 42 }))
}

async fn handler_html() -> Html<&'static str> {
    print_handler("handler_html");
    Html("<p>Hello, World!</p>")
}

async fn handler_status() -> StatusCode {
    StatusCode::OK
}
