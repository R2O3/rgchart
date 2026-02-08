use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Editor {
    pub bookmarks: Vec<i32>,
    pub distance_spacing: Option<f32>,
    pub beat_divisor: Option<i32>,
    pub grid_size: Option<i32>,
    pub timeline_zoom: Option<f32>,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            bookmarks: Vec::new(),
            distance_spacing: Some(1f32),
            beat_divisor: Some(4),
            grid_size: Some(4),
            timeline_zoom: Some(0.7),
        }
    }
}

impl FromStr for Editor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut editor = Editor::default();
        
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
        
        if let Some(value) = key_values.get("Bookmarks") {
            editor.bookmarks = value.split(',')
                .filter(|s| !s.is_empty())
                .map(|s| s.trim().parse::<i32>())
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| format!("Invalid Bookmarks value: {}", value))?;
        }
        
        if let Some(value) = key_values.get("DistanceSpacing") {
            editor.distance_spacing = Some(value.parse::<f32>()
                .map_err(|_| format!("Invalid DistanceSpacing value: {}", value))?);
        }
        
        if let Some(value) = key_values.get("BeatDivisor") {
            editor.beat_divisor = Some(value.parse::<i32>()
                .map_err(|_| format!("Invalid BeatDivisor value: {}", value))?);
        }
        
        if let Some(value) = key_values.get("GridSize") {
            editor.grid_size = Some(value.parse::<i32>()
                .map_err(|_| format!("Invalid GridSize value: {}", value))?);
        }
        
        if let Some(value) = key_values.get("TimelineZoom") {
            editor.timeline_zoom = Some(value.parse::<f32>()
                .map_err(|_| format!("Invalid TimelineZoom value: {}", value))?);
        }
        
        Ok(editor)
    }
}

impl Editor {
    pub fn to_str(&self) -> String {
        let mut lines = Vec::new();
        
        if !self.bookmarks.is_empty() {
            let bookmarks_str = self.bookmarks.iter()
                .map(|b| b.to_string())
                .collect::<Vec<_>>()
                .join(",");
            lines.push(format!("Bookmarks: {}", bookmarks_str));
        }
        
        if let Some(spacing) = self.distance_spacing {
            lines.push(format!("DistanceSpacing: {}", spacing));
        }
        
        if let Some(divisor) = self.beat_divisor {
            lines.push(format!("BeatDivisor: {}", divisor));
        }
        
        if let Some(size) = self.grid_size {
            lines.push(format!("GridSize: {}", size));
        }
        
        if let Some(zoom) = self.timeline_zoom {
            lines.push(format!("TimelineZoom: {}", zoom));
        }
        
        lines.join("\n")
    }
}