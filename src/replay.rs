use super::physical_device;
use crate::{
    physical_device::InputEventKindCheck,
    raw_input_stack::{RawInputFragment, RawInputStack},
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

    let raw_stack = Arc::new(Mutex::new(RawInputStack::new()));

    let mut handles = vec![];
    for (alias, path) in dictionary {
        let h = handle_input_device(alias, path, Arc::clone(&raw_stack));
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
) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut d = physical_device::from_path(&device_path);
        match d.grab() {
            Ok(_) => println!("Grabbed {} {} SUCCESSFULLY", device_alias, device_path),
            Err(_) => println!("FAILED TO GRAB {} {}", device_alias, device_path),
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
                    raw_stack.handle_incoming_input(fragment);
                    // raw_stack.print();
                }
            }
        }
    })
}
