use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::models::fluxis::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DualMode {
    Disabled = 0,
    Enabled = 1,
    Separate = 2,
}

impl Serialize for DualMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            DualMode::Disabled => serializer.serialize_i32(0),
            DualMode::Enabled => serializer.serialize_i32(1),
            DualMode::Separate => serializer.serialize_i32(2),
        }
    }
}

impl<'de> Deserialize<'de> for DualMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        match value {
            0 => Ok(DualMode::Disabled),
            1 => Ok(DualMode::Enabled),
            2 => Ok(DualMode::Separate),
            _ => Err(serde::de::Error::custom(format!("Invalid DualMode value: {}", value))),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FscFile {
    #[serde(rename = "AudioFile")]
    pub audio_file: String,
    
    #[serde(rename = "BackgroundFile")]
    pub background_file: String,
    
    #[serde(rename = "CoverFile")]
    pub cover_file: String,
    
    #[serde(rename = "VideoFile")]
    pub video_file: String,
    
    #[serde(rename = "EffectFile")]
    pub effect_file: String,
    
    #[serde(rename = "StoryboardFile")]
    pub storyboard_file: String,
    
    #[serde(alias = "Metadata", alias = "MetaData", alias = "METADATA")]
    pub metadata: metadata::Metadata,

    pub colors: metadata::Colors,
    
    #[serde(rename = "HitObjects")]
    pub hit_objects: Vec<hitobjects::HitObject>,
    
    #[serde(rename = "TimingPoints")]
    pub timing_points: Vec<timing_points::TimingPoint>,
    
    #[serde(rename = "ScrollVelocities")]
    pub scroll_velocities: Vec<timing_points::ScrollVelocity>,
    
    #[serde(rename = "HitSoundFades")]
    pub hit_sound_fades: Vec<sound::HitSoundFade>,
    
    #[serde(rename = "AccuracyDifficulty", skip_serializing_if = "Option::is_none")]
    pub accuracy_difficulty: Option<f32>,
    
    #[serde(rename = "HealthDifficulty", skip_serializing_if = "Option::is_none")]
    pub health_difficulty: Option<f32>,
    
    #[serde(rename = "dual", skip_serializing_if = "Option::is_none")]
    pub dual_mode: Option<DualMode>,
    
    #[serde(rename = "ls-v2", skip_serializing_if = "Option::is_none")]
    pub new_lane_switch_layout: Option<bool>,
    
    #[serde(rename = "editor-time", skip_serializing_if = "Option::is_none")]
    pub time_in_editor: Option<i32>,
    
    #[serde(rename = "extra-playfields", skip_serializing_if = "Option::is_none")]
    pub extra_playfields: Option<i32>,
}

impl FscFile {
    pub fn from_str(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

impl FscFile {
    pub const MIN_KEYMODE: isize = 1;
    pub const MAX_KEYMODE: isize = -1;
    
    pub fn new() -> Self {
        Self {
            audio_file: String::new(),
            background_file: String::new(),
            cover_file: String::new(),
            video_file: String::new(),
            effect_file: String::new(),
            storyboard_file: String::new(),
            metadata: metadata::Metadata::default(),
            colors: metadata::Colors::default(),
            hit_objects: Vec::new(),
            timing_points: vec![timing_points::TimingPoint::new(0.0, 120.0, 4)],
            scroll_velocities: Vec::new(),
            hit_sound_fades: Vec::new(),
            accuracy_difficulty: Some(8.0),
            health_difficulty: Some(8.0),
            dual_mode: Some(DualMode::Disabled),
            new_lane_switch_layout: Some(false),
            time_in_editor: Some(0),
            extra_playfields: Some(0),
        }
    }
    
    pub fn to_str(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
    
    pub fn is_dual(&self) -> bool {
        self.dual_mode.unwrap_or(DualMode::Disabled) as i32 > DualMode::Disabled as i32
    }
    
    pub fn is_split(&self) -> bool {
        self.dual_mode.unwrap_or(DualMode::Disabled) == DualMode::Separate
    }
    
    pub fn start_time(&self) -> f32 {
        self.hit_objects.first().map(|obj| obj.time).unwrap_or(0.0)
    }
    
    pub fn end_time(&self) -> f32 {
        self.hit_objects
            .iter()
            .map(|obj| obj.end_time())
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0.0)
    }
    
    pub fn max_combo(&self) -> i32 {
        self.hit_objects.iter().fold(0, |acc, obj| {
            acc + 1 + if obj.is_ln() { 1 } else { 0 }
        })
    }
    
    pub fn key_count(&self) -> isize {
        self.hit_objects
            .iter()
            .map(|obj| obj.lane)
            .max()
            .unwrap_or(4)
    }
    
    pub fn validate(&self) -> Result<(), String> {
        if self.hit_objects.is_empty() {
            return Err("Map has no hit objects.".to_string());
        }
        
        if self.timing_points.is_empty() {
            return Err("Map has no timing points.".to_string());
        }
        
        for timing_point in &self.timing_points {
            if timing_point.bpm <= 0.0 {
                return Err("A timing point has an invalid BPM.".to_string());
            }
            
            if timing_point.signature <= 0 {
                return Err("A timing point has an invalid signature.".to_string());
            }
        }
        
        if self.hit_objects.iter().any(|obj| obj.lane < 1) {
            return Err("A hit object in this map is in a lane below 1.".to_string());
        }
        
        let mode = self.key_count();
        
        if mode < Self::MIN_KEYMODE || mode > Self::MAX_KEYMODE {
            return Err(format!("Map has an invalid keymode: {}. Must be between {} and {}.", 
                mode, Self::MIN_KEYMODE, Self::MAX_KEYMODE));
        }
        
        Ok(())
    }
    
    pub fn get_normal_notes(&self) -> Vec<&hitobjects::HitObject> {
        self.hit_objects.iter().filter(|obj| obj.is_normal()).collect()
    }

    pub fn get_long_notes(&self) -> Vec<&hitobjects::HitObject> {
        self.hit_objects.iter().filter(|obj| obj.is_ln()).collect()
    }
    
    pub fn get_tick_notes(&self) -> Vec<&hitobjects::HitObject> {
        self.hit_objects.iter().filter(|obj| obj.is_tick()).collect()
    }
}

impl Default for FscFile {
    fn default() -> Self {
        Self::new()
    }
}