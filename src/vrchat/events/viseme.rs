use strum::EnumString;

#[derive(Debug, PartialEq, Default, EnumString)]
pub enum Viseme {
    #[default]
    Sil = 0,
    Pp = 1,
    Ff = 2,
    Th = 3,
    Dd = 4,
    Kk = 5,
    Ch = 6,
    Ss = 7,
    Nn = 8,
    Rr = 9,
    Aa = 10,
    E = 11,
    I = 12,
    O = 13,
    U = 14,
}

impl From<u8> for Viseme {
    fn from(value: u8) -> Self {
        match value {
            0 => Viseme::Sil,
            1 => Viseme::Pp,
            2 => Viseme::Ff,
            3 => Viseme::Th,
            4 => Viseme::Dd,
            5 => Viseme::Kk,
            6 => Viseme::Ch,
            7 => Viseme::Ss,
            8 => Viseme::Nn,
            9 => Viseme::Rr,
            10 => Viseme::Aa,
            11 => Viseme::E,
            12 => Viseme::I,
            13 => Viseme::O,
            14 => Viseme::U,
            _ => panic!("Invalid viseme value: {value}"),
        }
    }
}

