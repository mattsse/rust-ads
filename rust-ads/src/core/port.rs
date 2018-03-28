use chrono::Duration;
use core::ads::State;
use core::connection::AmsConnection;
use core::notify::NotificationDispatcher;
use std::sync::{Arc, RwLock};

pub type NotifyMapping<'a> = (
    u32,
    Arc<RwLock<NotificationDispatcher<'a, AmsConnection<'a>>>>,
);

#[derive(Debug)]
pub struct AdsPort<'a> {
    port: u16,
    timeout: Option<Duration>,
    state: State,
    mappings: Vec<NotifyMapping<'a>>,
}

impl<'a> AdsPort<'a> {
    pub fn new(port: u16, state: State) -> Self {
        AdsPort {
            port,
            timeout: None,
            state,
            mappings: Vec::new(),
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
        // TODO erase mappings

        self.state = State::CLOSED;
        self.port
    }

    pub fn add_notification(&mut self, mappign: NotifyMapping) {
        unimplemented!()
    }
}
