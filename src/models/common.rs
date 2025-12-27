use std::fmt;
use crate::wasm_bindgen;

def_varied_type_enum!(pub ChartDefaults {
    TITLE: &'static str => "Unknown Title",
    ALT_TITLE: &'static str => "Unknown Title",
    ARTIST: &'static str => "Unknown Artist",
    ALT_ARTIST: &'static str => "Unknown Artist",
    CREATOR: &'static str => "Unknown Creator",
    GENRE: &'static str => "Unknown Genre",
    SOURCE: &'static str => "Unknown Source",
    TAGS: Vec<String> => Vec::<String>::new(),

    BPM: &'static f32 => &0.0,
    DIFFICULTY_NAME: &'static str => "Unknown Difficulty",
    BG_PATH: &'static str => "Unknown Background Path",
    SONG_PATH: &'static str => "Unknown Song File Path",
    AUDIO_OFFSET: &'static i32 => &0,
    PREVIEW_TIME: &'static i32 => &0,
    OVERALL_DIFFICULTY: &'static f32 => &7.2,
    KEY_COUNT: &'static u8 => &4,
    
    RAW_NOTES: &'static str => "No Note Data",
    RAW_BPMS: &'static str => "No BPM Data",
    RAW_STOPS: &'static str => "No STOPS Data",
    RAW_SV: &'static str => "No SV Data",

    HITSOUND: [u8; 4] => [0, 0, 0, 0],
});

#[derive(Clone)]
pub struct HitObjectRow {
    pub time: i32,
    pub beat: f32,
    pub keys: Vec<Key>
}

impl HitObjectRow {
    pub fn empty(time: i32, beat: f32, key_count: usize) -> Self {
        HitObjectRow {
            time,
            beat,
            keys: vec![Key::empty(); key_count]
        }
    }
}

pub type Measure = Vec<HitObjectRow>;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimingChangeType {
    Bpm,
    Sv,
    Stop
}

#[allow(unused)]
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum GameMode {
    Mania,
    Taiko,
    Catch,
    OsuStandard
}

impl fmt::Display for GameMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Mania => write!(f, "mania"),
            Self::Taiko => write!(f, "taiko"),
            Self::Catch => write!(f, "catch"),
            Self::OsuStandard => write!(f, "osu! standard"),
        }
    }
}

pub trait HitObject {
    fn game_mode(&self) -> GameMode;
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyType { 
    Empty,
    Normal,
    SliderStart,
    SliderEnd,
    Mine,
    Fake,
    Unknown,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Key {
    #[wasm_bindgen(getter_with_clone)]
    pub key_type: KeyType,
    #[wasm_bindgen(getter_with_clone)]
    pub slider_end_time: Option<i32>,
}

impl HitObject for Key { 
    fn game_mode(&self) -> GameMode {
        GameMode::Mania
    }
}

#[wasm_bindgen]
impl Key {
    #[wasm_bindgen]
    pub fn empty() -> Self {
        Self {
            key_type: KeyType::Empty,
            slider_end_time: None,
        }
    }

    #[wasm_bindgen]
    pub fn normal() -> Self {
        Self {
            key_type: KeyType::Normal,
            slider_end_time: None,
        }
    }

    #[wasm_bindgen]
    pub fn slider_start(value: Option<i32>) -> Self {
        Self {
            key_type: KeyType::SliderStart,
            slider_end_time: value,
        }
    }

    #[wasm_bindgen]
    pub fn slider_end() -> Self {
        Self {
            key_type: KeyType::SliderEnd,
            slider_end_time: None,
        }
    }

    #[wasm_bindgen]
    pub fn mine() -> Self {
        Self {
            key_type: KeyType::Mine,
            slider_end_time: None,
        }
    }

    #[wasm_bindgen]
    pub fn fake() -> Self {
        Self {
            key_type: KeyType::Fake,
            slider_end_time: None,
        }
    }

    #[wasm_bindgen]
    pub fn unknown() -> Self {
        Self {
            key_type: KeyType::Unknown,
            slider_end_time: None,
        }
    }

    #[wasm_bindgen]
    pub fn slider_end_time(&self) -> Option<i32> {
        self.slider_end_time
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaikoHitobjectType {
    Empty,
    Don,
    Kat,
    BonusDon,
    BonusKat,
    DrumRoll,
    BonusDrumRoll,
    Balloon,
    Unknown,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TaikoHitobject {
    #[wasm_bindgen(getter_with_clone)]
    pub note_type: TaikoHitobjectType,
    #[wasm_bindgen(getter_with_clone)]
    pub end_time: Option<i32>,
}

impl HitObject for TaikoHitobject {
    fn game_mode(&self) -> GameMode {
        GameMode::Taiko
    }
}

#[wasm_bindgen]
impl TaikoHitobject {
    #[wasm_bindgen]
    pub fn empty() -> Self {
        Self {
            note_type: TaikoHitobjectType::Empty,
            end_time: None,
        }
    }

    #[wasm_bindgen]
    pub fn don() -> Self {
        Self {
            note_type: TaikoHitobjectType::Don,
            end_time: None,
        }
    }

    #[wasm_bindgen]
    pub fn kat() -> Self {
        Self {
            note_type: TaikoHitobjectType::Kat,
            end_time: None,
        }
    }

    #[wasm_bindgen]
    pub fn bonus_don() -> Self {
        Self {
            note_type: TaikoHitobjectType::BonusDon,
            end_time: None,
        }
    }

    #[wasm_bindgen]
    pub fn bonus_kat() -> Self {
        Self {
            note_type: TaikoHitobjectType::BonusKat,
            end_time: None,
        }
    }

    #[wasm_bindgen]
    pub fn drum_roll(end_time: i32) -> Self {
        Self {
            note_type: TaikoHitobjectType::DrumRoll,
            end_time: Some(end_time),
        }
    }

    #[wasm_bindgen]
    pub fn bonus_drum_roll(end_time: i32) -> Self {
        Self {
            note_type: TaikoHitobjectType::BonusDrumRoll,
            end_time: Some(end_time),
        }
    }

    #[wasm_bindgen]
    pub fn balloon(end_time: i32) -> Self {
        Self {
            note_type: TaikoHitobjectType::Balloon,
            end_time: Some(end_time),
        }
    }

    #[wasm_bindgen]
    pub fn unknown() -> Self {
        Self {
            note_type: TaikoHitobjectType::Unknown,
            end_time: None,
        }
    }

    #[wasm_bindgen]
    pub fn end_time(&self) -> Option<i32> {
        self.end_time
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CatchHitobjectType {
    Empty,
    Fruit,
    Juice,
    Banana,
    Hyperfruit,
    Unknown,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CatchHitobject {
    #[wasm_bindgen(getter_with_clone)]
    pub object_type: CatchHitobjectType,
    #[wasm_bindgen(getter_with_clone)]
    pub x_position: i32,
    #[wasm_bindgen(getter_with_clone)]
    pub end_time: Option<i32>,
    #[wasm_bindgen(getter_with_clone)]
    pub hyperdash: bool,
}

impl HitObject for CatchHitobject {
    fn game_mode(&self) -> GameMode {
        GameMode::Catch
    }
}

#[wasm_bindgen]
impl CatchHitobject {
    #[wasm_bindgen]
    pub fn empty() -> Self {
        Self {
            object_type: CatchHitobjectType::Empty,
            x_position: 0,
            end_time: None,
            hyperdash: false,
        }
    }

    #[wasm_bindgen]
    pub fn fruit(x_position: i32) -> Self {
        Self {
            object_type: CatchHitobjectType::Fruit,
            x_position,
            end_time: None,
            hyperdash: false,
        }
    }

    #[wasm_bindgen]
    pub fn juice(x_position: i32) -> Self {
        Self {
            object_type: CatchHitobjectType::Juice,
            x_position,
            end_time: None,
            hyperdash: false,
        }
    }

    #[wasm_bindgen]
    pub fn banana(x_position: i32, end_time: i32) -> Self {
        Self {
            object_type: CatchHitobjectType::Banana,
            x_position,
            end_time: Some(end_time),
            hyperdash: false,
        }
    }

    #[wasm_bindgen]
    pub fn hyperfruit(x_position: i32) -> Self {
        Self {
            object_type: CatchHitobjectType::Hyperfruit,
            x_position,
            end_time: None,
            hyperdash: true,
        }
    }

    #[wasm_bindgen]
    pub fn unknown() -> Self {
        Self {
            object_type: CatchHitobjectType::Unknown,
            x_position: 0,
            end_time: None,
            hyperdash: false,
        }
    }

    #[wasm_bindgen]
    pub fn end_time(&self) -> Option<i32> {
        self.end_time
    }

    #[wasm_bindgen]
    pub fn is_hyperdash(&self) -> bool {
        self.hyperdash
    }
}


// This is only a dummy for osu standard
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OsuHitobjectType {
    Empty,
    HitCircle,
    Slider,
    Spinner,
    Unknown,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OsuHitobject {
    #[wasm_bindgen(getter_with_clone)]
    pub object_type: OsuHitobjectType,
    #[wasm_bindgen(getter_with_clone)]
    pub x: i32,
    #[wasm_bindgen(getter_with_clone)]
    pub y: i32,
     #[wasm_bindgen(getter_with_clone)]
    pub end_time: Option<i32>,
    #[wasm_bindgen(getter_with_clone)]
    pub new_combo: bool,
    #[wasm_bindgen(getter_with_clone)]
    pub combo_skip: u8,
}

impl HitObject for OsuHitobject {
    fn game_mode(&self) -> GameMode {
        GameMode::OsuStandard
    }
}

#[wasm_bindgen]
impl OsuHitobject {
    #[wasm_bindgen]
    pub fn empty() -> Self {
        Self {
            object_type: OsuHitobjectType::Empty,
            x: 0,
            y: 0,
            end_time: None,
            new_combo: false,
            combo_skip: 0,
        }
    }

    #[wasm_bindgen]
    pub fn hit_circle(x: i32, y: i32) -> Self {
        Self {
            object_type: OsuHitobjectType::HitCircle,
            x,
            y,
            end_time: None,
            new_combo: false,
            combo_skip: 0,
        }
    }

    #[wasm_bindgen]
    pub fn slider(x: i32, y: i32) -> Self {
        Self {
            object_type: OsuHitobjectType::Slider,
            x,
            y,
            end_time: None,
            new_combo: false,
            combo_skip: 0,
        }
    }

    #[wasm_bindgen]
    pub fn spinner(end_time: i32) -> Self {
        Self {
            object_type: OsuHitobjectType::Spinner,
            x: 256,
            y: 192,
            end_time: Some(end_time),
            new_combo: false,
            combo_skip: 0,
        }
    }

    #[wasm_bindgen]
    pub fn unknown() -> Self {
        Self {
            object_type: OsuHitobjectType::Unknown,
            x: 0,
            y: 0,
            end_time: None,
            new_combo: false,
            combo_skip: 0,
        }
    }

    #[wasm_bindgen]
    pub fn with_new_combo(mut self) -> Self {
        self.new_combo = true;
        self
    }

    #[wasm_bindgen]
    pub fn with_combo_skip(mut self, skip: u8) -> Self {
        self.combo_skip = skip;
        self
    }

    #[wasm_bindgen]
    pub fn end_time(&self) -> Option<i32> {
        self.end_time
    }

    #[wasm_bindgen]
    pub fn is_new_combo(&self) -> bool {
        self.new_combo
    }

    #[wasm_bindgen]
    pub fn combo_skip(&self) -> u8 {
        self.combo_skip
    }
}