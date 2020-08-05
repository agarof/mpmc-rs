use std::sync::atomic::Ordering;
use std::sync::Arc;

use super::channel::Channel;
use super::errors::{RecvError, TryRecvError};
use super::sender;
use sender::Sender;

#[derive(Clone)]
pub struct Receiver<T> {
    channel: Arc<Channel<T>>,
}

impl<T> Receiver<T> {
    pub fn try_recv(&self) -> Result<T, TryRecvError> {
        let mut queue = self.channel.queue.lock().unwrap();
        let cond = &self.channel.cond;
        let has_sender = || self.channel.sender_count.load(Ordering::Relaxed) > 0;

        if queue.is_empty() && has_sender() {
            queue = cond.wait(queue).unwrap();
        }

        match (queue.pop_front(), has_sender()) {
            (Some(value), _) => Ok(value),
            (None, true) => Err(TryRecvError::Empty),
            (None, false) => Err(TryRecvError::Disconnected),
        }
    }

    pub fn recv(&self) -> Result<T, RecvError> {
        loop {
            match self.try_recv() {
                Ok(value) => return Ok(value),
                Err(TryRecvError::Empty) => continue,
                Err(TryRecvError::Disconnected) => return Err(RecvError()),
            }
        }
    }

    pub fn create_sender(&self) -> Sender<T> {
        sender::from_channel(self.channel.clone())
    }

    fn new(channel: Arc<Channel<T>>) -> Self {
        Self { channel }
    }
}

pub fn from_channel<T>(channel: Arc<Channel<T>>) -> Receiver<T> {
    Receiver::new(channel)
}
