use crate::message::Message;
use std::time::SystemTime;

use super::messages_gateway::MessageGateway;

pub async fn add_message(message_gateway: Box<dyn MessageGateway>, text: String) -> Message {
    let message = Message {
        message: text,
        timestamp: SystemTime::now(),
    };
    message_gateway.add_message(message.clone());
    return message;
}

pub async fn get_messages(
    message_gateway: Box<dyn MessageGateway>,
    start: isize,
    stop: isize,
) -> Vec<Message> {
    return message_gateway.get_messages(start, stop);
}
