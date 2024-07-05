use eyre::Result;
use rosc::{OscMessage, OscPacket};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::net::UdpSocket;

pub const RECV_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9001);
pub const SEND_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9000);

#[allow(dead_code)]
pub struct OscClient {
    socket: UdpSocket,
    send_addr: SocketAddr,
    recv_addr: SocketAddr,
}

impl OscClient {
    pub async fn new(send_addr: SocketAddr, recv_addr: SocketAddr) -> Result<Self> {
        let socket = UdpSocket::bind(recv_addr).await?;
        Ok(Self {
            socket,
            send_addr,
            recv_addr,
        })
    }

    pub async fn default() -> Result<Self> {
        Self::new(SEND_ADDR, RECV_ADDR).await
    }

    fn encode(msg: OscMessage) -> Result<Vec<u8>, rosc::OscError> {
        let packet = rosc::OscPacket::Message(msg);
        rosc::encoder::encode(&packet)
    }

    /// Decode received u8 array into rosc packet
    fn decode(msg: &[u8]) -> Result<OscPacket> {
        let (_, packet) = rosc::decoder::decode_udp(msg)?;
        Ok(packet)
    }

    /// Parse the rosc packet into rosc messages
    fn parse(packet: OscPacket) -> Vec<OscMessage> {
        match packet {
            OscPacket::Message(msg) => {
                vec![msg]
            }
            OscPacket::Bundle(bundle) => bundle
                .content
                .iter()
                .map(|p| p.to_owned())
                .flat_map(Self::parse)
                .collect::<Vec<OscMessage>>(),
        }
    }

    pub async fn send(&self, msg: OscMessage) -> Result<()> {
        let msg = Self::encode(msg)?;
        let msg = &msg[..];
        self.socket.send_to(msg, self.send_addr).await?;
        Ok(())
    }

    pub async fn recv(&self) -> Result<Vec<OscMessage>> {
        let mut buf = [0; 2048];
        let (size, _) = self.socket.recv_from(&mut buf).await?;
        let packet = Self::decode(&buf[..size])?;
        Ok(Self::parse(packet))
    }

    pub async fn listen(&self, on_msg: fn(&OscMessage) -> ()) -> ! {
        loop {
            self.recv().await.iter().flatten().for_each(on_msg);
        }
    }
}
