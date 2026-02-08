use std::str::FromStr;
use std::collections::HashMap;

use crate::models::common::GameMode;
use crate::models::osu::sound::SampleSet;
use crate::models::osu::chart::OsuMode;

#[derive(Debug, Clone, PartialEq)]
pub enum OverlayPosition {
    NoChange,
    Below,
    Above,
}

impl FromStr for OverlayPosition {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NoChange" => Ok(OverlayPosition::NoChange),
            "Below" => Ok(OverlayPosition::Below),
            "Above" => Ok(OverlayPosition::Above),
            _ => Err(format!("Invalid OverlayPosition: {}", s)),
        }
    }
}

impl ToString for OverlayPosition {
    fn to_string(&self) -> String {
        match self {
            OverlayPosition::NoChange => "NoChange".to_string(),
            OverlayPosition::Below => "Below".to_string(),
            OverlayPosition::Above => "Above".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct General {
    pub audio_filename: String,
    pub audio_lead_in: i32,
    pub preview_time: i32,
    pub countdown: i32,
    pub sample_set: SampleSet,
    pub stack_leniency: f32,
    pub mode: u8,
    pub letterbox_in_breaks: bool,
    pub special_style: bool,
    pub widescreen_storyboard: bool,
    
    pub audio_hash: Option<String>,
    pub story_fire_in_front: Option<bool>,
    pub use_skin_sprites: Option<bool>,
    pub always_show_playfield: Option<bool>,
    pub overlay_position: Option<OverlayPosition>,
    pub skin_preference: Option<String>,
    pub epilepsy_warning: Option<bool>,
    pub countdown_offset: Option<i32>,
    pub samples_match_playback_rate: Option<bool>,
}

impl Default for General {
    fn default() -> Self {
        Self {
            audio_filename: String::new(),
            audio_lead_in: 0,
            preview_time: -1,
            countdown: 1,
            sample_set: SampleSet::Normal,
            stack_leniency: 0.7,
            mode: 0,
            letterbox_in_breaks: false,
            special_style: false,
            widescreen_storyboard: false,
            
            audio_hash: None,
            story_fire_in_front: None,
            use_skin_sprites: None,
            always_show_playfield: None,
            overlay_position: None,
            skin_preference: None,
            epilepsy_warning: None,
            countdown_offset: None,
            samples_match_playback_rate: None,
        }
    }
}

impl FromStr for General {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut general = General::default();
        
        let mut key_values = HashMap::new();
        for line in s.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("//") {
                continue;
            }
            
            if let Some((key, value)) = line.split_once(':') {
                let key = key.trim();
                let value = value.trim();
                key_values.insert(key.to_string(), value.to_string());
            }
        }
        
        if let Some(value) = key_values.get("AudioFilename") {
            general.audio_filename = value.clone();
        }
        
        if let Some(value) = key_values.get("AudioLeadIn") {
            general.audio_lead_in = value.parse::<i32>()
                .map_err(|_| format!("Invalid AudioLeadIn value: {}", value))?;
        }
        
        if let Some(value) = key_values.get("PreviewTime") {
            general.preview_time = value.parse::<i32>()
                .map_err(|_| format!("Invalid PreviewTime value: {}", value))?;
        }
        
        if let Some(value) = key_values.get("Countdown") {
            general.countdown = value.parse::<i32>()
                .map_err(|_| format!("Invalid Countdown value: {}", value))?;
        }
        
        if let Some(value) = key_values.get("SampleSet") {
            general.sample_set = SampleSet::from_str(value)?;
        }
        
        if let Some(value) = key_values.get("StackLeniency") {
            general.stack_leniency = value.parse::<f32>()
                .map_err(|_| format!("Invalid StackLeniency value: {}", value))?;
        }
        
        if let Some(value) = key_values.get("Mode") {
            general.mode = value.parse::<u8>()
                .map_err(|_| format!("Invalid Mode value: {}", value))?;
        }
        
        if let Some(value) = key_values.get("LetterboxInBreaks") {
            general.letterbox_in_breaks = value.parse::<i32>()
                .map_err(|_| format!("Invalid LetterboxInBreaks value: {}", value))? != 0;
        }
        
        if let Some(value) = key_values.get("SpecialStyle") {
            general.special_style = value.parse::<i32>()
                .map_err(|_| format!("Invalid SpecialStyle value: {}", value))? != 0;
        }
        
        if let Some(value) = key_values.get("WidescreenStoryboard") {
            general.widescreen_storyboard = value.parse::<i32>()
                .map_err(|_| format!("Invalid WidescreenStoryboard value: {}", value))? != 0;
        }
        
        if let Some(value) = key_values.get("AudioHash") {
            general.audio_hash = Some(value.clone());
        }
        
        if let Some(value) = key_values.get("StoryFireInFront") {
            general.story_fire_in_front = Some(value.parse::<i32>()
                .map_err(|_| format!("Invalid StoryFireInFront value: {}", value))? != 0);
        }
        
        if let Some(value) = key_values.get("UseSkinSprites") {
            general.use_skin_sprites = Some(value.parse::<i32>()
                .map_err(|_| format!("Invalid UseSkinSprites value: {}", value))? != 0);
        }
        
        if let Some(value) = key_values.get("AlwaysShowPlayfield") {
            general.always_show_playfield = Some(value.parse::<i32>()
                .map_err(|_| format!("Invalid AlwaysShowPlayfield value: {}", value))? != 0);
        }
        
        if let Some(value) = key_values.get("OverlayPosition") {
            general.overlay_position = Some(OverlayPosition::from_str(value)?);
        }
        
        if let Some(value) = key_values.get("SkinPreference") {
            general.skin_preference = Some(value.clone());
        }
        
        if let Some(value) = key_values.get("EpilepsyWarning") {
            general.epilepsy_warning = Some(value.parse::<i32>()
                .map_err(|_| format!("Invalid EpilepsyWarning value: {}", value))? != 0);
        }
        
        if let Some(value) = key_values.get("CountdownOffset") {
            general.countdown_offset = Some(value.parse::<i32>()
                .map_err(|_| format!("Invalid CountdownOffset value: {}", value))?);
        }
        
        if let Some(value) = key_values.get("SamplesMatchPlaybackRate") {
            general.samples_match_playback_rate = Some(value.parse::<i32>()
                .map_err(|_| format!("Invalid SamplesMatchPlaybackRate value: {}", value))? != 0);
        }
        
        Ok(general)
    }
}

impl General {
    pub fn get_mode_str(&self) -> &'static str {
        match self.mode {
            0 => "osu!",
            1 => "osu!taiko",
            2 => "osu!catch",
            3 => "osu!mania",
            _ => "unknown",
        }
    }

    pub fn get_mode_osu(&self) -> OsuMode {
        match self.mode {
            0 => OsuMode::Standard,
            1 => OsuMode::Taiko,
            2 => OsuMode::Catch,
            3 => OsuMode::Mania,
            _ => OsuMode::Unknown,
        }
    }

    pub fn get_mode(&self) -> GameMode {
        match self.mode {
            0 => GameMode::OsuStandard,
            1 => GameMode::Taiko,
            2 => GameMode::Catch,
            3 => GameMode::Mania,
            _ => GameMode::OsuStandard,
        }
    }

    pub fn is_osu_standard(&self) -> bool {
        self.mode == 0
    }

    pub fn is_taiko(&self) -> bool {
        self.mode == 1
    }

    pub fn is_catch(&self) -> bool {
        self.mode == 2
    }

    pub fn is_mania(&self) -> bool {
        self.mode == 3
    }

    pub fn to_str(&self) -> String {
        let mut lines = Vec::new();
        
        lines.push(format!("AudioFilename: {}", self.audio_filename));
        lines.push(format!("AudioLeadIn: {}", self.audio_lead_in));
        lines.push(format!("PreviewTime: {}", self.preview_time));
        lines.push(format!("Countdown: {}", self.countdown));
        lines.push(format!("SampleSet: {}", self.sample_set.to_string()));
        lines.push(format!("StackLeniency: {}", self.stack_leniency));
        lines.push(format!("Mode: {}", self.mode));
        lines.push(format!("LetterboxInBreaks: {}", if self.letterbox_in_breaks { 1 } else { 0 }));
        lines.push(format!("SpecialStyle: {}", if self.special_style { 1 } else { 0 }));
        lines.push(format!("WidescreenStoryboard: {}", if self.widescreen_storyboard { 1 } else { 0 }));
        
        if let Some(ref hash) = self.audio_hash {
            lines.push(format!("AudioHash: {}", hash));
        }
        
        if let Some(story_fire) = self.story_fire_in_front {
            lines.push(format!("StoryFireInFront: {}", if story_fire { 1 } else { 0 }));
        }
        
        if let Some(skin_sprites) = self.use_skin_sprites {
            lines.push(format!("UseSkinSprites: {}", if skin_sprites { 1 } else { 0 }));
        }
        
        if let Some(always_show) = self.always_show_playfield {
            lines.push(format!("AlwaysShowPlayfield: {}", if always_show { 1 } else { 0 }));
        }
        
        if let Some(ref overlay) = self.overlay_position {
            lines.push(format!("OverlayPosition: {}", overlay.to_string()));
        }
        
        if let Some(ref skin) = self.skin_preference {
            lines.push(format!("SkinPreference: {}", skin));
        }
        
        if let Some(epilepsy) = self.epilepsy_warning {
            lines.push(format!("EpilepsyWarning: {}", if epilepsy { 1 } else { 0 }));
        }
        
        if let Some(offset) = self.countdown_offset {
            lines.push(format!("CountdownOffset: {}", offset));
        }
        
        if let Some(samples_match) = self.samples_match_playback_rate {
            lines.push(format!("SamplesMatchPlaybackRate: {}", if samples_match { 1 } else { 0 }));
        }
        
        lines.join("\n")
    }
}