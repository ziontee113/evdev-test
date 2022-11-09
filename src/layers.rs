use crate::{action::Action, trigger::Trigger};
use std::collections::HashMap;

struct LayersLibrary {
    layers: HashMap<String, Layer>,
}

impl LayersLibrary {
    pub fn new(layers: Vec<Layer>) -> Self {
        let mut map = HashMap::new();
        for l in layers {
            map.insert(l.name.to_string(), l.clone());
        }
        Self { layers: map }
    }
}

#[derive(Clone)]
struct Layer {
    rules: HashMap<Trigger, Action>,
    name: String,
}

impl Layer {
    pub fn new(name: String) -> Self {
        Self {
            rules: HashMap::new(),
            name,
        }
    }
}
