use std::{
    sync::{Arc, Mutex},
    time::SystemTime,
};

use crate::{
    layers::LayerLibrary,
    trigger::{Trigger, TriggerKeyFragment},
};

#[derive(Debug)]
pub struct RawInputFragment {
    pub device_alias: String,
    pub code: u16,
    pub value: i32,
    pub time: SystemTime,
}

#[derive(Debug)]
pub struct RawInputStack {
    fragments: Vec<RawInputFragment>,
}

impl RawInputStack {
    pub fn new() -> Self {
        Self { fragments: vec![] }
    }

    pub fn handle_incoming_input(&mut self, fragment: RawInputFragment, layers_lib: &LayerLibrary) {
        self.parse_input(&fragment, layers_lib);

        // handle self.fragments?
        match fragment.value {
            0 => {
                let i = self.fragments.iter().position(|f| {
                    f.device_alias == fragment.device_alias && f.code == fragment.code
                });
                if i.is_some() {
                    self.fragments.remove(i.unwrap());
                }
            }
            1 => self.fragments.push(fragment),
            _ => (),
        }

        // self.detect_union();
    }

    fn parse_input(&self, fragment: &RawInputFragment, layers_lib: &LayerLibrary) {
        // check for single remaps
        let alias = &fragment.device_alias;
        let code = &fragment.code;

        let rules = layers_lib.get_current_layer().get_rules();
        let trigger = TriggerKeyFragment::new(alias.to_owned(), code.to_owned());
        let trigger = Trigger::KeyPress(trigger);

        match rules.get(&trigger) {
            Some(action) => {
                dbg!(action);
                // TODO: implement action.invoke()
            }
            _ => (),
        }
    }

    #[allow(dead_code)]
    fn detect_union(&self) {
        let total: u128 = self
            .fragments
            .iter()
            .map(|f| f.time.elapsed().unwrap().as_millis())
            .sum();

        // TODO: Union detector based on this
        if total < 30 && self.fragments.len() > 1 {
            println!("----------------------");
            println!("total time = {}", total);
            dbg!(&self.fragments);
        }
    }
}

impl RawInputStack {
    #[allow(dead_code)]
    pub fn print(&self) {
        dbg!(&self.fragments);
    }
}
