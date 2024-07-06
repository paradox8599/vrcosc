use rosc::OscType;

#[derive(Debug)]
pub enum VrcType {
    Int(u8),
    Float(f32),
    Bool(bool),
}

impl From<OscType> for VrcType {
    fn from(ty: OscType) -> Self {
        match ty {
            OscType::Int(v) => Self::Int(v as u8),
            OscType::Float(v) => Self::Float(v),
            OscType::Bool(v) => Self::Bool(v),
            _ => unreachable!(),
        }
    }
}

impl From<VrcType> for OscType {
    fn from(ty: VrcType) -> Self {
        match ty {
            VrcType::Int(v) => Self::Int(v as i32),
            VrcType::Float(v) => Self::Float(v),
            VrcType::Bool(v) => Self::Bool(v),
        }
    }
}

