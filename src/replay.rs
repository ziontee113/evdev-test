use super::physical_device;
use crate::{
    physical_device::InputEventKindCheck,
    raw_input_stack::{RawInputFragment, RawInputStack},
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
    time::SystemTime,
};

pub fn replay() {
    let devices_dictionary = HashMap::from([
        ("L1", "usb-0000:00:1d.0-1.5.1.4/input0"),
        ("R1", "usb-0000:00:1d.0-1.5.2/input0"),
    ]);

    let raw_stack = Arc::new(Mutex::new(RawInputStack::new()));

    let mut handles = Vec::new();
    for (device_alias, device_path) in devices_dictionary {
        let raw_stack = Arc::clone(&raw_stack);

        let handle = thread::spawn(move || -> ! {
            let mut d = physical_device::from_path(device_path);
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
                        raw_stack.receive(fragment);
                        // raw_stack.print();

                        if ev.value() == 1 {
                            raw_stack.print_combined_time()
                        }
                    }
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
