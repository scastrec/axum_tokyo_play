use crate::message::Message;

pub trait MessageGateway: Send {
    fn add_message(&self, message: Message);
    fn get_messages(&self, start: isize, stop: isize) -> Vec<Message>;
}
