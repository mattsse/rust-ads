/// TODO implement ADS Connection-> use simple TCP Listener
/// standard port should be 3000
/// TODO add async wrapper based on tokio or directly async impl?!

use core::ads::{ADS_TCP_SERVER_PORT, AdsCommandId, AdsError, AmsAddress, AmsNetId, Result};
use core::requests::*;
use core::responses::*;
use core::router::RouterState;
use std::io;
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream, UdpSocket};
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};

// TODO see BytesMut for Buffer impl crate bytes
// TODO see Decode Encode traits for enc/dec the request/response data

// TODO impl
pub trait AmsConnectionMockup {
    // TODO impl connection
    // TODO impl methods for send/recv junks of data

    /// Beckhoff ads impl
    /// Connections owns a tcpsocket which connects to AmsAddr (Ipv4:port)
    /// tcp connections spawns a new thread which is the reactor and handles recv
    /// check how opcua stores the tcplistner object

    /// disconnect from its router
    fn disconnect() -> bool;

    // could return a future when used in async
    fn connect(addr: AmsAddress) -> bool;
}

/// is responsible for connecting the server with an ads client

pub struct AmsConnection {
    router_state: Arc<RwLock<RouterState>>,
    //TODO is the AmsRouter parent reference optional?
    // connection has an AmsRouter as its parent element
    dest_ip: Ipv4Addr,
    ams_id: AmsNetId,
    stream: Option<TcpStream>,
}

// TODO implement buffer with preallocated memory -> @see BytesMut

impl AmsConnection {
    /// create a new AmsConnection object
    pub fn new(router_state: Arc<RwLock<RouterState>>, dest_ip: Ipv4Addr, ams_id: AmsNetId) -> Self {
        AmsConnection {
            router_state,
            dest_ip,
            ams_id,
            stream: None,
        }
    }

    pub fn is_connected(&self) -> bool {
        self.stream.is_some()
    }

    fn stream(&mut self) -> &mut TcpStream {
        self.stream.as_mut().unwrap()
    }

    pub fn local_addr(&self) -> Result<SocketAddr> {
        match self.stream {
            Some(ref s) => s.local_addr().map_err(|_| AdsError::BadStreamNotConnected),
            _ => Err(AdsError::BadStreamNotConnected)
        }
    }

    /// connect the stream
    pub fn connect(&mut self) -> Result<()> {
        if self.is_connected() {
            panic!("Should not try to connect when already connected");
        }

        let stream = TcpStream::connect((self.dest_ip, ADS_TCP_SERVER_PORT))
            .map_err(|e| AdsError::BadStreamNotConnected)?;
        self.stream = Some(stream);

        Ok(())
    }

    // TODO how to return a trait as result datatype?!
    pub fn write<T:AdsCommandPayload>(
        &mut self,
        request: &AdsRequest<T>,
        src_addr: AmsAddress,
    ) -> Result<()> {



        // steps:
        // 1. create the AmsHeader
        // 2. create the amsTcpHeader
        // 3. form the payload [tcpheader, amsheader, [request_info(cmd_id, length), request_data]]
        // 4. write to tcpstream

        Err(AdsError::TargetNotReachable)
    }

    pub fn dest_id(&self) -> &Ipv4Addr {
        &self.dest_ip
    }

    pub fn update_dest_ip(&mut self, dest_ip: Ipv4Addr) -> Result<()> {
        unimplemented!()
    }

    pub fn ams_id(&self) -> &AmsNetId {
        &self.ams_id
    }
}

impl Drop for AmsConnection {
    fn drop(&mut self) {
        // join any waiting recieves
    }
}