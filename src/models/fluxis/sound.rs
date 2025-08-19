use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HitSoundFade {
    pub time: f32,
    pub volume: f32,
    pub fade_in: f32,
    pub fade_out: f32,
}