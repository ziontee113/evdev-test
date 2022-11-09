use crate::{action::Action, trigger::Trigger};
use std::collections::BTreeMap;

struct LayersLibrary {
    layers: BTreeMap<String, Layer>,
}

impl LayersLibrary {
    pub fn new(layers: Vec<Layer>) -> Self {
        let mut map = BTreeMap::new();
        for l in layers {
            map.insert(l.name.to_string(), l.clone());
        }
        Self { layers: map }
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Layer {
    rules: BTreeMap<Trigger, Action>,
    name: String,
}

impl Layer {
    pub fn new(name: String) -> Self {
        Self {
            rules: BTreeMap::new(),
            name,
        }
    }

    pub fn add_rule(&mut self, trigger: Trigger, action: Action) {
        self.rules.insert(trigger, action);
    }

    pub fn remove_rule(&mut self, trigger: Trigger) {
        match self.rules.remove(&trigger) {
            None => println!("Cannot remove rule with invalid {:?} trigger", trigger),
            _ => (),
        }
    }
}
