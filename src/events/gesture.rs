use strum::{EnumString, VariantArray};

#[derive(Debug, PartialEq, Default, EnumString, VariantArray)]
pub enum Gesture {
    #[default]
    Neutral = 0,
    Fist = 1,
    HandOpen = 2,
    FingerPoint = 3,
    Victory = 4,
    RockNRoll = 5,
    HandGun = 6,
    ThumbsUp = 7,
}

impl From<u8> for Gesture {
    fn from(value: u8) -> Self {
        match value {
            0 => Gesture::Neutral,
            1 => Gesture::Fist,
            2 => Gesture::HandOpen,
            3 => Gesture::FingerPoint,
            4 => Gesture::Victory,
            5 => Gesture::RockNRoll,
            6 => Gesture::HandGun,
            7 => Gesture::ThumbsUp,
            _ => panic!("Invalid gesture value: {value}"),
        }
    }
}
