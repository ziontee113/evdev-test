#![allow(dead_code)]

use std::collections::HashMap;

use evdev::{
    uinput::{VirtualDevice, VirtualDeviceBuilder},
    AttributeSet, Device, EventType, InputEvent, InputEventKind, Key,
};

mod _pick_device;
mod experiment;
mod mamba;
mod physical_device;
mod virtual_device;

fn main() {
    // let device_paths = vec![
    //     "usb-0000:00:1d.0-1.5.1.4/input0", // A
    //     "usb-0000:00:1d.0-1.5.2/input0",   // B
    //     "usb-0000:00:1d.0-1.5.1.2/input0", // C
    //     "usb-0000:00:1d.0-1.5.1.1/input0",
    //     "usb-0000:00:1d.0-1.5.3/input0",
    // ];
    //
    // test_hard_coded_device(device_paths.get(1).unwrap())

    // experiment::something()

    mamba::black_mamba();

    // hash_map_experiment()
}

fn hash_map_experiment() {
    let mut map = HashMap::new();

    map.insert(Key::KEY_LEFTSHIFT, "left_shift");
    map.insert(Key::KEY_A, "a");

    let result = map.get(&Key::KEY_A).unwrap();
    println!(" {:#?}", result)
}

fn new_virtual_keyboard() -> VirtualDevice {
    let mut keys = AttributeSet::<Key>::new();
    keys.insert(Key::KEY_A);

    let virtual_device = VirtualDeviceBuilder::new()
        .unwrap()
        .name("Fake Keyboard")
        .with_keys(&keys)
        .unwrap()
        .build()
        .unwrap();

    return virtual_device;
}

fn emit_keyboard_event_constructor(value: i32) -> InputEvent {
    let type_ = EventType::KEY;
    let code = Key::KEY_A.code();
    InputEvent::new(type_, code, value)
}

fn test_hard_coded_device(name: &str) {
    let mut device = pick_device_from_path(name);
    device.grab().unwrap();

    let mut virtual_device = new_virtual_keyboard();

    loop {
        for ev in device.fetch_events().unwrap() {
            if ev.kind() == InputEventKind::Key(Key::KEY_J) {
                let emit_event = emit_keyboard_event_constructor(ev.value());
                virtual_device.emit(&[emit_event]).unwrap();
            }
        }
    }
}

fn pick_device_from_path(path: &str) -> Device {
    let devices = evdev::enumerate().map(|t| t.1).collect::<Vec<_>>();
    let mut x = 0;
    for (i, d) in devices.iter().enumerate() {
        // println!("{}", d.physical_path().unwrap());
        if d.physical_path().unwrap() == path {
            x = i;
            break;
        }
    }
    devices.into_iter().nth(x).unwrap()
}

fn evtest() {
    let mut d = _pick_device::pick_device();
    println!("{}", d);
    println!("Events:");
    loop {
        for ev in d.fetch_events().unwrap() {
            println!("{:?}", ev);
        }
    }
}
