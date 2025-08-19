use serde::{Deserialize, Serialize};
use crate::models::generic::sound;
use crate::utils::serde::{is_default_f32};

#[derive(Debug, Serialize, Deserialize)]
pub struct HitObject {
    pub time: f32,
    pub lane: isize,
    
    #[serde(rename = "visual-lane", default, skip_serializing_if = "is_default_f32")]
    pub visual_lane: f32,
    
    #[serde(default, skip_serializing_if = "is_default_f32")]
    pub holdtime: f32,
    
    pub hitsound: String,
    
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    
    #[serde(rename = "type")]
    pub is_tick: i8,
}

impl Default for HitObject {
    fn default() -> Self {
        Self {
            time: 0.0,
            lane: 0,
            visual_lane: 0.0,
            holdtime: 0.0,
            hitsound: ":normal".to_string(),
            group: None,
            is_tick: 0,
        }
    }
}

impl HitObject {
    pub fn is_normal(&self) -> bool {
        self.holdtime <= 0.0 && self.is_tick == 0
    }

    pub fn is_ln(&self) -> bool {
        self.holdtime > 0.0 && self.is_tick == 0
    }
    
    pub fn is_tick(&self) -> bool {
        self.is_tick == 1
    }
    
    pub fn end_time(&self) -> f32 {
        self.time + self.holdtime
    }
    
    pub fn set_end_time(&mut self, end_time: f32) {
        self.holdtime = end_time - self.time;
    }

    pub fn get_generic_keysound(&self) -> sound::KeySound {
        let hitsound_type = match self.hitsound.to_lowercase().as_str() {
            ":clap" => sound::HitSoundType::Clap,
            ":whistle" => sound::HitSoundType::Whistle,
            ":finish" => sound::HitSoundType::Finish,
            _ => sound::HitSoundType::Normal,
        };

        sound::KeySound::of_type(100, hitsound_type)
    }
    
    pub fn new_normal_note(time: f32, lane: isize ) -> Self {
        HitObject {
            time,
            lane,
            visual_lane: 0.0,
            holdtime: 0.0,
            hitsound: ":normal".to_string(),
            group: None,
            is_tick: 0,
        }
    }
    
    pub fn new_long_note(time: f32, lane: isize, holdtime: f32) -> Self {
        HitObject {
            time,
            lane,
            visual_lane: 0.0,
            holdtime,
            hitsound: ":normal".to_string(),
            group: None,
            is_tick: 0,
        }
    }
    
    pub fn new_tick_note(time: f32, lane: isize, visual_lane: f32) -> Self {
        HitObject {
            time,
            lane,
            visual_lane,
            holdtime: 0.0,
            hitsound: ":normal".to_string(),
            group: None,
            is_tick: 1,
        }
    }
}