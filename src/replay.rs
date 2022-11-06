use crate::physical_device::InputEventKindCheck;

use super::physical_device;
use std::{collections::HashMap, thread};

pub fn replay() {
    let devices_dictionary = HashMap::from([
        ("L1", "usb-0000:00:1d.0-1.5.1.4/input0"),
        ("R1", "usb-0000:00:1d.0-1.5.2/input0"),
    ]);

    let mut handles = Vec::new();
    for (device_alias, device_path) in devices_dictionary {
        let handle = thread::spawn(move || -> ! {
            let mut d = physical_device::from_path(device_path);
            match d.grab() {
                Ok(_) => println!("Grabbed {} {} SUCCESSFULLY", device_alias, device_path),
                Err(_) => println!("FAILED TO GRAB {} {}", device_alias, device_path),
            }

            loop {
                for ev in d.fetch_events().unwrap() {
                    if ev.is_type_key() {
                        println!("event is: {:?}", ev)
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
