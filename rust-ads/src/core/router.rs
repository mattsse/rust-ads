use core::ads::*;
use core::connection::AmsConnection;
use core::port::AdsPort;
use core::requests::{AdsCommandPayload, AdsRequest};
use std::net::{Ipv4Addr, TcpListener, TcpStream};
use std::sync::{Arc, RwLock};

/// max amount of connections the router can handle
pub const MAX_PORTS: usize = 128;

pub const PORT_BASE: usize = 3000;

pub struct RouterState {
    local_ams_net_id: AmsNetId,
}

/// Router is central Notification Broker -> Dispatches all incoming/outgoing notifications

/// A Ams Router that manages routes and connections
///
/// workflow:   1. open port
///             2. Add a  route
///             3. check if already available
///             4. create new connection that spwans a TcpListener
pub struct AmsRouter {
    /// current configuration of the router
    state: Arc<RwLock<RouterState>>,
    /// all connections to this router
    connections: Vec<Arc<RwLock<AmsConnection>>>,
    ports: Vec<Arc<RwLock<AdsPort>>>,
}

impl AmsRouter {
    /// create a new AmsRouter with the local ams net id
    pub fn new(local_ams_net_id: AmsNetId) -> AmsRouter {
        let state = Arc::new(RwLock::new(RouterState { local_ams_net_id }));
        AmsRouter {
            state,
            connections: Vec::new(),
            ports: Vec::with_capacity(MAX_PORTS),
        }
    }

    pub fn open_port(&mut self) -> Result<u16> {
        for port in self.ports.iter() {
            let mut lock = port.write().map_err(|_| AdsError::SyncError)?;
            if lock.is_closed() {
                return Ok(lock.open());
            }
        }
        if self.ports.len() >= MAX_PORTS {
            return Err(AdsError::NoMemoryLeft);
        }
        let open_port = PORT_BASE + self.ports.len();
        let ads_port = AdsPort::new(open_port as u16, State::OPEN);
        self.ports.push(Arc::new(RwLock::new(ads_port)));
        Ok(open_port as u16)
    }

    fn port_in_range(&self, port: usize) -> bool {
        port >= PORT_BASE && port < PORT_BASE + self.ports.len()
    }

    pub fn close_port(&mut self, port: u16) -> Result<u16> {
        if !self.port_in_range(port as usize) {
            return Err(AdsError::BadPort(port));
        }

        let p = self.ports
            .get((port as usize) - (PORT_BASE + 1))
            .ok_or(AdsError::BadPort(port))?;
        let mut lock = p.write().map_err(|_| AdsError::SyncError)?;
        if lock.is_open() {
            Ok(lock.close())
        } else {
            Err(AdsError::PortNotOpen(port))
        }
    }

    fn is_port_open(&self, port: u16) -> Result<bool> {
        if !self.port_in_range(port as usize) {
            return Err(AdsError::BadPort(port));
        }
        let p = self.ports
            .get((port as usize) - (PORT_BASE + 1))
            .ok_or(AdsError::BadPort(port))?;
        let lock = p.read().map_err(|_| AdsError::SyncError)?;
        Ok(lock.is_open())
    }

    /// add a new route with the ams net id targeting the ipv4 address
    pub fn add_route(&mut self, addr: AmsNetId, ipv4: Ipv4Addr) -> Result<&Ipv4Addr> {
        if let Some(lock) = self.any_conn(&addr) {
            let rw = lock?;
            let conn = rw.read().map_err(|_| AdsError::SyncError)?;
            if *conn.dest_id() != ipv4 {
                // there is already a route for this netid but with a different id
                return Err(AdsError::PortAlreadyInUse(3000));
            }
        }
        // TODO add route
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

    /// get the local address to this router described by the ams net id and the port
    pub fn local_address(&self, port: u16) -> Result<AmsAddress> {
        let is_open = self.is_port_open(port)?;
        if !is_open {
            Err(AdsError::PortNotOpen(port))
        } else {
            let lock = self.state.read().map_err(|_| AdsError::SyncError)?;
            Ok(AmsAddress::new(lock.local_ams_net_id.clone(), port))
        }
    }

    /// update the current ams net id of the router
    pub fn set_local_net_id(&mut self, net_id: AmsNetId) -> Result<&RwLock<RouterState>> {
        let mut lock = self.state.write().map_err(|_| AdsError::SyncError)?;
        lock.local_ams_net_id = net_id;
        Ok(&self.state)
    }

    pub fn add_notification<T: AdsCommandPayload>(
        &mut self,
        request: &AdsRequest<T>,
    ) -> Result<()> {
        unimplemented!()
    }
}
