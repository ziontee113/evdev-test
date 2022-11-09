use evdev::Key;

use super::physical_device;
use crate::{
    action::{Action, SingleKeyAction},
    layers::{Layer, LayerLibrary},
    physical_device::InputEventKindCheck,
    raw_input_stack::{RawInputFragment, RawInputStack},
    trigfrag,
    trigger::{Trigger, TriggerKeyFragment},
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    time::SystemTime,
};

pub fn replay() {
    let dictionary = HashMap::from([
        (
            String::from("L1"),
            String::from("usb-0000:00:1d.0-1.5.1.4/input0"),
        ),
        (
            String::from("R1"),
            String::from("usb-0000:00:1d.0-1.5.2/input0"),
        ),
    ]);

    // fun stuff
    let fragment_1 = trigfrag!("L1", Key::KEY_A.code());
    let fragment_2a = trigfrag!("L1", Key::KEY_LEFTCTRL.code());
    let fragment_2b = trigfrag!("R1", Key::KEY_J.code());

    let trigger_1 = Trigger::KeyPress(fragment_1);
    let trigger_2 = Trigger::KeyChain(vec![fragment_2a, fragment_2b]);

    let action_1 = SingleKeyAction::new(String::from("R1"), Key::KEY_O.code());
    let action_1 = Action::SingleKeyAction(action_1);
    let action_2 = SingleKeyAction::new(String::from("R1"), Key::KEY_P.code());
    let action_2 = Action::SingleKeyAction(action_2);

    let mut base_layer = Layer::new(String::from("Base Layer"));
    base_layer.add_rule(trigger_1, action_1);
    base_layer.add_rule(trigger_2, action_2);

    let layers_lib = LayerLibrary::new(vec![base_layer]);

    // normal stuff
    let raw_stack = Arc::new(Mutex::new(RawInputStack::new()));

    let mut handles = vec![];
    for (alias, path) in dictionary {
        let h = handle_input_device(alias, path, Arc::clone(&raw_stack), layers_lib.clone());
        handles.push(h);
    }
    for h in handles {
        h.join().unwrap();
    }
}

fn handle_input_device(
    device_alias: String,
    device_path: String,
    raw_stack: Arc<Mutex<RawInputStack>>,
    layers_lib: LayerLibrary,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut d = physical_device::from_path(&device_path);
        match d.grab() {
            Ok(_) => println!("Grabbed {} {} SUCCESSFULLY", device_alias, device_path),
            Err(err) => {
                println!(
                    "FAILED TO GRAB {} {},\n{},\n------------------",
                    device_alias, device_path, err
                );
            }
        }

        loop {
            for ev in d.fetch_events().unwrap() {
                if ev.is_type_key() {
                    let fragment = RawInputFragment {
                        device_alias: device_alias.to_string(),
                        code: ev.code(),
                        value: ev.value(),
                        time: SystemTime::now(),
                    };
                    let mut raw_stack = raw_stack.lock().unwrap();
                    raw_stack.handle_incoming_input(fragment, &layers_lib);
                }
            }
        }
    })
}
