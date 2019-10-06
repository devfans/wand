use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::cell::RefCell;

pub type Input = Rc<RefCell<InputProto>>;

pub struct InputProto {
    keydown: HashSet<String>,
    keydown_cache: HashSet<String>,
    keyup: HashSet<String>,
    // axis: HashMap<String, f32>
}

impl InputProto {
    pub fn new() -> Input {
        Rc::new(RefCell::new(Self {
            keydown: HashSet::new(),
            keydown_cache: HashSet::new(),
            keyup: HashSet::new(),
            // axis: HashMap::new(),
        }))
    }

    /*
    pub fn axis(&self, key: &str) -> f32 {
        match self.axis.get(key) {
            Some(v) => v.clone(),
            None => 0.,
        }
    }
    */

    pub fn axis(&mut self, pos: &str, neg: &str) -> f32 {
        if self.keydown(pos) {
            if self.keydown(neg) {
                0.
            } else {
                1.
            }
        } else if self.keydown(neg) {
            -1.
        } else {
            0.
        }
    }

    pub fn keydown(&mut self, key: &str) -> bool {
        self.keydown.contains(key) || self.keydown_cache.take(key).is_some()
    }

    pub fn keyup(&self, key: &str) -> bool {
        self.keyup.contains(key)
    }

    pub fn on_keydown(&mut self, key: &str) {
        // log!("Keydown {}", key);
        self.keydown.insert(key.to_string());
    }

    pub fn on_keyup(&mut self, key: &str) {
        // log!("Keyup {}", key);
        match self.keydown.take(key) {
            Some(v) => { self.keydown_cache.insert(v.clone()); },
            None => {},
        }
    }

}

