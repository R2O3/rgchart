use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Metadata {
    pub title: String,
    
    pub title_unicode: String,
    
    pub artist: String,
    
    pub artist_unicode: String,
    
    pub creator: String,
    
    pub version: String,
    
    pub source: String,
    
    pub tags: Vec<String>,
    
    pub beatmap_id: i32,
    
    pub beatmap_set_id: i32,
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            title: String::new(),
            title_unicode: String::new(),
            artist: String::new(),
            artist_unicode: String::new(),
            creator: String::new(),
            version: String::new(),
            source: String::new(),
            tags: Vec::new(),
            beatmap_id: 0,
            beatmap_set_id: 0,
        }
    }
}

impl FromStr for Metadata {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut metadata = Metadata::default();
        
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
        
        if let Some(value) = key_values.get("Title") {
            metadata.title = value.clone();
        }
        
        if let Some(value) = key_values.get("TitleUnicode") {
            metadata.title_unicode = value.clone();
        }
        
        if let Some(value) = key_values.get("Artist") {
            metadata.artist = value.clone();
        }
        
        if let Some(value) = key_values.get("ArtistUnicode") {
            metadata.artist_unicode = value.clone();
        }
        
        if let Some(value) = key_values.get("Creator") {
            metadata.creator = value.clone();
        }
        
        if let Some(value) = key_values.get("Version") {
            metadata.version = value.clone();
        }
        
        if let Some(value) = key_values.get("Source") {
            metadata.source = value.clone();
        }
        
        if let Some(value) = key_values.get("Tags") {
            metadata.tags = value
                .split_whitespace()
                .map(|tag| tag.to_string())
                .collect();
        }
        
        if let Some(value) = key_values.get("BeatmapID") {
            metadata.beatmap_id = value.parse::<i32>()
                .map_err(|_| format!("Invalid BeatmapID value: {}", value))?;
        }
        
        if let Some(value) = key_values.get("BeatmapSetID") {
            metadata.beatmap_set_id = value.parse::<i32>()
                .map_err(|_| format!("Invalid BeatmapSetID value: {}", value))?;
        }
        
        Ok(metadata)
    }
}

impl Metadata {
    pub fn new(
        title: String,
        title_unicode: String,
        artist: String,
        artist_unicode: String,
        creator: String,
        version: String,
        source: String,
        tags: Vec<String>,
        beatmap_id: i32,
        beatmap_set_id: i32,
    ) -> Self {
        Self {
            title,
            title_unicode,
            artist,
            artist_unicode,
            creator,
            version,
            source,
            tags,
            beatmap_id,
            beatmap_set_id,
        }
    }
    
    pub fn display_title(&self) -> &str {
        if !self.title_unicode.is_empty() {
            &self.title_unicode
        } else {
            &self.title
        }
    }
    
    pub fn display_artist(&self) -> &str {
        if !self.artist_unicode.is_empty() {
            &self.artist_unicode
        } else {
            &self.artist
        }
    }
    
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }
    
    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.retain(|t| t != tag);
    }
    
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t.eq_ignore_ascii_case(tag))
    }
    
    pub fn tags_string(&self) -> String {
        self.tags.join(" ")
    }
    
    pub fn set_tags_from_string(&mut self, tags_str: &str) {
        self.tags = tags_str
            .split_whitespace()
            .map(|tag| tag.to_string())
            .collect();
    }
    
    pub fn is_submitted(&self) -> bool {
        self.beatmap_id > 0 && self.beatmap_set_id > 0
    }
    
    pub fn has_unicode(&self) -> bool {
        !self.title_unicode.is_empty() || !self.artist_unicode.is_empty()
    }
    
    pub fn to_str(&self) -> String {
        let mut lines = Vec::new();
        
        lines.push(format!("Title: {}", self.title));
        lines.push(format!("TitleUnicode: {}", self.title_unicode));
        lines.push(format!("Artist: {}", self.artist));
        lines.push(format!("ArtistUnicode: {}", self.artist_unicode));
        lines.push(format!("Creator: {}", self.creator));
        lines.push(format!("Version: {}", self.version));
        lines.push(format!("Source: {}", self.source));
        lines.push(format!("Tags: {}", self.tags_string()));
        lines.push(format!("BeatmapID: {}", self.beatmap_id));
        lines.push(format!("BeatmapSetID: {}", self.beatmap_set_id));
        
        lines.join("\n")
    }
}