use core::ads::*;
use core::connection::AmsConnection;
use std::net::{Ipv4Addr, TcpListener, TcpStream};
use std::sync::{Arc, RwLock};

/// max amount of connections the router can handle
pub const MAX_CONNECTIONS: u8 = 128;

pub const PORT_BASE: u16 = 3000;

pub struct RouterState {
    local_ams_net_id: AmsNetId
}

/// A Ams Router that manages routes and connections
///
/// workflow:   1. open port
///             2. Add a  route
///             3. check if already available
///             4. create new connection that spwans a TcpListener
pub struct AmsRouter {
    state: Arc<RwLock<RouterState>>,
    connections: Vec<Arc<RwLock<AmsConnection>>>,
}

impl AmsRouter {
    pub fn new(local_ams_net_id: AmsNetId) -> AmsRouter {
        let state = Arc::new(RwLock::new(RouterState {
            local_ams_net_id
        }));
        AmsRouter {
            state,
            connections: Vec::new(),
        }
    }

    pub fn add_route(&mut self, addr: AmsNetId, ipv4: Ipv4Addr) -> Result<&Ipv4Addr> {
        if let Some(lock) = self.any_conn(&addr) {
            let rw = lock?;
            let conn = rw.read().map_err(|_| AdsError::SyncError)?;
            if *conn.dest_id() != ipv4 {
                // there is already a route for this netid but with a different id
                return Err(AdsError::PortAlreadyInUse(3000));
            }
        }

        Err(AdsError::PortAlreadyInUse(3000))
    }
    pub fn add_route_derive(&mut self, addr: AmsNetId) {}

    pub fn close_route(&mut self, addr: &AmsNetId) {
        // TODO drop the route if available; should return a resutl
    }

    // TODO figure out how to pass both ams net id and ipv4 addr as ref?! mb as trait object?! --> investigate

    fn any_conn(&self, addr: &AmsNetId) -> Option<Result<&RwLock<AmsConnection>>> {
        for conn in &self.connections {
            if let Ok(lock) = conn.read() {
                if lock.ams_id() == addr {
                    return Some(Ok(conn));
                }
            } else {
                return Some(Err(AdsError::InvalidAddress));
            }
        }
        None
    }

    /// find the Connection matching the ams id
    pub fn connection(&self, addr: &AmsNetId) -> Result<&RwLock<AmsConnection>> {
        match self.any_conn(addr) {
            Some(lock) => lock,
            _ => Err(AdsError::InvalidAddress),
        }
    }
}
