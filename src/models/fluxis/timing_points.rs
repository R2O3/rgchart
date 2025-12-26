use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TimingPoint {
    pub time: f32,
    pub bpm: f32,
    pub signature: u32,
    
    #[serde(rename = "hide-lines")]
    pub hide_lines: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScrollVelocity {
    pub time: f32,
    pub multiplier: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
}

impl TimingPoint {
    pub fn new(time: f32, bpm: f32, signature: u32) -> Self {
        TimingPoint {
            time,
            bpm,
            signature,
            hide_lines: false,
        }
    }
}

impl Default for TimingPoint {
    fn default() -> Self {
        Self {
            time: 0.0,
            bpm: 120.0,
            signature: 4,
            hide_lines: false,
        }
    }
}

impl ScrollVelocity {
    pub fn new(time: f32, multiplier: f32) -> Self {
        ScrollVelocity {
            time,
            multiplier,
            groups: Some(vec!["$1".to_string(), "$2".to_string(), "$3".to_string(), "$4".to_string()]),
        }
    }
}

impl Default for ScrollVelocity {
    fn default() -> Self {
        Self {
            time: 0.0,
            multiplier: 1.0,
            groups: Some(vec!["$1".to_string(), "$2".to_string(), "$3".to_string(), "$4".to_string()]),
        }
    }
}