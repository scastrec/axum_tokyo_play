#[path = "./model/message.rs"]
mod message;
#[path = "./domain/messages/mod.rs"]
mod messages;

#[path = "./infra/messages_repository.rs"]
mod messages_repository;

use axum::extract::Json;
use axum::response::IntoResponse;
use axum::{routing::get, Router};
use axum_macros::debug_handler;
use messages::messages::{add_message, get_messages};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct AddMessage {
    message: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/healthcheck", get(|| async { "I'm alive!" }))
        .route(
            "/messages",
            get(get_messages_handler).post(add_message_handler),
        );

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[debug_handler]
async fn get_messages_handler() -> impl IntoResponse {
    // TODO parse params for start/stop
    let message_gateway = messages_repository::MessageRepository::new();
    let messages = get_messages(Box::new(message_gateway), 0, 10).await;
    return Json(messages);
}

async fn add_message_handler(Json(payload): Json<AddMessage>) {
    /*if payload.message.is_empty() {
        Ok((StatusCode::BAD_REQUEST, Json(ErrorMessage{error: "message"})).into_response())
    }*/
    let message_gateway = messages_repository::MessageRepository::new();
    let _message = add_message(Box::new(message_gateway), payload.message).await;
    //Ok(Json(message))
}
