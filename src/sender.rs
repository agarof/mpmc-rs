use std::sync::atomic::Ordering;
use std::sync::Arc;

use super::channel::Channel;
use super::receiver;
use super::receiver::Receiver;

pub struct Sender<T> {
    channel: Arc<Channel<T>>,
}

impl<T> Sender<T> {
    pub fn new() -> Self {
        Self {
            channel: Arc::new(Channel::new()),
        }
    }

    fn new_from_channel(channel: Arc<Channel<T>>) -> Self {
        channel.sender_count.fetch_add(1, Ordering::Relaxed);

        Self { channel }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            channel: Arc::new(Channel::with_capacity(capacity)),
        }
    }

    pub fn create_receiver(&self) -> Receiver<T> {
        receiver::from_channel(self.channel.clone())
    }

    pub fn send(&self, value: T) {
        let mut queue = self.channel.queue.lock().unwrap();

        queue.push_back(value);
        self.channel.cond.notify_one();
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Self::new_from_channel(self.channel.clone())
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let _guard = self.channel.queue.lock().unwrap();
        let count = self.channel.sender_count.fetch_sub(1, Ordering::Relaxed) - 1;

        if count == 0 {
            self.channel.cond.notify_all();
        }
    }
}

pub fn from_channel<T>(channel: Arc<Channel<T>>) -> Sender<T> {
    Sender::new_from_channel(channel)
}
