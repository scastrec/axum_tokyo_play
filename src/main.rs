#[path = "./model/message.rs"]
mod message;
#[path = "./domain/messages/messages.rs"]
mod messages;

use axum::http::StatusCode;
use axum::{extract, routing::get, Json, Router};
use messages::{add_message, get_messages};
use serde::Deserialize;

#[derive(Deserialize)]
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

async fn get_messages_handler() -> Json<Vec<message::Message>> {
    // TODO parse params for start/stop
    return Json(get_messages(0, 10).await);
}

async fn add_message_handler(
    extract::Json(payload): extract::Json<AddMessage>,
) -> Result<(), StatusCode> {
    if payload.message.is_empty() {
        Err(StatusCode::BAD_REQUEST)?;
    }
    add_message(payload.message).await;
    Ok(())
}
