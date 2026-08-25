use std::net::{IpAddr, UdpSocket, SocketAddr};

use crate::artnet;

pub struct ArtNetSender {
    pub target_ip: IpAddr,
    pub target_port: u16,

    socket: Option<UdpSocket>,
    addr: SocketAddr,
}

impl ArtNetSender {
    pub fn new(ip: IpAddr, port: u16) -> Self {
        Self {
            target_ip: ip,
            target_port: port,
            socket: UdpSocket::bind("0.0.0.0:0").ok(),
            addr: SocketAddr::new(ip, port)
        }
    }

    pub fn send(&self, packet: &artnet::ArtNetPacket) {
        if let Some(socket) = &self.socket {
            socket.send_to(&packet.to_raw(), self.addr);
        }
    }
}