use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
};

use evdev::{
    uinput::{VirtualDevice, VirtualDeviceBuilder},
    AttributeSet, Device, EventType, InputEvent, InputEventKind, Key,
};

struct InputSequenceElement {
    device: String,
    key: Key,
    value: i32,
}

pub fn something() {
    // let device_paths = vec![
    //     "usb-0000:00:1d.0-1.5.1.4/input0", // A
    //     "usb-0000:00:1d.0-1.5.2/input0",   // B
    //     "usb-0000:00:1d.0-1.5.1.2/input0", // C
    //     "usb-0000:00:1d.0-1.5.1.1/input0",
    //     "usb-0000:00:1d.0-1.5.3/input0",
    // ];

    let device_hash_map = HashMap::from([
        ("L1", "usb-0000:00:1d.0-1.5.1.4/input0"),
        ("R1", "usb-0000:00:1d.0-1.5.2/input0"),
        ("L2", "usb-0000:00:1d.0-1.5.1.2/input0"),
    ]);

    let capslock_value = Arc::new(Mutex::new(0));
    let virtual_device = Arc::new(Mutex::new(new_virtual_keyboard()));

    let handle_1 = grab_device(
        device_hash_map.get("L1").unwrap().to_string(),
        Arc::clone(&capslock_value),
        Arc::clone(&virtual_device),
    );
    let handle_2 = grab_device(
        device_hash_map.get("R1").unwrap().to_string(),
        Arc::clone(&capslock_value),
        Arc::clone(&virtual_device),
    );

    handle_1.join().unwrap();
    handle_2.join().unwrap();
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

        // TODO: make some rules
        // if a key doesn't have logic
        // it behaves normally
        // --> with a virtual device, you can easily "cut the ties"

        loop {
            for ev in device.fetch_events().unwrap() {
                if path == first_path && ev.kind() == InputEventKind::Key(Key::KEY_CAPSLOCK) {
                    let capslock_value = Arc::clone(&capslock_value);
                    let mut capslock_value = capslock_value.lock().unwrap();
                    *capslock_value = ev.value()
                }
                if path == second_path && ev.kind() == InputEventKind::Key(Key::KEY_J) {
                    if *capslock_value.lock().unwrap() > 0 {
                        let emmit_event = emit_event_constructor(ev.value());
                        virtual_device.lock().unwrap().emit(&[emmit_event]).unwrap();
                    }
                }

                if path == first_path && ev.kind() == InputEventKind::Key(Key::KEY_A) {
                    let emmit_event =
                        InputEvent::new(EventType::KEY, Key::KEY_LEFTSHIFT.code(), ev.value());
                    virtual_device.lock().unwrap().emit(&[emmit_event]).unwrap();
                }
            }
        }
    });

    return handle;
}

fn emit_event_constructor(value: i32) -> InputEvent {
    let type_ = EventType::KEY;
    let code = Key::KEY_Z.code();
    InputEvent::new(type_, code, value)
}

fn new_virtual_keyboard() -> VirtualDevice {
    let mut keys = AttributeSet::<Key>::new();
    keys.insert(Key::KEY_LEFTSHIFT);
    keys.insert(Key::KEY_LEFTCTRL);
    keys.insert(Key::KEY_LEFTALT);
    keys.insert(Key::KEY_LEFTMETA);
    keys.insert(Key::KEY_Q);
    keys.insert(Key::KEY_W);
    keys.insert(Key::KEY_E);
    keys.insert(Key::KEY_R);
    keys.insert(Key::KEY_T);
    keys.insert(Key::KEY_Y);
    keys.insert(Key::KEY_I);
    keys.insert(Key::KEY_O);
    keys.insert(Key::KEY_P);
    keys.insert(Key::KEY_A);
    keys.insert(Key::KEY_S);
    keys.insert(Key::KEY_D);
    keys.insert(Key::KEY_F);
    keys.insert(Key::KEY_G);
    keys.insert(Key::KEY_H);
    keys.insert(Key::KEY_J);
    keys.insert(Key::KEY_K);
    keys.insert(Key::KEY_L);
    keys.insert(Key::KEY_Z);
    keys.insert(Key::KEY_X);
    keys.insert(Key::KEY_C);
    keys.insert(Key::KEY_V);
    keys.insert(Key::KEY_B);
    keys.insert(Key::KEY_V);
    keys.insert(Key::KEY_N);
    keys.insert(Key::KEY_M);
    keys.insert(Key::KEY_COMMA);

    let virtual_device = VirtualDeviceBuilder::new()
        .unwrap()
        .name("Fake Keyboard")
        .with_keys(&keys)
        .unwrap()
        .build()
        .unwrap();

    return virtual_device;
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
