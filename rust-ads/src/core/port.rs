use chrono::Duration;
use core::ads::State;

pub struct AdsPort {
    port: u16,
    timeout: Option<Duration>,
    state: State,
}

impl AdsPort {
    fn new(port: u16) -> Self {
        AdsPort {
            port,
            timeout: None,
            state: State::CLOSED,
        }
    }
}
