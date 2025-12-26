use crate::wasm_bindgen;
use crate::models::common::{Key, KeyType};
use crate::models::generic::sound::{KeySound};

#[derive(Debug, Clone)]
pub struct HitObject {
    pub time: i32,
    pub beat: f32,
    pub keysound: KeySound,
    pub key: Key,
    pub lane: u8,
}

// TODO: add wasm bindings for HitObject
// TODO: add row and object count
#[wasm_bindgen]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct HitObjects {
    #[wasm_bindgen(skip)]
    pub objects: Vec<HitObject>,
}

impl From<Vec<HitObject>> for HitObjects {
    fn from(objects: Vec<HitObject>) -> Self {
        Self { objects }
    }
}

impl HitObjects {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            objects: Vec::with_capacity(capacity),
        }
    }

    pub fn new(objects: Vec<HitObject>) -> Self {
        Self { objects }
    }

    #[inline]
    pub fn add_hitobject(
        &mut self,
        time: i32,
        beat: f32,
        keysound: KeySound,
        key: Key,
        lane: u8
    ) {
        if key.key_type == KeyType::Empty {
            return;
        }
        self.objects.push(HitObject {
            time,
            beat,
            keysound,
            key,
            lane,
        });
    }

    #[inline]
    pub fn add_hitobject_sorted(&mut self, object: HitObject) {
        let len = self.objects.len();
        
        if len == 0 || object.time >= self.objects[len - 1].time {
            self.objects.push(object);
            return;
        }

        let pos = self.objects.binary_search_by(|obj| 
            obj.time.cmp(&object.time)
        ).unwrap_or_else(|pos| pos);
        
        self.objects.insert(pos, object);
    }

    pub fn iter(&self) -> impl Iterator<Item = &HitObject> {
        self.objects.iter()
    }
}