#![allow(dead_code)]

use evdev::{Device, InputEventKind, Key, MiscType, Synchronization};

mod _pick_device;

fn main() {
    // println!("Hello, world!");
    // evtest()

    let name = "usb-0000:00:1d.0-1.5.2/input0";
    let mut device = pick_device_from_path(name);
    device.grab().unwrap();

    loop {
        for ev in device.fetch_events().unwrap() {
            let kind = ev.kind();
            if kind != InputEventKind::Synchronization(Synchronization(0))
                && kind != InputEventKind::Misc(MiscType::MSC_SCAN)
            {
                if kind == InputEventKind::Key(Key::KEY_J) && ev.value() > 0 {
                    println!("hello fucking venus")
                } else {
                    println!("ev {:#?}", ev)
                }
            }
        }
    }
}

fn pick_device_from_path(path: &str) -> Device {
    let devices = evdev::enumerate().map(|t| t.1).collect::<Vec<_>>();
    let mut x = 0;
    for (i, d) in devices.iter().enumerate() {
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
