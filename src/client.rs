use eyre::Result;
use rosc::OscPacket;
use serde::Serialize;
use std::{
    fmt::Display,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};
use tokio::net::UdpSocket;

use super::vrc_message::VrcMessage;

pub const BIND_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0);

#[derive(Debug)]
pub struct VrchatClient {
    pub ip: IpAddr,
    pub send_addr: SocketAddr,
    pub recv_addr: SocketAddr,
    send_socket: Option<UdpSocket>,
    recv_socket: Option<UdpSocket>,
}

impl VrchatClient {
    pub fn json(&self) -> serde_json::Value {
        serde_json::json!({
            "ip": self.ip,
            "send_port": self.send_port(),
            "recv_port": self.recv_port()
        })
    }

    pub fn send_port(&self) -> u16 {
        self.send_addr.port()
    }

    pub fn recv_port(&self) -> u16 {
        self.recv_addr.port()
    }
}

impl Display for VrchatClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.json())
    }
}

impl Serialize for VrchatClient {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}", self.json()))
    }
}

/// Create a [VrchatClient] using default addresses and ports
///  - send to vrchat: 127.0.0.1:9000
///  - receive from vrchat: 127.0.0.1:9001
impl Default for VrchatClient {
    fn default() -> Self {
        Self::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9000, 9001)
    }
}

impl VrchatClient {
    pub fn new(ip: IpAddr, send_port: u16, recv_port: u16) -> Self {
        Self {
            ip,
            send_socket: None,
            recv_socket: None,
            send_addr: SocketAddr::new(ip, send_port),
            recv_addr: SocketAddr::new(ip, recv_port),
        }
    }

    pub async fn bind(&mut self) -> Result<()> {
        let recv = UdpSocket::bind(self.recv_addr).await?;
        let send = UdpSocket::bind(BIND_ADDR).await?;
        self.recv_socket = Some(recv);
        self.send_socket = Some(send);
        Ok(())
    }

    pub async fn unbind(&mut self) {
        drop(self.recv_socket.take());
        drop(self.send_socket.take());
    }

    pub fn binded(&self) -> bool {
        self.send_socket.is_some() && self.recv_socket.is_some()
    }

    /// Encode [crate::vrchat::VrcMessage] into an [u8] array
    ///     that can be passed to [tokio::net::UdpSocket::send_to]
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
        let socket = self.send_socket.as_ref();
        match socket {
            Some(socket) => {
                let msg = Self::encode(msg)?;
                let msg = &msg[..];
                socket.send_to(msg, self.send_addr).await?;
                Ok(())
            }
            None => Err(eyre::eyre!(
                "VrchatClient should be binded to the address before sending messages"
            )),
        }
    }

    /// [async] Receive [crate::vrchat::VrcMessage]s from vrchat
    pub async fn recv(&self) -> Result<Vec<VrcMessage>> {
        let socket = self.recv_socket.as_ref();
        match socket {
            Some(socket) => {
                let mut buf = [0; 2048];
                let (size, _) = socket.recv_from(&mut buf).await?;
                let msgs = Self::decode(&buf[..size])?;
                Ok(msgs)
            }
            None => Err(eyre::eyre!(
                "VrchatClient should be binded to the address before receiving messages"
            )),
        }
    }

    /// Try to read received [crate::vrchat::VrcMessage]s
    pub fn try_recv(&self) -> Option<Vec<VrcMessage>> {
        let socket = self.recv_socket.as_ref();
        match socket {
            Some(socket) => {
                let mut buf = [0; 2048];
                if let Ok((size, _)) = socket.try_recv_from(&mut buf) {
                    let msgs = Self::decode(&buf[..size]);
                    if let Ok(msgs) = msgs {
                        return Some(msgs);
                    }
                }
                None
            }
            None => None,
        }
    }

    /// Listen for [crate::vrchat::VrcMessage]s
    pub async fn listen(&self, on_msg: fn(&VrcMessage) -> ()) {
        loop {
            self.recv().await.iter().flatten().for_each(on_msg);
        }
    }
}
