use chrono::Duration;
use core::ads::State;

#[derive(Debug, PartialEq, Clone)]
pub struct AdsPort {
    port: u16,
    timeout: Option<Duration>,
    state: State,
}

impl AdsPort {
    pub fn new(port: u16, state: State) -> Self {
        AdsPort {
            port,
            timeout: None,
            state,
        }
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn is_open(&self) -> bool {
        self.state == State::OPEN
    }
    pub fn is_closed(&self) -> bool {
        self.state == State::CLOSED
    }

    pub fn open(&mut self) -> u16 {
        self.state = State::OPEN;
        self.port
    }

    pub fn close(&mut self) -> u16 {
        self.state = State::CLOSED;
        self.port
    }
}
