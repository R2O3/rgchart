use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use crate::KeyType;
use crate::models::generic::sound;
use crate::utils::serde::{is_default_f32};

#[derive(Debug, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum FluXisHitType {
    NormalOrHold = 0,
    Tick = 1,
    Landmine = 2,
}

impl FluXisHitType {
    pub fn to_generic(&self) -> KeyType {
        match self {
            FluXisHitType::NormalOrHold => KeyType::Normal,
            FluXisHitType::Tick => KeyType::Tick,
            FluXisHitType::Landmine => KeyType::Mine,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HitObject {
    pub time: f32,
    pub lane: isize,
    
    #[serde(rename = "visual-lane", default, skip_serializing_if = "is_default_f32")]
    pub visual_lane: f32,
    
    #[serde(default, skip_serializing_if = "is_default_f32")]
    pub holdtime: f32,
    
    pub hitsound: String,

    #[serde(default, skip_serializing_if = "is_default_hidden")]
    pub hidden: bool,
    
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    
    #[serde(rename = "type", default = "default_type")]
    pub hit_type: FluXisHitType,
}

impl Default for HitObject {
    fn default() -> Self {
        Self {
            time: 0.0,
            lane: 0,
            visual_lane: 0.0,
            holdtime: 0.0,
            hitsound: ":normal".to_string(),
            hidden: false,
            group: None,
            hit_type: FluXisHitType::NormalOrHold,
        }
    }
}

impl HitObject {
    pub fn is_normal(&self) -> bool {
        self.holdtime <= 0.0 && self.hit_type == FluXisHitType::NormalOrHold
    }

    pub fn is_ln(&self) -> bool {
        self.holdtime > 0.0 && self.hit_type == FluXisHitType::NormalOrHold
    }
    
    pub fn is_tick(&self) -> bool {
        self.hit_type == FluXisHitType::Tick
    }

    pub fn is_landmine(&self) -> bool {
        self.hit_type == FluXisHitType::Landmine
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
            hidden: false,
            group: None,
            hit_type: FluXisHitType::NormalOrHold,
        }
    }
    
    pub fn new_long_note(time: f32, lane: isize, holdtime: f32) -> Self {
        HitObject {
            time,
            lane,
            visual_lane: 0.0,
            holdtime,
            hitsound: ":normal".to_string(),
            hidden: false,
            group: None,
            hit_type: FluXisHitType::NormalOrHold,
        }
    }
    
    pub fn new_tick_note(time: f32, lane: isize, visual_lane: f32) -> Self {
        HitObject {
            time,
            lane,
            visual_lane,
            holdtime: 0.0,
            hitsound: ":normal".to_string(),
            hidden: false,
            group: None,
            hit_type: FluXisHitType::Tick,
        }
    }
}

fn is_default_hidden(value: &bool) -> bool { *value == false }
fn default_type() -> FluXisHitType { FluXisHitType::NormalOrHold }