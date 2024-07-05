use strum::EnumString;

#[derive(Debug, PartialEq, Default, EnumString)]
pub enum TrackingType {
    #[default]
    Uninitialized = 0,
    /// Can be any kind of tracking, as generic so ignored
    /// If VRMode is `0`, might be a desktop user
    GenericRig = 1,
    HandsOnlyAV2 = 2,
    /// If VRMode is `1`, this user is in 3-point VR
    /// If VRMode is `0`, this is a desktop user in a humanoid avatar
    HeadNHands = 3,
    Points4 = 4,
    Points5 = 5,
    /// Head, hands, hip, feet
    FullBody = 6,
}

impl From<u8> for TrackingType {
    fn from(value: u8) -> Self {
        match value {
            0 => TrackingType::Uninitialized,
            1 => TrackingType::GenericRig,
            2 => TrackingType::HandsOnlyAV2,
            3 => TrackingType::HeadNHands,
            4 => TrackingType::Points4,
            5 => TrackingType::Points5,
            6 => TrackingType::FullBody,
            _ => panic!("Invalid tracking type value: {value}"),
        }
    }
}
