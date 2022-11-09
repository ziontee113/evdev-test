use crate::{action::Action, trigger::Trigger};
use std::collections::BTreeMap;

pub struct LayerLibrary {
    layers: BTreeMap<String, Layer>,
    active_layer: String,
}

impl LayerLibrary {
    pub fn new(layers: Vec<Layer>) -> Self {
        let mut map = BTreeMap::new();
        for l in layers {
            map.insert(l.name.to_string(), l.clone());
        }
        Self {
            active_layer: map.keys().next().unwrap().to_string(),
            layers: map,
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Layer {
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
