use crate::core::graphics::{Palette, Resolution, Sprite};
use serde::{Deserialize, Serialize};

use super::graphics::FrameRate;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rom {
    pub resolution: Resolution,
    pub frame_rate: FrameRate,
    pub sprites: Box<[Sprite]>,
    pub palettes: Box<[Palette]>,
    pub sounds: Sounds,
}

impl Default for Rom {
    fn default() -> Self {
        Self {
            resolution: Resolution::Low,
            frame_rate: FrameRate::Normal,
            sprites: vec![].into_boxed_slice(),
            palettes: vec![Palette::bubblegum16()].into_boxed_slice(),
            sounds: Sounds {},
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sounds {
    //TODO: This
}
