use std::str::FromStr;
use std::ops::{Index, IndexMut};
use std::slice::SliceIndex;
use crate::models::osu::sound::HitSample;
use crate::models::common::{self, Key};
use crate::models::generic::sound::{self, SoundBank};

#[derive(Debug, Clone, PartialEq)]
pub enum OsuMode {
    Standard,
    Taiko,
    Catch,
    Mania,
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HitObject {
    pub x: i32,
    
    pub y: i32,
    
    pub time: f32,
    
    pub object_type: u8,
    
    pub hit_sound: u8,
    
    pub object_params: Vec<String>,
    
    pub hit_sample: HitSample,
}

impl FromStr for HitObject {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_with_mode(s, &OsuMode::Standard)
    }
}

impl Default for HitObject {
    fn default() -> Self {
        HitObject { x: 256, y: 192, time: 0.0, object_type: 1, hit_sound: 0, object_params: Vec::new(), hit_sample: HitSample::default() }
    }
}

impl HitObject {
    pub fn from_str_with_mode(s: &str, mode: &OsuMode) -> Result<Self, String> {
        let parts: Vec<&str> = s.split(',').collect();
        
        if parts.len() < 5 {
            return Err(format!("Expected at least 5 comma-separated values, found {}", parts.len()));
        }
        
        let x = parts[0].parse::<i32>()
            .map_err(|_| format!("Invalid x value: {}", parts[0]))?;
            
        let y = parts[1].parse::<i32>()
            .map_err(|_| format!("Invalid y value: {}", parts[1]))?;
            
        let time = parts[2].parse::<f32>()
            .map_err(|_| format!("Invalid time value: {}", parts[2]))?;
            
        let object_type = parts[3].parse::<u8>()
            .map_err(|_| format!("Invalid type value: {}", parts[3]))?;
            
        let hit_sound = parts[4].parse::<u8>()
            .map_err(|_| format!("Invalid hitSound value: {}", parts[4]))?;
        
        let mut object_params = Vec::new();
        let mut hit_sample = HitSample::default();
        
        if parts.len() > 5 {
            let is_mania_hold = matches!(mode, OsuMode::Mania) && (object_type & 128) != 0;
            
            if is_mania_hold {
                if parts.len() >= 6 {
                    let sixth_param = parts[5];
                    if sixth_param.contains(':') {
                        let param_parts: Vec<&str> = sixth_param.split(':').collect();
                        if !param_parts.is_empty() {
                            object_params.push(param_parts[0].to_string());
                            
                            if param_parts.len() > 1 {
                                let hit_sample_part = param_parts[1..].join(":");
                                hit_sample = HitSample::from_str(&hit_sample_part)?;
                            }
                        }
                    } else {
                        object_params.push(sixth_param.to_string());
                    }
                }
            } else {
                let last_part = parts[parts.len() - 1];
                if last_part.contains(':') {
                    hit_sample = HitSample::from_str(last_part)?;
                    object_params = parts[5..parts.len()-1].iter()
                        .map(|s| s.to_string())
                        .collect();
                } else {
                    object_params = parts[5..].iter()
                        .map(|s| s.to_string())
                        .collect();
                }
            }
        }
        
        Ok(HitObject {
            x,
            y,
            time,
            object_type,
            hit_sound,
            object_params,
            hit_sample,
        })
    }

    pub fn new(x: i32, y: i32, time: f32, object_type: u8, hit_sound: u8) -> Self {
        Self {
            x,
            y,
            time,
            object_type,
            hit_sound,
            object_params: Vec::new(),
            hit_sample: HitSample::default(),
        }
    }
    
    pub fn is_hit_circle(&self) -> bool {
        (self.object_type & 1) != 0
    }
    
    pub fn is_slider(&self) -> bool {
        (self.object_type & 2) != 0
    }
    
    pub fn is_new_combo(&self) -> bool {
        (self.object_type & 4) != 0
    }
    
    pub fn is_spinner(&self) -> bool {
        (self.object_type & 8) != 0
    }

    pub fn is_normal(&self) -> bool {
        (self.object_type & 1) == 1
    }
    
    pub fn is_hold(&self) -> bool {
        (self.object_type & 128) != 0
    }
    
    pub fn has_normal(&self) -> bool {
        (self.hit_sound & 1) != 0
    }
    
    pub fn has_whistle(&self) -> bool {
        (self.hit_sound & 2) != 0
    }
    
    pub fn has_finish(&self) -> bool {
        (self.hit_sound & 4) != 0
    }
    
    pub fn has_clap(&self) -> bool {
        (self.hit_sound & 8) != 0
    }

    pub fn combo_skip(&self) -> u8 {
        (self.object_type >> 4) & 7
    }
    
    pub fn end_time(&self) -> Option<i32> {
        if self.is_spinner() || self.is_hold() {
            if !self.object_params.is_empty() {
                return self.object_params[0].parse::<i32>().ok();
            }
        }
        None
    }
    
    pub fn slider_params(&self) -> Option<(String, Vec<String>, i32, f32)> {
        if self.is_slider() && self.object_params.len() >= 3 {
            let curve_info = &self.object_params[0];
            let curve_parts: Vec<&str> = curve_info.split('|').collect();
            
            if curve_parts.is_empty() {
                return None;
            }
            
            let curve_type = curve_parts[0].to_string();
            let curve_points: Vec<String> = curve_parts[1..].iter()
                .map(|s| s.to_string())
                .collect();
            
            let slides = self.object_params[1].parse::<i32>().ok()?;
            let length = self.object_params[2].parse::<f32>().ok()?;
            
            Some((curve_type, curve_points, slides, length))
        } else {
            None
        }
    }
    
    pub fn slider_edge_sounds(&self) -> Vec<i32> {
        if self.is_slider() && self.object_params.len() >= 4 {
            self.object_params[3]
                .split('|')
                .filter_map(|s| s.parse::<i32>().ok())
                .collect()
        } else {
            Vec::new()
        }
    }
    
    pub fn slider_edge_sets(&self) -> Vec<(i32, i32)> {
        if self.is_slider() && self.object_params.len() >= 5 {
            self.object_params[4]
                .split('|')
                .filter_map(|s| {
                    let parts: Vec<&str> = s.split(':').collect();
                    if parts.len() >= 2 {
                        let normal = parts[0].parse::<i32>().ok()?;
                        let addition = parts[1].parse::<i32>().ok()?;
                        Some((normal, addition))
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            Vec::new()
        }
    }
    
    pub fn mania_column(&self, column_count: u8) -> u8 {
        let column = (self.x * (column_count as i32) / 512).max(0).min((column_count as i32) - 1);
        column as u8
    }
    
    pub fn to_taiko(&self) -> common::TaikoHitobject {
        use common::TaikoHitobject;
        
        let is_bonus = self.has_finish();
        let is_kat = self.has_whistle() || self.has_clap();
        let is_slider = self.is_slider();
        let is_spinner = self.is_spinner();
        
        match (is_slider, is_spinner, is_bonus, is_kat) {
            (true, _, true, _) => TaikoHitobject::bonus_drum_roll(self.end_time().unwrap_or(0)),
            (true, _, false, _) => TaikoHitobject::drum_roll(self.end_time().unwrap_or(0)),
            (_, true, _, _) => TaikoHitobject::balloon(self.end_time().unwrap_or(0)),
            (false, false, true, false) => TaikoHitobject::bonus_don(),
            (false, false, true, true) => TaikoHitobject::bonus_kat(),
            (false, false, false, false) => TaikoHitobject::don(),
            (false, false, false, true) => TaikoHitobject::kat(),
        }
    }
    
    pub fn to_catch(&self) -> common::CatchHitobject {
        use common::CatchHitobject;

        if self.is_hit_circle() {
            CatchHitobject::fruit(self.x)
        } else if self.is_slider() {
            CatchHitobject::juice(self.x)
        } else if self.is_spinner() {
            CatchHitobject::banana(self.x, self.end_time().unwrap_or(0))
        } else {
            CatchHitobject::empty()
        }
    }
    
    pub fn to_mania(&self, column_count: u8) -> (Key, u8) {
        let column = self.mania_column(column_count);
        let object_type = if self.is_normal() {
            Key::normal()
        } else if self.is_hold() {
            Key::slider_start(Some(self.end_time().unwrap_or(0)))
        } else {
            Key::empty()
        };
        
        (object_type, column)
    }

    pub fn get_generic_keysound(&self, soundbank: &mut SoundBank) -> sound::KeySound {
        let hitsound_type = match self.hit_sound {
            3 => sound::HitSoundType::Clap,
            1 => sound::HitSoundType::Whistle,
            2 => sound::HitSoundType::Finish,
            _ => sound::HitSoundType::Normal,
        };

        if self.hit_sample.filename.trim().is_empty() {
            sound::KeySound::of_type(100, hitsound_type)
        } else {
            let keysound_idx = soundbank.add_sound_sample(self.hit_sample.filename.clone());
            sound::KeySound::with_custom(self.hit_sample.volume.clamp(0, 100), keysound_idx, Some(hitsound_type))
        }
    }
    
    pub fn to_osu_format(&self) -> String {
        let mut parts = vec![
            self.x.to_string(),
            self.y.to_string(),
            self.time.to_string(),
            self.object_type.to_string(),
            self.hit_sound.to_string(),
        ];
        
        parts.extend(self.object_params.iter().cloned());
        
        let hit_sample_str = self.hit_sample.to_osu_format();
        parts.push(hit_sample_str);
        
        parts.join(",")
    }
}

impl HitObject {
    pub fn to_osu_format_with_mode(&self, mode: &OsuMode, soundbank: Option<&mut SoundBank>) -> String {
        match mode {
            OsuMode::Standard => self.to_osu_format_standard(),
            OsuMode::Taiko => self.to_osu_format_taiko(),
            OsuMode::Catch => self.to_osu_format_catch(),
            OsuMode::Mania => {
                if let Some(sb) = soundbank {
                    self.to_osu_format_mania(sb)
                } else {
                    self.to_osu_format_mania_no_soundbank()
                }
            },
            OsuMode::Unknown => self.to_osu_format(),
        }
    }

    fn to_osu_format_standard(&self) -> String {
        let mut parts = vec![
            self.x.to_string(),
            self.y.to_string(),
            self.time.to_string(),
            self.object_type.to_string(),
            self.hit_sound.to_string(),
        ];
        
        if self.is_slider() {
            parts.extend(self.object_params.iter().cloned());
        } else if self.is_spinner() {
            if let Some(end_time) = self.end_time() {
                parts.push(end_time.to_string());
            }
        }
        
        let hit_sample_str = self.hit_sample.to_osu_format();
        parts.push(hit_sample_str);
        
        parts.join(",")
    }

    fn to_osu_format_taiko(&self) -> String {
        let taiko_obj = self.to_taiko();
        let mut parts = vec![
            "256".to_string(),
            "192".to_string(),
            self.time.to_string(),
        ];

        match taiko_obj.note_type {
            common::TaikoHitobjectType::Don => {
                parts.push("1".to_string());
                parts.push("0".to_string());
            },
            common::TaikoHitobjectType::Kat => {
                parts.push("1".to_string());
                parts.push("2".to_string());
            },
            common::TaikoHitobjectType::BonusDon => {
                parts.push("1".to_string());
                parts.push("4".to_string());
            },
            common::TaikoHitobjectType::BonusKat => {
                parts.push("1".to_string());
                parts.push("6".to_string());
            },
            common::TaikoHitobjectType::DrumRoll => {
                parts.push("2".to_string());
                parts.push("0".to_string());
                let end_time = taiko_obj.end_time.unwrap_or(self.time as i32);
                parts.push(format!("L|256:192,1,{}", end_time - self.time as i32));
            },
            common::TaikoHitobjectType::BonusDrumRoll => {
                parts.push("2".to_string());
                parts.push("4".to_string());
                let end_time = taiko_obj.end_time.unwrap_or(self.time as i32);
                parts.push(format!("L|256:192,1,{}", end_time - self.time as i32));
            },
            common::TaikoHitobjectType::Balloon => {
                parts.push("8".to_string());
                parts.push("0".to_string());
                let end_time = taiko_obj.end_time.unwrap_or(self.time as i32);
                parts.push(end_time.to_string());
            },
            common::TaikoHitobjectType::Empty | common::TaikoHitobjectType::Unknown => {
                return String::new();
            }
        }

        let hit_sample_str = self.hit_sample.to_osu_format();
        parts.push(hit_sample_str);

        return parts.join(",");
    }

    fn to_osu_format_catch(&self) -> String {
        let catch_obj = self.to_catch();
        let mut parts = vec![
            self.x.to_string(),
            "192".to_string(),
            self.time.to_string(),
        ];

        match catch_obj.object_type {
            common::CatchHitobjectType::Fruit | common::CatchHitobjectType::Hyperfruit => {
                parts.push("1".to_string());
                parts.push(self.hit_sound.to_string());
            },
            common::CatchHitobjectType::Juice => {
                parts.push("2".to_string());
                parts.push(self.hit_sound.to_string());
                if !self.object_params.is_empty() {
                    parts.extend(self.object_params.iter().cloned());
                }
            },
            common::CatchHitobjectType::Banana => {
                parts.push("8".to_string());
                parts.push(self.hit_sound.to_string());
                parts.push(catch_obj.end_time().unwrap_or(0).to_string());
            },
            common::CatchHitobjectType::Empty | common::CatchHitobjectType::Unknown => {
                return String::new();
            },
        }

        let hit_sample_str = self.hit_sample.to_osu_format();
        parts.push(hit_sample_str);

        parts.join(",")
    }

    fn to_osu_format_mania(&self, soundbank: &mut SoundBank) -> String {
        let keysound = self.get_generic_keysound(soundbank);
        let mut parts = vec![
            self.x.to_string(),
            "192".to_string(),
            self.time.to_string(),
            self.object_type.to_string(),
            self.hit_sound.to_string(),
        ];

        let custom_sample = if keysound.has_custom {
            soundbank.get_sound_sample(keysound.sample.unwrap_or(0))
                .unwrap_or_default()
        } else {
            String::new()
        };

        let volume = if keysound.volume >= 100 { 0 } else { keysound.volume };
        let hit_sample = HitSample {
            normal_set: 0,
            addition_set: 0,
            index: 0,
            volume,
            filename: custom_sample,
        };

        if self.is_hold() {
            if let Some(end_time) = self.end_time() {
                let hit_sample_str = hit_sample.to_osu_format();
                parts.push(format!("{}:{}", end_time, hit_sample_str));
            }
        } else {
            let hit_sample_str = hit_sample.to_osu_format();
            parts.push(hit_sample_str);
        }

        parts.join(",")
    }

    fn to_osu_format_mania_no_soundbank(&self) -> String {
        let mut parts = vec![
            self.x.to_string(),
            "192".to_string(),
            self.time.to_string(),
            self.object_type.to_string(),
            self.hit_sound.to_string(),
        ];

        if self.is_hold() {
            if let Some(end_time) = self.end_time() {
                let hit_sample_str = self.hit_sample.to_osu_format();
                parts.push(format!("{}:{}", end_time, hit_sample_str));
            }
        } else {
            let hit_sample_str = self.hit_sample.to_osu_format();
            parts.push(hit_sample_str);
        }

        parts.join(",")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HitObjects {
    hit_objects: Vec<HitObject>,
}

impl Default for HitObjects {
    fn default() -> Self {
        Self {
            hit_objects: Vec::new(),
        }
    }
}

impl FromStr for HitObjects {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_with_mode(s, &OsuMode::Standard)
    }
}

impl HitObjects {
    pub fn from_str_with_mode(s: &str, mode: &OsuMode) -> Result<Self, String> {
        let mut hit_objects = Vec::new();
        
        for line in s.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("//") {
                continue;
            }
            
            let hit_object = HitObject::from_str_with_mode(line, mode)?;
            hit_objects.push(hit_object);
        }
        
        Ok(HitObjects { hit_objects })
    }

    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn add_hit_object(&mut self, hit_object: HitObject) {
        self.hit_objects.push(hit_object);
    }
    
    pub fn count(&self) -> usize {
        self.hit_objects.len()
    }
    
    pub fn hit_circle_count(&self) -> usize {
        self.hit_objects.iter().filter(|obj| obj.is_hit_circle()).count()
    }
    
    pub fn slider_count(&self) -> usize {
        self.hit_objects.iter().filter(|obj| obj.is_slider()).count()
    }

    pub fn spinner_count(&self) -> usize {
        self.hit_objects.iter().filter(|obj| obj.is_spinner()).count()
    }
    
    pub fn start_time(&self) -> Option<f32> {
        self.hit_objects.first().map(|obj| obj.time)
    }
    
    pub fn end_time(&self) -> Option<i32> {
        self.hit_objects.last().and_then(|obj| {
            obj.end_time().or(Some(obj.time as i32))
        })
    }
    
    pub fn length(&self) -> Option<f32> {
        if let (Some(start), Some(end)) = (self.start_time(), self.end_time()) {
            Some(end as f32 - start)
        } else {
            None
        }
    }
    
    pub fn objects_in_time_range(&self, start_time: f32, end_time: f32) -> Vec<&HitObject> {
        self.hit_objects
            .iter()
            .filter(|obj| obj.time >= start_time && obj.time <= (end_time as f32))
            .collect()
    }

    pub fn sort_by_time(&mut self) {
        self.hit_objects.sort_by(|a, b| {
            match (a.time.is_nan(), b.time.is_nan()) {
                (true, true) => std::cmp::Ordering::Equal,
                (true, false) => std::cmp::Ordering::Greater,
                (false, true) => std::cmp::Ordering::Less,
                (false, false) => a.time.partial_cmp(&b.time).unwrap(),
            }
        });
    }
    
    pub fn to_osu_format(&self) -> String {
        self.hit_objects
            .iter()
            .map(|obj| obj.to_osu_format() )
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl HitObjects {
    pub fn to_osu_format_taiko(&self) -> String {
        self.hit_objects
            .iter()
            .map(|obj| obj.to_osu_format_taiko() )
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn to_osu_format_catch(&self) -> String {
        self.hit_objects
            .iter()
            .map(|obj| obj.to_osu_format_catch() )
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn to_osu_format_mania(&self, soundbank: &mut SoundBank) -> String {
        self.hit_objects
            .iter()
            .map(|obj| obj.to_osu_format_mania(soundbank) )
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn to_osu_format_mania_no_soundbank(&self) -> String {
        self.hit_objects
            .iter()
            .map(|obj| obj.to_osu_format_mania_no_soundbank() )
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn to_osu_format_standard(&self) -> String {
        self.to_osu_format()
    }
}

impl<I> Index<I> for HitObjects
where
    I: SliceIndex<[HitObject]>,
{
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.hit_objects[index]
    }
}

impl<I> IndexMut<I> for HitObjects
where
    I: SliceIndex<[HitObject], Output = [HitObject]>,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.hit_objects[index]
    }
}

impl IntoIterator for HitObjects {
    type Item = HitObject;
    type IntoIter = std::vec::IntoIter<HitObject>;

    fn into_iter(self) -> Self::IntoIter {
        self.hit_objects.into_iter()
    }
}

impl<'a> IntoIterator for &'a HitObjects {
    type Item = &'a HitObject;
    type IntoIter = std::slice::Iter<'a, HitObject>;

    fn into_iter(self) -> Self::IntoIter {
        self.hit_objects.iter()
    }
}

impl<'a> IntoIterator for &'a mut HitObjects {
    type Item = &'a mut HitObject;
    type IntoIter = std::slice::IterMut<'a, HitObject>;

    fn into_iter(self) -> Self::IntoIter {
        self.hit_objects.iter_mut()
    }
}

impl HitObjects {
    pub fn iter(&'_ self) -> std::slice::Iter<'_, HitObject> {
        self.hit_objects.iter()
    }
    
    pub fn iter_mut(&'_ mut self) -> std::slice::IterMut<'_, HitObject> {
        self.hit_objects.iter_mut()
    }
    
    pub fn get(&self, index: usize) -> Option<&HitObject> {
        self.hit_objects.get(index)
    }
    
    pub fn get_mut(&mut self, index: usize) -> Option<&mut HitObject> {
        self.hit_objects.get_mut(index)
    }
    
    pub fn get_slice<I>(&self, index: I) -> Option<&I::Output>
    where
        I: SliceIndex<[HitObject]>,
    {
        self.hit_objects.get(index)
    }
    
    pub fn get_slice_mut<I>(&mut self, index: I) -> Option<&mut I::Output>
    where
        I: SliceIndex<[HitObject], Output = [HitObject]>,
    {
        self.hit_objects.get_mut(index)
    }
    
    pub fn is_empty(&self) -> bool {
        self.hit_objects.is_empty()
    }
    
    pub fn first(&self) -> Option<&HitObject> {
        self.hit_objects.first()
    }
    
    pub fn last(&self) -> Option<&HitObject> {
        self.hit_objects.last()
    }
    
    pub fn first_mut(&mut self) -> Option<&mut HitObject> {
        self.hit_objects.first_mut()
    }
    
    pub fn last_mut(&mut self) -> Option<&mut HitObject> {
        self.hit_objects.last_mut()
    }
}