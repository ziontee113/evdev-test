use std::{
    sync::{Arc, Mutex},
    thread,
};

use evdev::{
    uinput::{VirtualDevice, VirtualDeviceBuilder},
    AttributeSet, Device, EventType, InputEvent, InputEventKind, Key,
};

pub fn something() {
    let device_paths = vec![
        "usb-0000:00:1d.0-1.5.1.4/input0", // A
        "usb-0000:00:1d.0-1.5.2/input0",   // B
        "usb-0000:00:1d.0-1.5.1.2/input0", // C
        "usb-0000:00:1d.0-1.5.1.1/input0",
        "usb-0000:00:1d.0-1.5.3/input0",
    ];

    let capslock_value = Arc::new(Mutex::new(0));
    let vvip = Arc::new(Mutex::new(new_virtual_keyboard()));

    let handle_1 = grab_device(
        device_paths.get(1).unwrap().to_string(),
        Arc::clone(&capslock_value),
        Arc::clone(&vvip),
    );
    let handle_2 = grab_device(
        device_paths.get(0).unwrap().to_string(),
        Arc::clone(&capslock_value),
        Arc::clone(&vvip),
    );

    handle_1.join().unwrap();
    handle_2.join().unwrap();
}

fn press_z(value: i32) -> InputEvent {
    let type_ = EventType::KEY;
    let code = Key::KEY_Z.code();
    let event = InputEvent::new(type_, code, value);

    return event;
}

fn new_virtual_keyboard() -> VirtualDevice {
    let mut keys = AttributeSet::<Key>::new();
    keys.insert(Key::KEY_Z);

    let virtual_device = VirtualDeviceBuilder::new()
        .unwrap()
        .name("Fake Keyboard")
        .with_keys(&keys)
        .unwrap()
        .build()
        .unwrap();

    return virtual_device;
}

fn grab_device(
    path: String,
    capslock_value: Arc<Mutex<i32>>,
    virtual_device: Arc<Mutex<VirtualDevice>>,
) -> thread::JoinHandle<()> {
    let handle = thread::spawn(move || {
        let mut device = get_device_from_path(&path);
        device.grab().unwrap();

        // HACK:
        let first_path = "usb-0000:00:1d.0-1.5.1.4/input0";
        let second_path = "usb-0000:00:1d.0-1.5.2/input0";

        loop {
            for ev in device.fetch_events().unwrap() {
                if path == first_path && ev.kind() == InputEventKind::Key(Key::KEY_CAPSLOCK) {
                    let capslock_value = Arc::clone(&capslock_value);
                    let mut capslock_value = capslock_value.lock().unwrap();
                    *capslock_value = ev.value()
                }
                if path == second_path && ev.kind() == InputEventKind::Key(Key::KEY_J) {
                    if *capslock_value.lock().unwrap() > 0 {
                        let emmit_event = press_z(ev.value());
                        virtual_device.lock().unwrap().emit(&[emmit_event]).unwrap();
                    }
                }
            }
        }
    });

    return handle;
}

fn get_device_from_path(path: &str) -> Device {
    let devices = evdev::enumerate().map(|t| t.1).collect::<Vec<_>>();
    let mut device_index = 0;
    for (i, d) in devices.iter().enumerate() {
        // println!("{}", d.physical_path().unwrap());
        if d.physical_path().unwrap() == path {
            device_index = i;
            break;
        }
    }
    devices.into_iter().nth(device_index).unwrap()
}
