use strum::EnumString;

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

impl From<f32> for Upright {
    fn from(value: f32) -> Self {
        if value <= 0.35 {
            Upright::Prone
        } else if value <= 0.59999996 {
            Upright::Crouching
        } else {
            Upright::Standing
        }
    }
}
