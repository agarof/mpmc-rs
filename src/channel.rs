use std::collections::VecDeque;
use std::sync::atomic::AtomicUsize;
use std::sync::{Condvar, Mutex};

pub struct Channel<T> {
    pub queue: Mutex<VecDeque<T>>,
    pub cond: Condvar,
    pub sender_count: AtomicUsize,
}

impl<T> Channel<T> {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
            cond: Condvar::new(),
            sender_count: AtomicUsize::new(1),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            queue: Mutex::new(VecDeque::with_capacity(capacity)),
            cond: Condvar::new(),
            sender_count: AtomicUsize::new(1),
        }
    }
}
