use eyre::Result;
use rosc::OscPacket;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::net::UdpSocket;

use crate::vrchat::VrcMessage;

pub const RECV_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9001);
pub const SEND_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9000);

#[allow(dead_code)]
pub struct VrchatClient {
    socket: UdpSocket,
    send_addr: SocketAddr,
    recv_addr: SocketAddr,
}

impl VrchatClient {
    pub async fn new(send_addr: SocketAddr, recv_addr: SocketAddr) -> Result<Self> {
        let socket = UdpSocket::bind(recv_addr).await?;
        Ok(Self {
            socket,
            send_addr,
            recv_addr,
        })
    }

    /// Create a [OscClient] using default addresses and ports
    ///  - send to vrchat: 127.0.0.1:9000
    ///  - receive from vrchat: 127.0.0.1:9001
    pub async fn default() -> Result<Self> {
        Self::new(SEND_ADDR, RECV_ADDR).await
    }

    /// Encode [crate::vrchat::VrcMessage] into [u8] array that can be passed to [tokio::net::UdpSocket::send_to]
    fn encode(msg: VrcMessage) -> Result<Vec<u8>, rosc::OscError> {
        let packet = rosc::OscPacket::Message(msg.into());
        rosc::encoder::encode(&packet)
    }

    /// Decode received [u8] array into [VrcMessage]s
    fn decode(msg: &[u8]) -> Result<Vec<VrcMessage>> {
        let (_, packet) = rosc::decoder::decode_udp(msg)?;
        Ok(Self::parse(packet))
    }

    /// Extract [rosc::OscMessage]s from [rosc::OscPacket]
    /// and convert to [crate::vrchat::VrcMessage]s
    fn parse(packet: OscPacket) -> Vec<VrcMessage> {
        match packet {
            OscPacket::Message(msg) => {
                vec![msg.into()]
            }
            OscPacket::Bundle(bundle) => bundle
                .content
                .iter()
                .map(|p| p.to_owned())
                .flat_map(Self::parse)
                .collect::<Vec<VrcMessage>>(),
        }
    }

    /// Send [crate::vrchat::VrcMessage] to vrchat
    pub async fn send(&self, msg: VrcMessage) -> Result<()> {
        let msg = Self::encode(msg)?;
        let msg = &msg[..];
        self.socket.send_to(msg, self.send_addr).await?;
        Ok(())
    }

    /// [async] Receive [crate::vrchat::VrcMessage]s from vrchat
    pub async fn recv(&self) -> Result<Vec<VrcMessage>> {
        let mut buf = [0; 2048];
        let (size, _) = self.socket.recv_from(&mut buf).await?;
        let msgs = Self::decode(&buf[..size])?;
        Ok(msgs)
    }

    /// Try to receive [crate::vrchat::VrcMessage]s
    pub fn try_recv(&self) -> Option<Vec<VrcMessage>> {
        let mut buf = [0; 2048];
        if let Ok((size, _)) = self.socket.try_recv_from(&mut buf) {
            let msgs = Self::decode(&buf[..size]);
            if let Ok(msgs) = msgs {
                return Some(msgs);
            }
        }
        None
    }

    /// Listen for [crate::vrchat::VrcMessage]s
    pub async fn listen(&self, on_msg: fn(&VrcMessage) -> ()) {
        loop {
            self.recv().await.iter().flatten().for_each(on_msg);
        }
    }
}
