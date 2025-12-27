use serde::{Deserialize, Serialize};
use crate::models::quaver::{
    editor::*,
    sound::*,
    timing_points::*,
    hitobjects::*,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct QuaFile {
    #[serde(rename = "AudioFile")]
    pub audio_file: String,
    
    #[serde(rename = "SongPreviewTime")]
    pub song_preview_time: i32,
    
    #[serde(rename = "BackgroundFile")]
    pub background_file: String,

    #[serde(rename = "MapId", default = "default_id")]
    pub map_id: i32,

    #[serde(rename = "MapSetId", default = "default_id")]
    pub mapset_id: i32,
    
    #[serde(rename = "Mode")]
    pub mode: String,
    
    #[serde(rename = "Title")]
    pub title: String,
    
    #[serde(rename = "Artist")]
    pub artist: String,
    
    #[serde(rename = "Source")]
    pub source: String,
    
    #[serde(rename = "Tags")]
    pub tags: String,
    
    #[serde(rename = "Creator")]
    pub creator: String,
    
    #[serde(rename = "DifficultyName")]
    pub difficulty_name: String,
    
    #[serde(rename = "BPMDoesNotAffectScrollVelocity", default = "default_bpm_does_not_affect_scroll_velocity")]
    pub bpm_does_not_affect_scroll_velocity: bool,
    
    #[serde(rename = "InitialScrollVelocity", default = "default_initial_scroll_velocity")]
    pub initial_scroll_velocity: f32,

    #[serde(rename = "HasScratchKey", default = "default_has_scratch_key")]
    pub has_scratch_key: bool,
    
    #[serde(rename = "EditorLayers")]
    pub editor_layers: Vec<EditorLayer>,
    
    #[serde(rename = "CustomAudioSamples")]
    pub custom_audio_samples: Vec<AudioSample>,

    #[serde(rename = "SoundEffects")]
    pub sound_effects: Vec<SoundEffect>,

    #[serde(rename = "TimingPoints")]
    pub timing_points: Vec<TimingPoint>,

    #[serde(rename = "SliderVelocities")]
    pub slider_velocities: Vec<SliderVelocity>,

    #[serde(rename = "HitObjects")]
    pub hitobjects: Vec<HitObject>,
}

impl QuaFile {
    pub fn to_str(&self) -> Result<String, serde_yaml_ng::Error> {
        serde_yaml_ng::to_string(self)
    }

    pub fn from_str(yaml: &str) -> Result<Self, serde_yaml_ng::Error> {
        serde_yaml_ng::from_str(yaml)
    }
}

fn default_id() -> i32 { -1i32 }
fn default_bpm_does_not_affect_scroll_velocity() -> bool { true }
fn default_initial_scroll_velocity() -> f32 { 1f32 }
fn default_has_scratch_key() -> bool { false }

impl Default for QuaFile {
    fn default() -> Self {
        Self {
            audio_file: String::new(),
            song_preview_time: 0,
            background_file: String::new(),
            map_id: default_id(),
            mapset_id: default_id(),
            mode: String::new(),
            title: String::new(),
            artist: String::new(),
            source: String::new(),
            tags: String::new(),
            creator: String::new(),
            difficulty_name: String::new(),
            bpm_does_not_affect_scroll_velocity: default_bpm_does_not_affect_scroll_velocity(),
            initial_scroll_velocity: default_initial_scroll_velocity(),
            has_scratch_key: default_has_scratch_key(),
            editor_layers: Vec::new(),
            custom_audio_samples: Vec::new(),
            sound_effects: Vec::new(),
            timing_points: Vec::new(),
            slider_velocities: Vec::new(),
            hitobjects: Vec::new(),
        }
    }
}