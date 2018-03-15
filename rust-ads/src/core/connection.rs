/// TODO implement ADS Connection-> use simple TCP Listener
/// standard port should be 3000
/// TODO add async wrapper based on tokio or directly async impl?!

use core::ads::{AdsCommandId, AmsAddress, AmsNetId};
use core::router::AmsRouter;
use core::requests::*;
use core::responses::*;

use std::rc::Weak;
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream, UdpSocket};

/// Types for IO-Errors
pub enum AdsConnectionError {
    MissingRoute,
    PortAlreadyInUse,
    ClientPortNotOpen,
    TargetNotReachable,
}

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
    router: Option<Weak<AmsRouter>>,
    //TODO is the AmsRouter parent reference optional?
    // connection has an AmsRouter as its parent element
    dest_ip: Ipv4Addr,
    owm_id: AmsNetId,
    stream: TcpStream,
}

// TODO implement buffer with preallocated memory -> @see BytesMut

impl AmsConnection {
    pub fn connect(&mut self) {}

    // TODO how to return a trait as result datatype?!
    pub fn write(
        &mut self,
        request: &AdsRequest,
        dest_addr: AmsAddress,
        cmd_id: AdsCommandId,
    ) -> Result<(), AdsConnectionError> {
        // steps:
        // 1. create the AmsHeader
        // 2. create the amsTcpHeader
        // 3. form the payload [tcpheader, amsheader, [request_info(cmd_id, length), request_data]]
        // 4. write to tcpstream

        Err(AdsConnectionError::TargetNotReachable)
    }
}
