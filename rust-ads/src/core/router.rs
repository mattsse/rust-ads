use core::ads::*;
use core::connection::AmsConnection;

use std::net::{Ipv4Addr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

/// max amount of connections the router can handle
pub const MAX_CONNECTIONS: u8 = 128;

pub const PORT_BASE1: u16 = 3000;

pub const ADS_TCP_SERVER_PORT: u16 = 0xBF02; // 48898 ADS-Protocol port

pub struct ConnectionEntry {
    conn: AmsConnection,
    net_id: AmsNetId,
}

/// A Ams Router that manages routes and connections
///
/// workflow:   1. Add a  route
///             2. check if already available
///             3. create new connection that spwans a TcpListener
pub struct AmsRouter {
    local_ams_net_id: Option<AmsNetId>,
    connections: Vec<Arc<Mutex<ConnectionEntry>>>,
}

impl AmsRouter {
    #[doc = "Create a new Router for a target AmsNetId"]
    pub fn new(local_ams_net_id: AmsNetId) -> AmsRouter {
        AmsRouter {
            local_ams_net_id: Some(local_ams_net_id),
            connections: Vec::new(),
        }
    }

    pub fn add_route(&mut self, addr: AmsNetId, ip_v4: Ipv4Addr) {}

    pub fn close_route(&mut self, addr: &AmsNetId) {
        // TODO drop the route if available; should return a resutl
    }

    // ->Result<Arc<Mutex<ConnectionEntry>
    pub fn get_connection(&self, addr: &AmsNetId) {
        // TODO check how to deref a mutex; aquire lock necessary?
        //        self.connections.iter()
        //            .filter(|x|x.)
    }
}
