use rosc::OscMessage;

use crate::vrchat::vrc_type::VrcType;

#[derive(Debug)]
pub struct VrcMessage {
    pub addr: String,
    pub value: VrcType,
}

impl VrcMessage {
    pub fn new(addr: String, value: VrcType) -> Self {
        Self { addr, value }
    }
}

impl From<OscMessage> for VrcMessage {
    fn from(msg: OscMessage) -> Self {
        Self {
            addr: msg.addr,
            value: msg.args.into_iter().next().unwrap().into(),
        }
    }
}

impl From<VrcMessage> for OscMessage {
    fn from(msg: VrcMessage) -> Self {
        Self {
            addr: msg.addr,
            args: vec![msg.value.into()],
        }
    }
}
