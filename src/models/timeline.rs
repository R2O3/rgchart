use std::collections::VecDeque;
use std::ops::{Index, IndexMut};
use crate::models::common::*;
use crate::models::generic::HitObject;
use crate::models::generic::KeySound;
use crate::models::generic::{TimingPoints, TimingChange};
use crate::utils::rhythm::calculate_beat_from_time;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TimelineTimingPoint {
    pub time: i32,
    pub value: f32,
    pub change_type: TimingChangeType,
}

pub trait TimelineOps<Item> {
    fn timeline(&self) -> &Vec<Item>;
    fn timeline_mut(&mut self) -> &mut Vec<Item>;
    fn is_sorted(&self) -> bool;
    fn set_sorted(&mut self, sorted: bool);
    fn item_time(item: &Item) -> i32;

    fn with_capacity(capacity: usize) -> Self where Self: Sized;

    fn new() -> Self where Self: Sized;

    #[inline]
    fn add(&mut self, timeline_object: Item) {
        if self.is_sorted() && !self.timeline().is_empty() {
            let is_sorted = Self::item_time(&timeline_object) >= Self::item_time(self.timeline().last().unwrap());
            self.set_sorted(is_sorted);
        }
        self.timeline_mut().push(timeline_object);
    }

    #[inline]
    fn len(&self) -> usize {
        self.timeline().len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.timeline().is_empty()
    }

    #[allow(unused)]
    #[inline]
    fn reserve(&mut self, additional: usize) {
        self.timeline_mut().reserve(additional);
    }

    #[allow(unused)]
    #[inline]
    fn shrink_to_fit(&mut self) {
        self.timeline_mut().shrink_to_fit();
    }

    #[inline]
    fn add_sorted(&mut self, timeline_object: Item) {
        let len = self.timeline().len();
        
        if len == 0 || Self::item_time(&timeline_object) >= Self::item_time(&self.timeline()[len - 1]) {
            self.timeline_mut().push(timeline_object);
            return;
        }

        let pos = self.timeline().binary_search_by(|obj| 
            Self::item_time(obj).cmp(&Self::item_time(&timeline_object))
        ).unwrap_or_else(|pos| pos);
        
        self.timeline_mut().insert(pos, timeline_object);
    }

    #[inline]
    fn sort(&mut self) {
        if !self.is_sorted() {
            self.timeline_mut().sort_unstable_by(|a, b| Self::item_time(a).cmp(&Self::item_time(b)));
            self.set_sorted(true);
        }
    }
}

pub struct HitObjectTimeline;

impl HitObjectTimeline {
    pub fn to_rows(hitobjects: &[HitObject], key_count: usize) -> Vec<HitObjectRow> {
        if hitobjects.is_empty() {
            return Vec::new();
        }

        let mut rows = Vec::new();
        let mut temp_row = vec![Key::empty(); key_count];
        
        let mut current_time = hitobjects[0].time;
        let mut i = 0;
        
        while i < hitobjects.len() {            
            while i < hitobjects.len() && hitobjects[i].time == current_time {
                let obj = &hitobjects[i];
                let lane = obj.lane as usize;
                
                if lane < key_count {
                    match obj.key.key_type {
                        KeyType::Normal => {
                            if temp_row[lane].key_type != KeyType::SliderStart {
                                temp_row[lane] = obj.key;
                            }
                        },
                        KeyType::SliderStart => {
                            temp_row[lane] = obj.key;
                        },
                        KeyType::SliderEnd => {
                            if temp_row[lane].key_type != KeyType::SliderStart {
                                temp_row[lane] = obj.key;
                            }
                        },
                        _ => {}
                    }
                }
                i += 1;
            }
            
            rows.push(HitObjectRow {
                time: current_time,
                beat: hitobjects[i - 1].beat,
                keys: temp_row.clone(),
            });
            
            if i < hitobjects.len() {
                current_time = hitobjects[i].time;
                temp_row.fill(Key::empty());
            }
        }
        
        rows
    }

    pub fn flatten_rows(rows: &[HitObjectRow], key_count: usize) -> Vec<HitObject> {
        let mut result = Vec::new();
        
        let mut slider_start_queues: Vec<VecDeque<usize>> = vec![VecDeque::new(); key_count];
        let mut slider_end_indices = Vec::new();
        
        for row in rows {
            for (lane_idx, key) in row.keys.iter().enumerate() {
                if lane_idx >= key_count {
                    continue;
                }
                
                if key.key_type == KeyType::Empty {
                    continue;
                }
                
                let hit_object = HitObject {
                    time: row.time,
                    beat: row.beat,
                    keysound: KeySound::default(),
                    key: *key,
                    lane: (lane_idx + 1) as u8,
                };
                
                let index = result.len();
                result.push(hit_object);
                
                match key.key_type {
                    KeyType::SliderStart => {
                        slider_start_queues[lane_idx].push_back(index);
                    }
                    KeyType::SliderEnd => {
                        slider_end_indices.push(index);
                    }
                    _ => {}
                }
            }
        }
        
        for &end_idx in &slider_end_indices {
            let end_time = result[end_idx].time;
            let lane = result[end_idx].lane as usize;
            
            if lane == 0 || lane > key_count {
                continue;
            }
            
            let queue_idx = lane - 1;
            let queue = &mut slider_start_queues[queue_idx];
            
            if queue.is_empty() {
                eprintln!("Warning: SliderEnd at time {} lane {} has no matching SliderStart", end_time, lane);
                continue;
            }
            
            let mut matched_idx = None;
            for (i, &start_idx) in queue.iter().enumerate() {
                if result[start_idx].time <= end_time {
                    matched_idx = Some(i);
                } else {
                    break;
                }
            }
            
            if let Some(idx) = matched_idx {
                let start_idx = queue.remove(idx).unwrap();
                result[start_idx].key.slider_end_time = Some(end_time);
            } else {
                eprintln!("Warning: SliderEnd at time {} lane {} has no matching SliderStart before it", end_time, lane);
            }
        }
        
        for (lane_idx, queue) in slider_start_queues.iter().enumerate() {
            if !queue.is_empty() {
                eprintln!("Warning: {} unmatched SliderStart(s) in lane {}", queue.len(), lane_idx + 1);
            }
        }
        
        result
    }
}

pub struct TimingPointTimeline {
    timeline: Vec<TimelineTimingPoint>,
    is_sorted: bool,
}

impl TimelineOps<TimelineTimingPoint> for TimingPointTimeline {
    #[inline]
    fn timeline(&self) -> &Vec<TimelineTimingPoint> {
        &self.timeline
    }

    #[inline]
    fn timeline_mut(&mut self) -> &mut Vec<TimelineTimingPoint> {
        &mut self.timeline
    }

    #[inline]
    fn is_sorted(&self) -> bool {
        self.is_sorted
    }

    #[inline]
    fn set_sorted(&mut self, sorted: bool) {
        self.is_sorted = sorted;
    }

    #[inline]
    fn item_time(item: &TimelineTimingPoint) -> i32 {
        item.time
    }

    #[inline]
    fn with_capacity(capacity: usize) -> Self {
        Self {
            timeline: Vec::with_capacity(capacity),
            is_sorted: true,
        }
    }

    #[inline]
    fn new() -> Self {
        Self {
            timeline: Vec::new(),
            is_sorted: true,
        }
    }
}

impl TimingPointTimeline {
    pub fn to_timing_points(&mut self, timing_points: &mut TimingPoints, offset: i32) {
        if self.timeline.is_empty() {
            return;
        }

        self.sort();
        
        let mut bpm_times = Vec::new();
        let mut bpms = Vec::new();
        
        for timing_point in &self.timeline {
            match timing_point.change_type {
                TimingChangeType::Bpm => {
                    bpm_times.push(timing_point.time);
                    bpms.push(timing_point.value);
                }
                _ => {}
            }
        }
        
        for timing_point in &self.timeline {
            let time = timing_point.time;
            let beat = calculate_beat_from_time(time, offset, (&bpm_times, &bpms));
            
            timing_points.add(
                time,
                beat,
                TimingChange {
                    value: timing_point.value,
                    change_type: timing_point.change_type,
                }
            );
        }
    }
}

impl IntoIterator for TimingPointTimeline {
    type Item = TimelineTimingPoint;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.timeline.into_iter()
    }
}

impl<'a> IntoIterator for &'a TimingPointTimeline {
    type Item = &'a TimelineTimingPoint;
    type IntoIter = std::slice::Iter<'a, TimelineTimingPoint>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.timeline.iter()
    }
}

impl<'a> IntoIterator for &'a mut TimingPointTimeline {
    type Item = &'a mut TimelineTimingPoint;
    type IntoIter = std::slice::IterMut<'a, TimelineTimingPoint>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.timeline.iter_mut()
    }
}

impl Index<usize> for TimingPointTimeline {
    type Output = TimelineTimingPoint;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.timeline[index]
    }
}

impl IndexMut<usize> for TimingPointTimeline {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.timeline[index]
    }
}

impl std::ops::Deref for TimingPointTimeline {
    type Target = [TimelineTimingPoint];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.timeline
    }
}

impl std::ops::DerefMut for TimingPointTimeline {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.timeline
    }
}