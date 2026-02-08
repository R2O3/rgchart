use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Difficulty {
    pub hp_drain_rate: f32,
    
    pub circle_size: f32,
    
    pub overall_difficulty: f32,
    
    pub approach_rate: f32,
    
    pub slider_multiplier: f32,
    
    pub slider_tick_rate: f32,
}

impl Default for Difficulty {
    fn default() -> Self {
        Self {
            hp_drain_rate: 5.0,
            circle_size: 5.0,
            overall_difficulty: 5.0,
            approach_rate: 5.0,
            slider_multiplier: 1.4,
            slider_tick_rate: 1.0,
        }
    }
}

impl FromStr for Difficulty {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut difficulty = Difficulty::default();
        
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
        
        if let Some(value) = key_values.get("HPDrainRate") {
            difficulty.hp_drain_rate = value.parse::<f32>()
                .map_err(|_| format!("Invalid HPDrainRate value: {}", value))?;
        }
        
        if let Some(value) = key_values.get("CircleSize") {
            difficulty.circle_size = value.parse::<f32>()
                .map_err(|_| format!("Invalid CircleSize value: {}", value))?;
        }
        
        if let Some(value) = key_values.get("OverallDifficulty") {
            difficulty.overall_difficulty = value.parse::<f32>()
                .map_err(|_| format!("Invalid OverallDifficulty value: {}", value))?;
        }
        
        if let Some(value) = key_values.get("ApproachRate") {
            difficulty.approach_rate = value.parse::<f32>()
                .map_err(|_| format!("Invalid ApproachRate value: {}", value))?;
        }
        
        if let Some(value) = key_values.get("SliderMultiplier") {
            difficulty.slider_multiplier = value.parse::<f32>()
                .map_err(|_| format!("Invalid SliderMultiplier value: {}", value))?;
        }
        
        if let Some(value) = key_values.get("SliderTickRate") {
            difficulty.slider_tick_rate = value.parse::<f32>()
                .map_err(|_| format!("Invalid SliderTickRate value: {}", value))?;
        }
        
        Ok(difficulty)
    }
}

impl Difficulty {
    pub fn new(
        hp_drain_rate: f32,
        circle_size: f32,
        overall_difficulty: f32,
        approach_rate: f32,
        slider_multiplier: f32,
        slider_tick_rate: f32,
    ) -> Self {
        Self {
            hp_drain_rate,
            circle_size,
            overall_difficulty,
            approach_rate,
            slider_multiplier,
            slider_tick_rate,
        }
    }
    
    pub fn is_valid(&self) -> bool {
        let in_range = |val: f32| val >= 0.0 && val <= 10.0;
        
        in_range(self.hp_drain_rate)
            && in_range(self.circle_size)
            && in_range(self.overall_difficulty)
            && in_range(self.approach_rate)
            && self.slider_multiplier > 0.0
            && self.slider_tick_rate > 0.0
    }
    
    pub fn difficulty_rating(&self) -> &'static str {
        let avg = (self.hp_drain_rate + self.circle_size + self.overall_difficulty + self.approach_rate) / 4.0;
        
        match avg {
            x if x < 2.0 => "Easy",
            x if x < 2.7 => "Normal",
            x if x < 4.0 => "Hard",
            x if x < 5.3 => "Insane",
            x if x < 6.5 => "Expert",
            _ => "Expert+",
        }
    }
    
    pub fn circle_size_pixels(&self) -> f32 {
        54.4 - 4.48 * self.circle_size
    }
    
    pub fn approach_rate_ms(&self) -> f32 {
        if self.approach_rate < 5.0 {
            1800.0 - 120.0 * self.approach_rate
        } else {
            1950.0 - 150.0 * self.approach_rate
        }
    }
    
    pub fn od_300_window(&self, overall_difficulty: Option<f32>) -> f32 {
        if overall_difficulty.is_some() {
            return 80.0 - 6.0 * overall_difficulty.unwrap()
        }

        80.0 - 6.0 * self.overall_difficulty
    }
    
    pub fn od_100_window(&self, overall_difficulty: Option<f32>) -> f32 {
        if overall_difficulty.is_some() {
            return 140.0 - 8.0 * overall_difficulty.unwrap()
        }

        140.0 - 8.0 * self.overall_difficulty
    }
    
    pub fn od_50_window(&self, overall_difficulty: Option<f32>) -> f32 {
        if overall_difficulty.is_some() {
            return 200.0 - 10.0 * overall_difficulty.unwrap()
        }

        200.0 - 10.0 * self.overall_difficulty
    }
    
    pub fn to_str(&self) -> String {
        format!(
            "HPDrainRate: {}\nCircleSize: {}\nOverallDifficulty: {}\nApproachRate: {}\nSliderMultiplier: {}\nSliderTickRate: {}",
            self.hp_drain_rate,
            self.circle_size,
            self.overall_difficulty,
            self.approach_rate,
            self.slider_multiplier,
            self.slider_tick_rate
        )
    }
}
