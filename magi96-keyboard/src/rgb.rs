use std::fmt;

/// RGB effects supported by the Magi96 keyboard
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RgbEffect {
    Off = 0,
    Wave = 1,
    ColourCloud = 2,
    Vortex = 3,
    MixColour = 4,
    Breathe = 5,
    Light = 6,
    SlowlyOff = 7,
    Stone = 8,
    Laser = 9,
    Starry = 10,
    FlowersOpen = 11,
    Traverse = 12,
    WaveBar = 13,
    Meteor = 14,
    Rain = 15,
    Scan = 16,
    TriggerColour = 17,
    CenterSpread = 18,
}

impl RgbEffect {
    /// Parse an effect from a string name
    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "off" => Some(Self::Off),
            "wave" => Some(Self::Wave),
            "colourcloud" | "colour_cloud" | "color_cloud" => Some(Self::ColourCloud),
            "vortex" => Some(Self::Vortex),
            "mixcolour" | "mix_colour" | "mix_color" => Some(Self::MixColour),
            "breathe" => Some(Self::Breathe),
            "light" => Some(Self::Light),
            "slowlyoff" | "slowly_off" => Some(Self::SlowlyOff),
            "stone" => Some(Self::Stone),
            "laser" => Some(Self::Laser),
            "starry" => Some(Self::Starry),
            "flowersopen" | "flowers_open" => Some(Self::FlowersOpen),
            "traverse" => Some(Self::Traverse),
            "wavebar" | "wave_bar" => Some(Self::WaveBar),
            "meteor" => Some(Self::Meteor),
            "rain" => Some(Self::Rain),
            "scan" => Some(Self::Scan),
            "triggercolour" | "trigger_colour" | "trigger_color" => Some(Self::TriggerColour),
            "centerspread" | "center_spread" => Some(Self::CenterSpread),
            _ => None,
        }
    }

    /// Get all available effects
    pub fn all() -> Vec<Self> {
        vec![
            Self::Off,
            Self::Wave,
            Self::ColourCloud,
            Self::Vortex,
            Self::MixColour,
            Self::Breathe,
            Self::Light,
            Self::SlowlyOff,
            Self::Stone,
            Self::Laser,
            Self::Starry,
            Self::FlowersOpen,
            Self::Traverse,
            Self::WaveBar,
            Self::Meteor,
            Self::Rain,
            Self::Scan,
            Self::TriggerColour,
            Self::CenterSpread,
        ]
    }
}

impl fmt::Display for RgbEffect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Off => "Off",
            Self::Wave => "Wave",
            Self::ColourCloud => "Colour Cloud",
            Self::Vortex => "Vortex",
            Self::MixColour => "Mix Colour",
            Self::Breathe => "Breathe",
            Self::Light => "Light",
            Self::SlowlyOff => "Slowly Off",
            Self::Stone => "Stone",
            Self::Laser => "Laser",
            Self::Starry => "Starry",
            Self::FlowersOpen => "Flowers Open",
            Self::Traverse => "Traverse",
            Self::WaveBar => "Wave Bar",
            Self::Meteor => "Meteor",
            Self::Rain => "Rain",
            Self::Scan => "Scan",
            Self::TriggerColour => "Trigger Colour",
            Self::CenterSpread => "Center Spread",
        };
        write!(f, "{}", name)
    }
}

/// HSV color representation
#[derive(Debug, Clone, Copy)]
pub struct HsvColor {
    pub hue: u8,        // 0-255
    pub saturation: u8, // 0-255
    pub value: u8,      // 0-255
}

impl HsvColor {
    pub fn new(hue: u8, saturation: u8, value: u8) -> Self {
        Self {
            hue,
            saturation,
            value,
        }
    }
}
