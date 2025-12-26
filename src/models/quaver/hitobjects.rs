use serde::{Deserialize, Serialize};
use crate::models::quaver::sound::KeySound;
use crate::models::generic::sound;
use crate::utils::serde::trim_float;

#[derive(Debug, Deserialize, Serialize)]
pub struct HitObject {
    #[serde(rename = "StartTime", serialize_with = "trim_float")]
    pub start_time: f32,
    
    #[serde(rename = "Lane")]
    pub lane: u8,
    
    #[serde(rename = "EndTime", skip_serializing_if = "Option::is_none")]
    pub endtime: Option<f32>,

    #[serde(rename = "HitSound", skip_serializing_if = "Option::is_none")]
    pub hit_sound: Option<String>,
    
    #[serde(rename = "KeySounds")]
    pub key_sounds: Vec<KeySound>,
}

impl Default for HitObject {
    fn default() -> Self {
        Self {
            start_time: 0.0,
            lane: 0,
            endtime: None,
            hit_sound: None,
            key_sounds: Vec::new()
        }
    }
}

impl HitObject {
    pub fn start_time(&self) -> f32 {
        self.start_time
    }

    pub fn end_time(&self) -> Option<f32> {
        self.endtime
    }

    pub fn lane(&self) -> u8 {
        self.lane
    }
    
    pub fn is_ln(&self) -> bool {
        self.end_time().is_some()
    }

    pub fn key_sounds(&self) -> &Vec<KeySound> {
        &self.key_sounds
    }
    
    pub fn hit_sound(&self) -> Option<&str> {
        self.hit_sound.as_deref()
    }

    pub fn get_generic_keysound(&self) -> sound::KeySound {
        let hitsound_type = match self.hit_sound().unwrap_or("").to_lowercase().as_str() {
            "clap" => sound::HitSoundType::Clap,
            "whistle" => sound::HitSoundType::Whistle,
            "finish" => sound::HitSoundType::Finish,
            _ => sound::HitSoundType::Normal,
        };

        if self.key_sounds().is_empty() {
            sound::KeySound::of_type(100, hitsound_type)
        } else {
            let key_sound = self.key_sounds().first().unwrap();
            sound::KeySound::with_custom(key_sound.volume.clamp(0, 100), key_sound.sample, Some(hitsound_type))
        }
    }
}