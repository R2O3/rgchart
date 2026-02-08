use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub struct Event {
    pub event_type: String,
    
    pub start_time: i32,
    
    pub event_params: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Background {
    pub filename: String,
    
    pub x_offset: i32,
    
    pub y_offset: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Video {
    pub start_time: i32,
    
    pub filename: String,
    
    pub x_offset: i32,
    
    pub y_offset: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Break {
    pub start_time: i32,
    
    pub end_time: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Sample {
    pub start_time: i32,
    
    pub layer: i32,
    
    pub filename: String,
    
    pub volume: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Events {
    pub background: Option<Background>,
    
    pub video: Option<Video>,
    
    pub breaks: Vec<Break>,
    
    pub samples: Vec<Sample>,
    
    pub raw_events: String,
}

impl Default for Events {
    fn default() -> Self {
        Self {
            background: None,
            video: None,
            breaks: Vec::new(),
            samples: Vec::new(),
            raw_events: String::new(),
        }
    }
}

impl FromStr for Event {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        
        if parts.len() < 2 {
            return Err(format!("Expected at least 2 comma-separated values, found {}", parts.len()));
        }
        
        let event_type = parts[0].trim().to_string();
        
        let start_time = parts[1].parse::<i32>()
            .map_err(|_| format!("Invalid start time value: {}", parts[1]))?;
        
        let event_params = parts[2..].iter()
            .map(|param| param.trim().to_string())
            .collect();
        
        Ok(Event {
            event_type,
            start_time,
            event_params,
        })
    }
}

impl FromStr for Events {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut events_section = Events::default();
        events_section.raw_events = s.to_string();
        
        for line in s.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("//") {
                continue;
            }
            
            let event = Event::from_str(line)?;
            
            match event.event_type.as_str() {
                "0" => {
                    if event.event_params.len() >= 1 {
                        let filename = event.event_params[0]
                            .trim_matches('"')
                            .to_string();
                        
                        let x_offset = if event.event_params.len() >= 2 {
                            event.event_params[1].parse::<i32>().unwrap_or(0)
                        } else {
                            0
                        };
                        
                        let y_offset = if event.event_params.len() >= 3 {
                            event.event_params[2].parse::<i32>().unwrap_or(0)
                        } else {
                            0
                        };
                        
                        events_section.background = Some(Background {
                            filename,
                            x_offset,
                            y_offset,
                        });
                    }
                }
                "1" | "Video" => {
                    if event.event_params.len() >= 1 {
                        let filename = event.event_params[0]
                            .trim_matches('"')
                            .to_string();
                        
                        let x_offset = if event.event_params.len() >= 2 {
                            event.event_params[1].parse::<i32>().unwrap_or(0)
                        } else {
                            0
                        };
                        
                        let y_offset = if event.event_params.len() >= 3 {
                            event.event_params[2].parse::<i32>().unwrap_or(0)
                        } else {
                            0
                        };
                        
                        events_section.video = Some(Video {
                            start_time: event.start_time,
                            filename,
                            x_offset,
                            y_offset,
                        });
                    }
                }
                "2" | "Break" => {
                    if event.event_params.len() >= 1 {
                        let end_time = event.event_params[0].parse::<i32>()
                            .map_err(|_| format!("Invalid break end time: {}", event.event_params[0]))?;
                        
                        events_section.breaks.push(Break {
                            start_time: event.start_time,
                            end_time,
                        });
                    }
                }
                "Sample" => {
                    if event.event_params.len() >= 2 {
                        let layer = event.event_params[0].parse::<i32>()
                            .map_err(|_| format!("Invalid sample layer: {}", event.event_params[0]))?;
                        
                        let filename = event.event_params[1]
                            .trim_matches('"')
                            .to_string();
                        
                        let volume = if event.event_params.len() >= 3 {
                            event.event_params[2].parse::<u8>().unwrap_or(100)
                        } else {
                            100
                        };
                        
                        events_section.samples.push(Sample {
                            start_time: event.start_time,
                            layer,
                            filename,
                            volume,
                        });
                    }
                }
                _ => { }
            }
        }
        
        Ok(events_section)
    }
}

impl Event {
    pub fn new(event_type: String, start_time: i32, event_params: Vec<String>) -> Self {
        Self {
            event_type,
            start_time,
            event_params,
        }
    }
    
    pub fn is_background(&self) -> bool {
        self.event_type == "0"
    }
    
    pub fn is_video(&self) -> bool {
        self.event_type == "1" || self.event_type == "Video"
    }
    
    pub fn is_break(&self) -> bool {
        self.event_type == "2" || self.event_type == "Break"
    }
    
    pub fn is_sample(&self) -> bool {
        self.event_type == "Sample"
    }
    
    pub fn to_str(&self) -> String {
        let mut parts = vec![self.event_type.clone(), self.start_time.to_string()];
        parts.extend(self.event_params.iter().cloned());
        parts.join(",")
    }
}

impl Background {
    pub fn new(filename: String, x_offset: i32, y_offset: i32) -> Self {
        Self {
            filename,
            x_offset,
            y_offset,
        }
    }
    
    pub fn to_str(&self) -> String {
        if self.x_offset == 0 && self.y_offset == 0 {
            format!("0,0,\"{}\"", self.filename)
        } else {
            format!("0,0,\"{}\",{},{}", self.filename, self.x_offset, self.y_offset)
        }
    }
}

impl Video {
    pub fn new(start_time: i32, filename: String, x_offset: i32, y_offset: i32) -> Self {
        Self {
            start_time,
            filename,
            x_offset,
            y_offset,
        }
    }
    
    pub fn to_str(&self) -> String {
        if self.x_offset == 0 && self.y_offset == 0 {
            format!("Video,{},\"{}\"", self.start_time, self.filename)
        } else {
            format!("Video,{},\"{}\",{},{}", self.start_time, self.filename, self.x_offset, self.y_offset)
        }
    }
}

impl Break {
    pub fn new(start_time: i32, end_time: i32) -> Self {
        Self {
            start_time,
            end_time,
        }
    }
    
    pub fn duration(&self) -> i32 {
        self.end_time - self.start_time
    }
    
    pub fn to_str(&self) -> String {
        format!("2,{},{}", self.start_time, self.end_time)
    }
}

impl Sample {
    pub fn new(start_time: i32, layer: i32, filename: String, volume: u8) -> Self {
        Self {
            start_time,
            layer,
            filename,
            volume,
        }
    }
    
    pub fn layer_name(&self) -> &'static str {
        match self.layer {
            0 => "Background",
            1 => "Fail",
            2 => "Pass",
            3 => "Foreground",
            _ => "Unknown",
        }
    }
    
    pub fn to_str(&self) -> String {
        if self.volume == 100 {
            format!("Sample,{},{},\"{}\"", self.start_time, self.layer, self.filename)
        } else {
            format!("Sample,{},{},\"{}\",{}", self.start_time, self.layer, self.filename, self.volume)
        }
    }
}

impl Events {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn set_background(&mut self, filename: String, x_offset: i32, y_offset: i32) {
        self.background = Some(Background::new(filename, x_offset, y_offset));
    }
    
    pub fn set_video(&mut self, start_time: i32, filename: String, x_offset: i32, y_offset: i32) {
        self.video = Some(Video::new(start_time, filename, x_offset, y_offset));
    }
    
    pub fn add_break(&mut self, start_time: i32, end_time: i32) {
        self.breaks.push(Break::new(start_time, end_time));
    }
    
    pub fn add_sample(&mut self, start_time: i32, layer: i32, filename: String, volume: u8) {
        self.samples.push(Sample::new(start_time, layer, filename, volume));
    }
    
    pub fn total_break_time(&self) -> i32 {
        self.breaks.iter().map(|b| b.duration()).sum()
    }
    
    pub fn has_breaks(&self) -> bool {
        !self.breaks.is_empty()
    }
    
    pub fn has_background(&self) -> bool {
        self.background.is_some()
    }
    
    pub fn has_video(&self) -> bool {
        self.video.is_some()
    }
    
    pub fn has_samples(&self) -> bool {
        !self.samples.is_empty()
    }
    
    pub fn samples_by_layer(&self, layer: i32) -> Vec<&Sample> {
        self.samples.iter().filter(|s| s.layer == layer).collect()
    }
    
    pub fn to_str(&self) -> String {
        let mut result = String::new();
        
        result.push_str("//Background and Video events\n");
        
        if let Some(ref bg) = self.background {
            result.push_str(&bg.to_str());
            result.push('\n');
        }
        
        if let Some(ref video) = self.video {
            result.push_str(&video.to_str());
            result.push('\n');
        }
        
        result.push_str("//Break Periods\n");
        for break_event in &self.breaks {
            result.push_str(&break_event.to_str());
            result.push('\n');
        }
        
        result.push_str("//Storyboard Layer 0 (Background)\n");
        for sample in self.samples_by_layer(0) {
            result.push_str(&sample.to_str());
            result.push('\n');
        }
        
        result.push_str("//Storyboard Layer 1 (Fail)\n");
        for sample in self.samples_by_layer(1) {
            result.push_str(&sample.to_str());
            result.push('\n');
        }
        
        result.push_str("//Storyboard Layer 2 (Pass)\n");
        for sample in self.samples_by_layer(2) {
            result.push_str(&sample.to_str());
            result.push('\n');
        }
        
        result.push_str("//Storyboard Layer 3 (Foreground)\n");
        for sample in self.samples_by_layer(3) {
            result.push_str(&sample.to_str());
            result.push('\n');
        }
        
        result.push_str("//Storyboard Layer 4 (Overlay)\n");
        for sample in &self.samples {
            if sample.layer >= 4 {
                result.push_str(&sample.to_str());
                result.push('\n');
            }
        }
        
        result.push_str("//Storyboard Sound Samples\n");
        
        result
    }
}