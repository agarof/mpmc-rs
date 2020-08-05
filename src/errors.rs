use std::fmt;

pub struct RecvError();

impl fmt::Display for RecvError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sender disconnected")
    }
}

impl fmt::Debug for RecvError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sender disconnected")
    }
}

impl std::error::Error for RecvError {}

pub enum TryRecvError {
    Empty,
    Disconnected,
}

impl TryRecvError {
    fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            TryRecvError::Empty => "try_recv returned an empty value",
            TryRecvError::Disconnected => "Sender disconnected",
        };

        write!(f, "{}", value)
    }
}

impl fmt::Display for TryRecvError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.print(f)
    }
}

impl fmt::Debug for TryRecvError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.print(f)
    }
}

impl std::error::Error for TryRecvError {}
