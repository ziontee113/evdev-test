#![allow(dead_code)]

use std::{thread::sleep, time::Duration};

use evdev::{
    uinput::VirtualDeviceBuilder, AttributeSet, Device, EventType, InputEvent, InputEventKind, Key,
    MiscType, Synchronization,
};

mod _pick_device;

fn main() {
    // evtest()
    let device_paths = vec![
        "usb-0000:00:1d.0-1.5.1.4/input0", // A
        "usb-0000:00:1d.0-1.5.2/input0",   // B
        "usb-0000:00:1d.0-1.5.1.2/input0", // C
        "usb-0000:00:1d.0-1.5.1.1/input0",
        "usb-0000:00:1d.0-1.5.3/input0",
    ];
    test_hard_coded_device(device_paths.get(1).unwrap())
}

fn virtual_input_test() -> std::io::Result<()> {
    let mut keys = AttributeSet::<Key>::new();
    keys.insert(Key::KEY_A);

    let mut device = VirtualDeviceBuilder::new()?
        .name("Fake Keyboard")
        .with_keys(&keys)?
        .build()
        .unwrap();

    for path in device.enumerate_dev_nodes_blocking()? {
        let path = path?;
        println!("Available as {}", path.display());
    }

    let type_ = EventType::KEY;
    let code = Key::KEY_A.code();

    println!("Waiting for Ctrl-C...");
    loop {
        let down_event = InputEvent::new(type_, code, 1);
        device.emit(&[down_event]).unwrap();
        println!("Pressed.");
        sleep(Duration::from_secs(2));

        let up_event = InputEvent::new(type_, code, 0);
        device.emit(&[up_event]).unwrap();
        println!("Released.");
        sleep(Duration::from_secs(2));
    }
}

fn test_hard_coded_device(name: &str) {
    let mut device = pick_device_from_path(name);
    device.grab().unwrap();

    //
    let mut keys = AttributeSet::<Key>::new();
    keys.insert(Key::KEY_A);

    let mut virtual_device = VirtualDeviceBuilder::new()
        .unwrap()
        .name("Fake Keyboard")
        .with_keys(&keys)
        .unwrap()
        .build()
        .unwrap();

    let type_ = EventType::KEY;
    let code = Key::KEY_A.code();

    loop {
        for ev in device.fetch_events().unwrap() {
            let kind = ev.kind();
            if kind == InputEventKind::Key(Key::KEY_J) {
                let le_event = InputEvent::new(type_, code, ev.value());
                virtual_device.emit(&[le_event]).unwrap();
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
