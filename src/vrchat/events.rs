use rosc::{OscMessage, OscType};
use strum::EnumString;

// Upright

#[derive(Debug, PartialEq, Default, EnumString)]
pub enum Upright {
    #[default]
    Standing,
    Crouching,
    Prone,
    Custom(f32),
}

impl Upright {
    pub fn value(&self) -> f32 {
        match self {
            Upright::Standing => 1.0,
            Upright::Crouching => 0.59999996,
            Upright::Prone => 0.35,
            Upright::Custom(v) => *v,
        }
    }
}

// Gestures

#[derive(Debug, PartialEq, Default, EnumString)]
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

// Visemes

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

// Tracking Types

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

// Avatar Parameters

/// https://creators.vrchat.com/avatars/animator-parameters/#parameters
#[derive(Debug, PartialEq, EnumString)]
pub enum VRChatEvent {
    /// If the avatar is being worn locally
    IsLocal(bool),
    /// Oculus viseme index (0-14).
    /// When using Jawbone/Jawflap, (0-100) indicating volume
    Viseme(i8),
    /// Microphone volume
    Voice(f32),
    /// Gesture from L hand control `(0-7)`
    GestureLeft(Gesture),
    /// Gesture from R hand control `(0-7)`
    GestureRight(Gesture),
    /// Analog trigger `L`
    GestureLeftWeight(f32),
    /// Analog trigger `R`
    GestureRightWeight(f32),
    /// Angular velocity on the Y axis
    AngularY(f32),
    /// Lateral move speed in m/s
    VelocityX(f32),
    /// Vertical move speed in m/s
    VelocityY(f32),
    /// Forward move speed in m/s
    VelocityZ(f32),
    /// Total magnitude of velocity
    VelocityMagnitude(f32),
    /// How "upright" you are.
    /// 0 is prone
    Upright(Upright),
    /// True if player touching ground
    Grounded(bool),
    /// True if player in station
    Seated(bool),
    /// Is player afk
    AFK(bool),

    /// TODO: Expression 1 - 16 ?

    /// Tracking types
    TrackingType(TrackingType),
    /// `1` if the user is in VR, `0` otherwise
    VRMode(i8),
    /// Whether the user has muted themselves
    MuteSelf(bool),
    /// Whether the user is in a station
    InStation(bool),
    /// Whether the user is in earmuff mode
    Earmuffs(bool),
    /// Whether the user is scaled using avatar scaling
    ScaleModified(bool),
    /// Scale ratio, scaled height / default height
    ScaleFactor(f32),
    /// Inverse scale ratio , default height / scaled height
    ScaleFactorInverse(f32),
    /// The avatr's eye height in meters
    EyeHeightAsMeters(f32),
    /// Relation of the avatar's eye height in meters relative to the default scaling limits (0.2-5.0).
    /// An avatar scaled to 2m will report: `(2.0 - 0.2) / (5.0 - 0.2) = 0.375`
    EyeHeightAsPercent(f32),

    /// Other custom messages
    Int(String, i8),
    Float(String, f32),
    Bool(String, bool),
}

// impl TryFrom<OscMessage> for Avatar {}

impl From<OscMessage> for VRChatEvent {
    fn from(value: OscMessage) -> Self {
        let addr = value.addr.split('/').last().unwrap().to_string();

        // // NOTE: Parse to int/float/bool only
        // let _ = {
        //     let addr = addr.clone();
        //     let arg: OscType = value
        //         .args
        //         .first()
        //         .expect("Message should have exactly one arg")
        //         .to_owned();
        //     match arg {
        //         OscType::Int(v) => AvatarMessage::Int(addr, v as i8),
        //         OscType::Float(v) => AvatarMessage::Float(addr, v),
        //         OscType::Bool(v) => AvatarMessage::Bool(addr, v),
        //         v => panic!("unhandled osc message type: {v:?} from {addr}"),
        //     }
        // };

        // FIX: parse enum with values
        {
            let a_msg = addr.parse::<VRChatEvent>();
            // let arg: OscType = value
            //     .args
            //     .first()
            //     .expect("Message should have exactly one arg")
            //     .to_owned();
            if let Ok(a) = a_msg {
                a
            } else {
                let arg: OscType = value
                    .args
                    .first()
                    .expect("Message should have one arg")
                    .to_owned();
                match arg {
                    OscType::Int(v) => VRChatEvent::Int(addr, v as i8),
                    OscType::Float(v) => VRChatEvent::Float(addr, v),
                    OscType::Bool(v) => VRChatEvent::Bool(addr, v),
                    v => panic!("unhandled osc message type: {v:?} from {a_msg:?}"),
                }
            }
        }
    }
}

impl VRChatEvent {
    fn get_name(&self) -> String {
        let s = format!("{:?}", self);
        let s = s.split_once('(').unwrap();
        s.0.to_string()
    }

    pub fn is_addr(&self, addr: &str) -> bool {
        addr.ends_with(&self.get_name())
    }
}
