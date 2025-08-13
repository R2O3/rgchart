use serde::{Deserialize, Serialize};
use crate::utils::serde::trim_float;

#[derive(Debug, Deserialize, Serialize)]
pub struct AudioSample {
    #[serde(rename = "Path")]
    pub path: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SoundEffect {
    #[serde(rename = "StartTime", serialize_with = "trim_float")]
    pub start_time: f32,
    
    #[serde(rename = "Sample")]
    pub sample: usize,
    
    #[serde(rename = "Volume", skip_serializing_if = "is_full_volume")]
    pub volume: u8,
}

fn is_full_volume(volume: &u8) -> bool {
    *volume >= 100
}

#[derive(Debug, Deserialize, Serialize)]
pub struct KeySound {
    #[serde(rename = "Sample")]
    pub sample: usize,
    
    #[serde(rename = "Volume", skip_serializing_if = "is_full_volume")]
    pub volume: u8,
}
