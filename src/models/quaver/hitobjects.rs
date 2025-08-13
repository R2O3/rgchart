use serde::{Deserialize, Serialize};
use crate::models::quaver::sound::KeySound;
use crate::models::generic::sound;
use crate::utils::serde::trim_float;

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum HitObject {
    NormalNote(NormalNote),
    LongNote(LongNote),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NormalNote {
    #[serde(rename = "StartTime", serialize_with = "trim_float")]
    pub start_time: f32,
    
    #[serde(rename = "Lane")]
    pub lane: usize,
    
    #[serde(rename = "HitSound", skip_serializing_if = "Option::is_none")]
    pub hit_sound: Option<String>,
    
    #[serde(rename = "KeySounds")]
    pub key_sounds: Vec<KeySound>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LongNote {
    #[serde(rename = "StartTime", serialize_with = "trim_float")]
    pub start_time: f32,
    
    #[serde(rename = "Lane")]
    pub lane: usize,
    
    #[serde(rename = "EndTime", serialize_with = "trim_float")]
    pub end_time: f32,
    
    #[serde(rename = "HitSound", skip_serializing_if = "Option::is_none")]
    pub hit_sound: Option<String>,
    
    #[serde(rename = "KeySounds")]
    pub key_sounds: Vec<KeySound>,
}
impl HitObject {
    pub fn start_time(&self) -> f32 {
        match self {
            Self::NormalNote(n) => n.start_time,
            Self::LongNote(n) => n.start_time,
        }
    }

    pub fn end_time(&self) -> Option<f32> {
        match self {
            Self::LongNote(n) => Some(n.end_time),
            _ => None,
        }
    }

    pub fn lane(&self) -> usize {
        match self {
            Self::NormalNote(n) => n.lane,
            Self::LongNote(n) => n.lane,
        }
    }
    
    pub fn is_ln(&self) -> bool {
        matches!(self, Self::LongNote(_))
    }

    pub fn key_sounds(&self) -> &Vec<KeySound> {
        match self {
            HitObject::NormalNote(note) => &note.key_sounds,
            HitObject::LongNote(note) => &note.key_sounds,
        }
    }
    
    pub fn hit_sound(&self) -> Option<&str> {
        match self {
            Self::NormalNote(n) => n.hit_sound.as_deref(),
            Self::LongNote(n) => n.hit_sound.as_deref(),
        }
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