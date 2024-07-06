use rosc::OscMessage;
use strum::{EnumString, VariantNames};

use crate::{
    vrchat::{Gesture, TrackingType, Upright, Viseme},
    VrcType,
};

/// Avatar Parameters
/// https://creators.vrchat.com/avatars/animator-parameters/#parameters
#[derive(Debug, PartialEq, EnumString, VariantNames)]
pub enum VRChatEvent {
    /// If the avatar is being worn locally
    IsLocal(bool),
    /// Oculus viseme index (0-14).
    /// When using Jawbone/Jawflap, (0-100) indicating volume
    Viseme(Viseme),
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
    VRMode(u8),
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
    Int(String, u8),
    Float(String, f32),
    Bool(String, bool),
}

impl VRChatEvent {
    /// Set the value to the given
    /// Do nothing if the value type is not matching
    pub fn set_value(&mut self, value: &VrcType) {
        *self = match self {
            VRChatEvent::IsLocal(_) if let VrcType::Bool(v) = value => VRChatEvent::IsLocal(*v),
            VRChatEvent::Viseme(_) if let VrcType::Int(v) = value => {
                VRChatEvent::Viseme(Viseme::from(*v))
            }
            VRChatEvent::Voice(_) if let VrcType::Float(v) = value => VRChatEvent::Voice(*v),
            VRChatEvent::GestureLeft(_) if let VrcType::Int(v) = value => {
                VRChatEvent::GestureLeft(Gesture::from(*v))
            }
            VRChatEvent::GestureRight(_) if let VrcType::Int(v) = value => {
                VRChatEvent::GestureRight(Gesture::from(*v))
            }
            VRChatEvent::GestureLeftWeight(_) if let VrcType::Float(v) = value => {
                VRChatEvent::GestureLeftWeight(*v)
            }
            VRChatEvent::GestureRightWeight(_) if let VrcType::Float(v) = value => {
                VRChatEvent::GestureRightWeight(*v)
            }
            VRChatEvent::AngularY(_) if let VrcType::Float(v) = value => VRChatEvent::AngularY(*v),
            VRChatEvent::VelocityX(_) if let VrcType::Float(v) = value => {
                VRChatEvent::VelocityX(*v)
            }
            VRChatEvent::VelocityY(_) if let VrcType::Float(v) = value => {
                VRChatEvent::VelocityY(*v)
            }
            VRChatEvent::VelocityZ(_) if let VrcType::Float(v) = value => {
                VRChatEvent::VelocityZ(*v)
            }
            VRChatEvent::VelocityMagnitude(_) if let VrcType::Float(v) = value => {
                VRChatEvent::VelocityMagnitude(*v)
            }
            VRChatEvent::Upright(_) if let VrcType::Float(v) = value => {
                VRChatEvent::Upright(Upright::from(*v))
            }
            VRChatEvent::Grounded(_) if let VrcType::Bool(v) = value => VRChatEvent::Grounded(*v),
            VRChatEvent::Seated(_) if let VrcType::Bool(v) = value => VRChatEvent::Seated(*v),
            VRChatEvent::AFK(_) if let VrcType::Bool(v) = value => VRChatEvent::AFK(*v),
            VRChatEvent::TrackingType(_) if let VrcType::Int(v) = value => {
                VRChatEvent::TrackingType(TrackingType::from(*v))
            }
            VRChatEvent::VRMode(_) if let VrcType::Int(v) = value => VRChatEvent::VRMode(*v),
            VRChatEvent::MuteSelf(_) if let VrcType::Bool(v) = value => VRChatEvent::MuteSelf(*v),
            VRChatEvent::InStation(_) if let VrcType::Bool(v) = value => VRChatEvent::InStation(*v),
            VRChatEvent::Earmuffs(_) if let VrcType::Bool(v) = value => VRChatEvent::Earmuffs(*v),
            VRChatEvent::ScaleModified(_) if let VrcType::Bool(v) = value => {
                VRChatEvent::ScaleModified(*v)
            }
            VRChatEvent::ScaleFactor(_) if let VrcType::Float(v) = value => {
                VRChatEvent::ScaleFactor(*v)
            }
            VRChatEvent::EyeHeightAsMeters(_) if let VrcType::Float(v) = value => {
                VRChatEvent::EyeHeightAsMeters(*v)
            }
            VRChatEvent::EyeHeightAsPercent(_) if let VrcType::Float(v) = value => {
                VRChatEvent::EyeHeightAsPercent(*v)
            }
            VRChatEvent::Int(name, _) if let VrcType::Int(v) = value => {
                VRChatEvent::Int(name.clone(), *v)
            }
            VRChatEvent::Float(name, _) if let VrcType::Float(v) = value => {
                VRChatEvent::Float(name.clone(), *v)
            }
            VRChatEvent::Bool(name, _) if let VrcType::Bool(v) = value => {
                VRChatEvent::Bool(name.clone(), *v)
            }
            _ => panic!("Invalid value type: {:?}", value),
        };
    }
}

impl From<OscMessage> for VRChatEvent {
    fn from(msg: OscMessage) -> Self {
        let addr = msg.addr.split('/').last().unwrap().to_string();

        // parse string into initial VRChatEvent (without value)
        let vrchat_event_result = addr.parse::<VRChatEvent>();
        // extract value from args
        let event_value = msg
            .args
            .first()
            .expect("Message should have exactly one arg")
            .to_owned();
        let event_value: VrcType = event_value.into();

        match vrchat_event_result {
            // if parsing was successful, set the value and return the event
            Ok(mut e) => {
                e.set_value(&event_value);
                e
            }
            // if parsing failed, try to set the addr and value as custom event
            Err(_) => match event_value {
                VrcType::Int(v) => VRChatEvent::Int(addr, v),
                VrcType::Float(v) => VRChatEvent::Float(addr, v),
                VrcType::Bool(v) => VRChatEvent::Bool(addr, v),
            },
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
