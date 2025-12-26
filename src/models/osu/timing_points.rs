use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub struct TimingPoint {
    pub time: f32,
    pub beat_length: f32,
    pub meter: i32,
    pub sample_set: i32,
    pub sample_index: i32,
    pub volume: i32,
    pub uninherited: bool,
    pub effects: i32,
}

impl TimingPoint {
    pub fn new(
        time: f32,
        beat_length: f32,
        meter: i32,
        sample_set: i32,
        sample_index: i32,
        volume: i32,
        uninherited: bool,
        effects: i32,
    ) -> Self {
        Self {
            time,
            beat_length,
            meter,
            sample_set,
            sample_index,
            volume,
            uninherited,
            effects,
        }
    }
    
    pub fn is_uninherited(&self) -> bool {
        self.uninherited
    }
    
    pub fn is_inherited(&self) -> bool {
        !self.uninherited
    }
    
    pub fn bpm(&self) -> Option<f32> {
        if self.is_uninherited() && self.beat_length > 0.0 {
            Some(60000.0 / self.beat_length)
        } else {
            None
        }
    }
    
    pub fn slider_velocity_multiplier(&self) -> Option<f32> {
        if self.is_inherited() && self.beat_length < 0.0 {
            Some(-100.0 / self.beat_length)
        } else {
            None
        }
    }
    
    pub fn to_osu_format(&self) -> String {
        format!(
            "{},{},{},{},{},{},{},{}",
            self.time,
            self.beat_length,
            self.meter,
            self.sample_set,
            self.sample_index,
            self.volume,
            if self.uninherited { 1 } else { 0 },
            self.effects
        )
    }
}

impl Default for TimingPoint {
    fn default() -> Self {
        TimingPoint { time: 0.0, beat_length: 500.0, meter: 4, sample_set: 0, sample_index: 0, volume: 100, uninherited: true, effects: 0 }
    }
}

impl FromStr for TimingPoint {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        
        if parts.len() != 8 {
            return Err(format!("Expected 8 comma-separated values, found {}", parts.len()));
        }
        
        let time = parts[0].parse::<f32>()
            .map_err(|_| format!("Invalid time value: {}", parts[0]))?;
            
        let beat_length = parts[1].parse::<f32>()
            .map_err(|_| format!("Invalid beatLength value: {}", parts[1]))?;
            
        let meter = parts[2].parse::<i32>()
            .map_err(|_| format!("Invalid meter value: {}", parts[2]))?;
            
        let sample_set = parts[3].parse::<i32>()
            .map_err(|_| format!("Invalid sampleSet value: {}", parts[3]))?;
            
        let sample_index = parts[4].parse::<i32>()
            .map_err(|_| format!("Invalid sampleIndex value: {}", parts[4]))?;
            
        let volume = parts[5].parse::<i32>()
            .map_err(|_| format!("Invalid volume value: {}", parts[5]))?;
            
        let uninherited_val = parts[6].parse::<i32>()
            .map_err(|_| format!("Invalid uninherited value: {}", parts[6]))?;
        let uninherited = uninherited_val != 0;
        
        let effects = parts[7].parse::<i32>()
            .map_err(|_| format!("Invalid effects value: {}", parts[7]))?;
        
        Ok(TimingPoint {
            time,
            beat_length,
            meter,
            sample_set,
            sample_index,
            volume,
            uninherited,
            effects,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TimingPoints {
    pub timing_points: Vec<TimingPoint>,
}

impl Default for TimingPoints {
    fn default() -> Self {
        Self {
            timing_points: Vec::new(),
        }
    }
}

impl FromStr for TimingPoints {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut timing_points = Vec::new();
        
        for line in s.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("//") {
                continue;
            }
            
            let timing_point = TimingPoint::from_str(line)?;
            timing_points.push(timing_point);
        }
        
        Ok(TimingPoints { timing_points })
    }
}

impl TimingPoints {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn add_timing_point(&mut self, timing_point: TimingPoint) {
        self.timing_points.push(timing_point);
    }
    
    pub fn count(&self) -> usize {
        self.timing_points.len()
    }
    
    pub fn uninherited_count(&self) -> usize {
        self.timing_points.iter().filter(|tp| tp.is_uninherited()).count()
    }
    
    pub fn inherited_count(&self) -> usize {
        self.timing_points.iter().filter(|tp| tp.is_inherited()).count()
    }
    
    pub fn start_time(&self) -> Option<f32> {
        self.timing_points.first().map(|tp| tp.time)
    }
    
    pub fn end_time(&self) -> Option<f32> {
        self.timing_points.last().map(|tp| tp.time)
    }
    
    pub fn points_in_range(&self, start_time: f32, end_time: f32) -> Vec<&TimingPoint> {
        self.timing_points
            .iter()
            .filter(|tp| tp.time >= start_time && tp.time <= end_time)
            .collect()
    }
    
    pub fn sort_by_time(&mut self) {
        self.timing_points.sort_by(|a, b| {
            match (a.time.is_nan(), b.time.is_nan()) {
                (true, true) => std::cmp::Ordering::Equal,
                (true, false) => std::cmp::Ordering::Greater,
                (false, true) => std::cmp::Ordering::Less,
                (false, false) => a.time.partial_cmp(&b.time).unwrap(),
            }
        });
    }
    
    pub fn get_bpms(&self) -> Vec<f32> {
        self.timing_points
            .iter()
            .filter_map(|tp| tp.bpm())
            .collect()
    }
    
    pub fn get_slider_velocities(&self) -> Vec<f32> {
        self.timing_points
            .iter()
            .filter_map(|tp| tp.slider_velocity_multiplier())
            .collect()
    }
    
    pub fn to_osu_format(&self) -> String {
        self.timing_points
            .iter()
            .map(|tp| tp.to_osu_format())
            .collect::<Vec<_>>()
            .join("\n")
    }
}