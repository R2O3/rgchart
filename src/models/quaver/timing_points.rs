use serde::{Deserialize, Serialize};
use crate::utils::serde::trim_float;

#[derive(Debug, Deserialize, Serialize)]
pub struct TimingPoint {
    #[serde(rename = "StartTime", serialize_with = "trim_float")]
    pub start_time: f32,
    
    #[serde(rename = "Bpm")]
    pub bpm: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SliderVelocity {
    #[serde(rename = "StartTime", serialize_with = "trim_float")]
    pub start_time: f32,
    
    #[serde(rename = "Multiplier")]
    pub multiplier: f32,
}
