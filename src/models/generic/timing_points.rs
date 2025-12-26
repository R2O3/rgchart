use crate::wasm_bindgen;
use crate::models::common::TimingChangeType;

#[derive(Debug, Clone, Copy)]
pub struct TimingChange {
    pub change_type: TimingChangeType,
    pub value: f32,
}

#[derive(Debug, Clone)]
pub struct TimingPoint {
    pub time: i32,
    pub beat: f32,
    pub change: TimingChange,
}

// TODO: add wasm bindings for Timings
#[wasm_bindgen]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TimingPoints {
    #[wasm_bindgen(skip)]
    pub points: Vec<TimingPoint>,
}

impl From<Vec<TimingPoint>> for TimingPoints {
    fn from(points: Vec<TimingPoint>) -> Self {
        Self { points }
    }
}

impl TimingPoints {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            points: Vec::with_capacity(capacity),
        }
    }

    pub fn new(points: Vec<TimingPoint>) -> Self {
        Self { points }
    }

    pub fn add(&mut self, time: i32, beat: f32, change: TimingChange) {
        self.points.push(TimingPoint {
            time,
            beat,
            change,
        });
    }

    pub fn iter(&self) -> impl Iterator<Item = &TimingPoint> {
        self.points.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut TimingPoint> {
        self.points.iter_mut()
    }

    pub fn bpm_changes(&self) -> impl Iterator<Item = &TimingPoint> + '_ {
        self.points
            .iter()
            .filter(|p| matches!(p.change.change_type, TimingChangeType::Bpm))
    }

    pub fn sv_changes(&self) -> impl Iterator<Item = &TimingPoint> + '_ {
        self.points
            .iter()
            .filter(|p| matches!(p.change.change_type, TimingChangeType::Sv))
    }

    pub fn is_bpms_empty(&self) -> bool {
        !self.points
            .iter()
            .any(|p| matches!(p.change.change_type, TimingChangeType::Bpm))
    }

    pub fn is_sv_empty(&self) -> bool {
        !self.points
            .iter()
            .any(|p| matches!(p.change.change_type, TimingChangeType::Sv))
    }

    pub fn bpms(&self) -> Vec<f32> {
        self.points
            .iter()
            .filter(|p| matches!(p.change.change_type, TimingChangeType::Bpm))
            .map(|p| p.change.value)
            .collect()
    }

    pub fn bpms_times(&self) -> Vec<i32> {
        self.points
            .iter()
            .filter(|p| matches!(p.change.change_type, TimingChangeType::Bpm))
            .map(|p| p.time)
            .collect()
    }

    pub fn sv(&self) -> Vec<f32> {
        self.points
            .iter()
            .filter(|p| matches!(p.change.change_type, TimingChangeType::Sv))
            .map(|p| p.change.value)
            .collect()
    }
}
