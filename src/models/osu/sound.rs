use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum SampleSet {
    Normal,
    Soft,
    Drum,
}

impl FromStr for SampleSet {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Normal" => Ok(SampleSet::Normal),
            "Soft" => Ok(SampleSet::Soft),
            "Drum" => Ok(SampleSet::Drum),
            _ => Err(format!("Invalid SampleSet: {}", s)),
        }
    }
}

impl ToString for SampleSet {
    fn to_string(&self) -> String {
        match self {
            SampleSet::Normal => "Normal".to_string(),
            SampleSet::Soft => "Soft".to_string(),
            SampleSet::Drum => "Drum".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HitSample {
    pub normal_set: u8,
    
    pub addition_set: u8,
    
    pub index: usize,
    
    pub volume: u8,
    
    pub filename: String,
}

impl Default for HitSample {
    fn default() -> Self {
        Self {
            normal_set: 0,
            addition_set: 0,
            index: 0,
            volume: 0,
            filename: String::new(),
        }
    }
}

impl FromStr for HitSample {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        
        if parts.len() < 4 {
            return Ok(HitSample::default());
        }
        
        let normal_set = parts[0].parse::<u8>().unwrap_or(0);
        let addition_set = parts[1].parse::<u8>().unwrap_or(0);
        let index = parts[2].parse::<usize>().unwrap_or(0);
        let volume = parts[3].parse::<u8>().unwrap_or(0);
        let filename = if parts.len() > 4 {
            parts[4].to_string()
        } else {
            String::new()
        };
        
        Ok(HitSample {
            normal_set,
            addition_set,
            index,
            volume,
            filename,
        })
    }
}

impl HitSample {
    pub fn to_osu_format(&self) -> String {
        format!("{}:{}:{}:{}:{}", 
                self.normal_set, 
                self.addition_set, 
                self.index, 
                self.volume, 
                self.filename)
    }
}
