use chrono::Duration;
use core::ads::{AdsError, AmsAddress, Result, State};
use core::connection::AmsConnection;
use core::notify::NotificationDispatcher;
use std::borrow::Borrow;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, RwLock};
#[derive(Debug)]
pub struct NotifyMapping<'a> {
    id: u32,
    dispatcher: Arc<RwLock<NotificationDispatcher<'a, AmsConnection<'a>>>>,
}

impl<'a> Hash for NotifyMapping<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<'a> PartialEq<NotifyMapping<'a>> for NotifyMapping<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<'a> Borrow<u32> for NotifyMapping<'a> {
    fn borrow(&self) -> &u32 {
        &self.id
    }
}

impl<'a> Eq for NotifyMapping<'a> {}

#[derive(Debug)]
pub struct AdsPort<'a> {
    port: u16,
    pub timeout: Option<Duration>,
    state: State,
    mappings: HashSet<NotifyMapping<'a>>,
}

impl<'a> AdsPort<'a> {
    pub fn new(port: u16, state: State) -> Self {
        AdsPort {
            port,
            timeout: None,
            state,
            mappings: HashSet::new(),
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

    pub fn add_notification(&mut self, mapping: NotifyMapping<'a>) -> bool {
        self.mappings.insert(mapping)
    }

    pub fn delete_notification(&mut self, ams: &AmsAddress, id: u32) -> Result<bool> {
//        let id = self.mappings.get(&id).map(move |m| {
//            // TODO erase
//            let mut dispatcher = m.dispatcher.write().map_err(|_| AdsError::SyncError)?;
//            if dispatcher.conn.1 == *ams {
//                dispatcher.erase(id, self.timeout);
//                // TODO erase notification mapping from mappings
//                self.mappings.remove(&id);
//                Ok(id)
//            } else {
//                Err(AdsError::InvalidAddress)
//            }
//        });
        Ok(false)
    }
}
