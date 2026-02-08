use std::fmt::{self, Display, Formatter};
use crate::{models::{generic::sound::SoundBank, osu::*}, parsers::osu::from_osu};

#[derive(Debug, Clone, PartialEq)]
pub enum OsuMode {
    Standard,
    Taiko,
    Catch,
    Mania,
    Unknown,
}

impl Display for OsuMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mode_str = match self {
            OsuMode::Standard => "osu!",
            OsuMode::Taiko => "osu!taiko",
            OsuMode::Catch => "osu!catch",
            OsuMode::Mania => "osu!mania",
            OsuMode::Unknown => "unknown",
        };
        write!(f, "{mode_str}")
    }
}

pub struct OsuFile {
    pub general: general::General,
    pub editor: Option<editor::Editor>,
    pub metadata: metadata::Metadata,
    pub difficulty: difficulty::Difficulty,
    pub events: events::Events,
    pub timing_points: timing_points::TimingPoints,
    pub hitobjects: hitobjects::HitObjects,
}

impl Default for OsuFile {
    fn default() -> Self {
        OsuFile {
            general: Default::default(),
            editor: None,
            metadata: Default::default(),
            difficulty: Default::default(),
            events: Default::default(),
            timing_points: Default::default(),
            hitobjects: Default::default(),
        }
    }
}

impl OsuFile {
    pub fn from_str(str: &str) -> Result<Self, Box<dyn std::error::Error>> {
        from_osu(str)
    }
}

impl OsuFile {
    pub fn to_str_taiko(&self) -> String {
        let mut result = String::new();
        result.push_str("osu file format v14\n\n");
        
        result.push_str("[General]\n");
        result.push_str(&self.general.to_str());
        result.push_str("\n\n");
        
        if let Some(ref editor) = self.editor {
            result.push_str("[Editor]\n");
            result.push_str(&editor.to_str());
            result.push_str("\n\n");
        }
        
        result.push_str("[Metadata]\n");
        result.push_str(&self.metadata.to_str());
        result.push_str("\n\n");
        
        result.push_str("[Difficulty]\n");
        result.push_str(&self.difficulty.to_str());
        result.push_str("\n\n");
        
        result.push_str("[Events]\n");
        result.push_str(&self.events.to_str());
        result.push_str("\n");
        
        result.push_str("[TimingPoints]\n");
        result.push_str(&self.timing_points.to_str());
        result.push_str("\n\n");
        
        result.push_str("[HitObjects]\n");
        result.push_str(&self.hitobjects.to_str_taiko());
        
        result
    }

    pub fn to_str_catch(&self) -> String {
        let mut result = String::new();
        result.push_str("osu file format v14\n\n");
        
        result.push_str("[General]\n");
        result.push_str(&self.general.to_str());
        result.push_str("\n\n");
        
        if let Some(ref editor) = self.editor {
            result.push_str("[Editor]\n");
            result.push_str(&editor.to_str());
            result.push_str("\n\n");
        }
        
        result.push_str("[Metadata]\n");
        result.push_str(&self.metadata.to_str());
        result.push_str("\n\n");
        
        result.push_str("[Difficulty]\n");
        result.push_str(&self.difficulty.to_str());
        result.push_str("\n\n");
        
        result.push_str("[Events]\n");
        result.push_str(&self.events.to_str());
        result.push_str("\n");
        
        result.push_str("[TimingPoints]\n");
        result.push_str(&self.timing_points.to_str());
        result.push_str("\n\n");
        
        result.push_str("[HitObjects]\n");
        result.push_str(&self.hitobjects.to_str_catch());
        
        result
    }

    pub fn to_str_mania(&self, soundbank: &mut SoundBank) -> String {
        let mut result = String::new();
        result.push_str("osu file format v14\n\n");
        
        result.push_str("[General]\n");
        result.push_str(&self.general.to_str());
        result.push_str("\n\n");
        
        if let Some(ref editor) = self.editor {
            result.push_str("[Editor]\n");
            result.push_str(&editor.to_str());
            result.push_str("\n\n");
        }
        
        result.push_str("[Metadata]\n");
        result.push_str(&self.metadata.to_str());
        result.push_str("\n\n");
        
        result.push_str("[Difficulty]\n");
        result.push_str(&self.difficulty.to_str());
        result.push_str("\n\n");
        
        result.push_str("[Events]\n");
        result.push_str(&self.events.to_str());
        result.push_str("\n");
        
        result.push_str("[TimingPoints]\n");
        result.push_str(&self.timing_points.to_str());
        result.push_str("\n\n");
        
        result.push_str("[HitObjects]\n");
        result.push_str(&self.hitobjects.to_str_mania(soundbank));
        
        result
    }

    pub fn to_str_mania_no_soundbank(&self) -> String {
        let mut result = String::new();
        result.push_str("osu file format v14\n\n");
        
        result.push_str("[General]\n");
        result.push_str(&self.general.to_str());
        result.push_str("\n\n");
        
        if let Some(ref editor) = self.editor {
            result.push_str("[Editor]\n");
            result.push_str(&editor.to_str());
            result.push_str("\n\n");
        }
        
        result.push_str("[Metadata]\n");
        result.push_str(&self.metadata.to_str());
        result.push_str("\n\n");
        
        result.push_str("[Difficulty]\n");
        result.push_str(&self.difficulty.to_str());
        result.push_str("\n\n");
        
        result.push_str("[Events]\n");
        result.push_str(&self.events.to_str());
        result.push_str("\n");
        
        result.push_str("[TimingPoints]\n");
        result.push_str(&self.timing_points.to_str());
        result.push_str("\n\n");
        
        result.push_str("[HitObjects]\n");
        result.push_str(&self.hitobjects.to_str_no_soundbank());
        
        result
    }

    pub fn to_str_standard(&self) -> String {
        self.to_str()
    }

    pub fn to_str(&self) -> String {
        let mut result = String::new();
        result.push_str("osu file format v14\n\n");
        
        result.push_str("[General]\n");
        result.push_str(&self.general.to_str());
        result.push_str("\n\n");
        
        if let Some(ref editor) = self.editor {
            result.push_str("[Editor]\n");
            result.push_str(&editor.to_str());
            result.push_str("\n\n");
        }
        
        result.push_str("[Metadata]\n");
        result.push_str(&self.metadata.to_str());
        result.push_str("\n\n");
        
        result.push_str("[Difficulty]\n");
        result.push_str(&self.difficulty.to_str());
        result.push_str("\n\n");
        
        result.push_str("[Events]\n");
        result.push_str(&self.events.to_str());
        result.push_str("\n");
        
        result.push_str("[TimingPoints]\n");
        result.push_str(&self.timing_points.to_str());
        result.push_str("\n\n");
        
        result.push_str("[HitObjects]\n");
        result.push_str(&self.hitobjects.to_str());
        
        result
    }
}