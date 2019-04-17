use liblurk::protocol::protocol_message::LurkMessageBlobify;
use uuid::Uuid;

use std::sync::Arc;

#[derive(Default)]
pub struct FeedbackItems(pub Vec<FeedbackItem>);

impl FeedbackItems {
    pub fn enqueue<T>(&mut self, packet: T, target: &Uuid)
    where
        T: 'static + LurkMessageBlobify + Send + Sync,
    {
        self.0.push(FeedbackItem::new(packet, target));
    }
}

pub struct FeedbackItem {
    pub send_target: Uuid,
    pub packet: Box<LurkMessageBlobify + Send + Sync>,
}

impl FeedbackItem {
    pub fn new<T>(impl_packet: T, target: &Uuid) -> FeedbackItem
    where
        T: 'static + LurkMessageBlobify + Send + Sync,
    {
        FeedbackItem {
            send_target: target.clone(),
            packet: Box::new(impl_packet),
        }
    }
}
