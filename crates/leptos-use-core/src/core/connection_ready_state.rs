use std::fmt;

/// The current state of a network connection.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Copy)]
pub enum ConnectionReadyState {
    Connecting,
    Open,
    Closing,
    Closed,
}

impl fmt::Display for ConnectionReadyState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ConnectionReadyState::Connecting => write!(f, "Connecting"),
            ConnectionReadyState::Open => write!(f, "Open"),
            ConnectionReadyState::Closing => write!(f, "Closing"),
            ConnectionReadyState::Closed => write!(f, "Closed"),
        }
    }
}
