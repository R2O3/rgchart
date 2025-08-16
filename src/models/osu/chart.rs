use std::fmt::{self, Display, Formatter};
use crate::models::{generic::sound::SoundBank, osu::*};

#[allow(unused)]
#[derive(Debug)]
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

impl OsuFile {
    pub fn to_osu_format_taiko(&self) -> String {
        let mut result = String::new();
        result.push_str("osu file format v14\n\n");
        
        result.push_str("[General]\n");
        result.push_str(&self.general.to_osu_format());
        result.push_str("\n\n");
        
        if let Some(ref editor) = self.editor {
            result.push_str("[Editor]\n");
            result.push_str(&editor.to_osu_format());
            result.push_str("\n\n");
        }
        
        result.push_str("[Metadata]\n");
        result.push_str(&self.metadata.to_osu_format());
        result.push_str("\n\n");
        
        result.push_str("[Difficulty]\n");
        result.push_str(&self.difficulty.to_osu_format());
        result.push_str("\n\n");
        
        result.push_str("[Events]\n");
        result.push_str(&self.events.to_osu_format());
        result.push_str("\n");
        
        result.push_str("[TimingPoints]\n");
        result.push_str(&self.timing_points.to_osu_format());
        result.push_str("\n\n");
        
        result.push_str("[HitObjects]\n");
        result.push_str(&self.hitobjects.to_osu_format_taiko());
        
        result
    }

    pub fn to_osu_format_catch(&self) -> String {
        let mut result = String::new();
        result.push_str("osu file format v14\n\n");
        
        result.push_str("[General]\n");
        result.push_str(&self.general.to_osu_format());
        result.push_str("\n\n");
        
        if let Some(ref editor) = self.editor {
            result.push_str("[Editor]\n");
            result.push_str(&editor.to_osu_format());
            result.push_str("\n\n");
        }
        
        result.push_str("[Metadata]\n");
        result.push_str(&self.metadata.to_osu_format());
        result.push_str("\n\n");
        
        result.push_str("[Difficulty]\n");
        result.push_str(&self.difficulty.to_osu_format());
        result.push_str("\n\n");
        
        result.push_str("[Events]\n");
        result.push_str(&self.events.to_osu_format());
        result.push_str("\n");
        
        result.push_str("[TimingPoints]\n");
        result.push_str(&self.timing_points.to_osu_format());
        result.push_str("\n\n");
        
        result.push_str("[HitObjects]\n");
        result.push_str(&self.hitobjects.to_osu_format_catch());
        
        result
    }

    pub fn to_osu_format_mania(&self, soundbank: &mut SoundBank) -> String {
        let mut result = String::new();
        result.push_str("osu file format v14\n\n");
        
        result.push_str("[General]\n");
        result.push_str(&self.general.to_osu_format());
        result.push_str("\n\n");
        
        if let Some(ref editor) = self.editor {
            result.push_str("[Editor]\n");
            result.push_str(&editor.to_osu_format());
            result.push_str("\n\n");
        }
        
        result.push_str("[Metadata]\n");
        result.push_str(&self.metadata.to_osu_format());
        result.push_str("\n\n");
        
        result.push_str("[Difficulty]\n");
        result.push_str(&self.difficulty.to_osu_format());
        result.push_str("\n\n");
        
        result.push_str("[Events]\n");
        result.push_str(&self.events.to_osu_format());
        result.push_str("\n");
        
        result.push_str("[TimingPoints]\n");
        result.push_str(&self.timing_points.to_osu_format());
        result.push_str("\n\n");
        
        result.push_str("[HitObjects]\n");
        result.push_str(&self.hitobjects.to_osu_format_mania(soundbank));
        
        result
    }

    pub fn to_osu_format_mania_no_soundbank(&self) -> String {
        let mut result = String::new();
        result.push_str("osu file format v14\n\n");
        
        result.push_str("[General]\n");
        result.push_str(&self.general.to_osu_format());
        result.push_str("\n\n");
        
        if let Some(ref editor) = self.editor {
            result.push_str("[Editor]\n");
            result.push_str(&editor.to_osu_format());
            result.push_str("\n\n");
        }
        
        result.push_str("[Metadata]\n");
        result.push_str(&self.metadata.to_osu_format());
        result.push_str("\n\n");
        
        result.push_str("[Difficulty]\n");
        result.push_str(&self.difficulty.to_osu_format());
        result.push_str("\n\n");
        
        result.push_str("[Events]\n");
        result.push_str(&self.events.to_osu_format());
        result.push_str("\n");
        
        result.push_str("[TimingPoints]\n");
        result.push_str(&self.timing_points.to_osu_format());
        result.push_str("\n\n");
        
        result.push_str("[HitObjects]\n");
        result.push_str(&self.hitobjects.to_osu_format_mania_no_soundbank());
        
        result
    }

    pub fn to_osu_format_standard(&self) -> String {
        self.to_osu_format()
    }

    pub fn to_osu_format(&self) -> String {
        let mut result = String::new();
        result.push_str("osu file format v14\n\n");
        
        result.push_str("[General]\n");
        result.push_str(&self.general.to_osu_format());
        result.push_str("\n\n");
        
        if let Some(ref editor) = self.editor {
            result.push_str("[Editor]\n");
            result.push_str(&editor.to_osu_format());
            result.push_str("\n\n");
        }
        
        result.push_str("[Metadata]\n");
        result.push_str(&self.metadata.to_osu_format());
        result.push_str("\n\n");
        
        result.push_str("[Difficulty]\n");
        result.push_str(&self.difficulty.to_osu_format());
        result.push_str("\n\n");
        
        result.push_str("[Events]\n");
        result.push_str(&self.events.to_osu_format());
        result.push_str("\n");
        
        result.push_str("[TimingPoints]\n");
        result.push_str(&self.timing_points.to_osu_format());
        result.push_str("\n\n");
        
        result.push_str("[HitObjects]\n");
        result.push_str(&self.hitobjects.to_osu_format());
        
        result
    }
}