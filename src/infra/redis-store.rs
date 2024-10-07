extern crate redis;
// TODO: improve error management

use crate::message::Message;

static MESSAGE_KEY: &'static str = "MESSAGES";

pub async fn get_messages(start: isize, stop: isize) -> Vec<Message> {
    let mut connection = get_connection();
    let items: Vec<String> = redis::cmd("LRANGE")
        .arg(MESSAGE_KEY)
        .arg(start)
        .arg(stop)
        .query(&mut connection)
        .expect("failed to get messages");
    let messages = items
        .iter()
        .map(|item| serde_json::from_str(item).expect("deserialize messages error"))
        .collect();
    return messages;
}
pub async fn add_message(message: Message) {
    let mut connection = get_connection();
    let to_store = serde_json::to_string(&message).expect("Cant't serialize message");
    println!("Storing message: {}", to_store);
    let _ = redis::cmd("LPUSH")
        .arg(MESSAGE_KEY)
        .arg(to_store)
        .exec(&mut connection)
        .expect("failed to store message");
}

fn get_connection() -> redis::Connection {
    let connection = redis::Client::open("redis://127.0.0.1:6379/")
        .expect("Invalid connection URL")
        .get_connection()
        .expect("failed to connect to Redis");

    return connection;
}
