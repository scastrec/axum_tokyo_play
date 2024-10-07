use crate::message::Message;
use std::time::SystemTime;

// TODO: Use an interface, not the infra implementation
#[path = "../../infra/redis-store.rs"]
mod store;

pub async fn add_message(text: String) {
    let message = Message {
        message: text,
        timestamp: SystemTime::now(),
    };
    store::add_message(message).await;
}

pub async fn get_messages(start: isize, stop: isize) -> Vec<Message> {
    return store::get_messages(start, stop).await;
}
