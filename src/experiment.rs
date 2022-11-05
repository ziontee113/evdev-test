use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
};

use evdev::{
    uinput::{VirtualDevice, VirtualDeviceBuilder},
    AttributeSet, Device, EventType, InputEvent, InputEventKind, Key, MiscType, Synchronization,
};

pub fn something() {
    let device_hash_map = HashMap::from([
        ("L1", "usb-0000:00:1d.0-1.5.1.4/input0"),
        ("R1", "usb-0000:00:1d.0-1.5.2/input0"),
        ("L2", "usb-0000:00:1d.0-1.5.1.2/input0"),
    ]);

    let rules = vec![
        vec![Key::KEY_LEFTCTRL.code(), Key::KEY_J.code()],
        vec![Key::KEY_LEFTCTRL.code(), Key::KEY_K.code()],
    ];

    let keypress_vector: Arc<Mutex<Vec<(String, u16)>>> = Arc::new(Mutex::new([].to_vec()));

    let virtual_device = Arc::new(Mutex::new(new_virtual_keyboard()));

    // threads
    let aliases = vec!["L1", "R1"];
    let mut handles = Vec::new();

    for alias in aliases {
        println!(" {:#?}", alias);

        let handle = grab_device(
            device_hash_map.get(alias).unwrap().to_string(),
            Arc::clone(&virtual_device),
            alias.to_string(),
            rules.to_vec(),
            Arc::clone(&keypress_vector),
        );

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn grab_device(
    path: String,
    virtual_device: Arc<Mutex<VirtualDevice>>,
    device_alias: String,
    rules: Vec<Vec<u16>>,
    keypress_vector: Arc<Mutex<Vec<(String, u16)>>>,
) -> thread::JoinHandle<()> {
    let handle = thread::spawn(move || {
        let mut device = get_device_from_path(&path);
        device.grab().unwrap();

        // TODO: make some rules
        // if a key doesn't have logic
        // it behaves normally
        // --> with a virtual device, you can easily "cut the ties"

        loop {
            for ev in device.fetch_events().unwrap() {
                if ev.kind() != InputEventKind::Synchronization(Synchronization::SYN_REPORT)
                    && ev.kind() != InputEventKind::Misc(MiscType::MSC_SCAN)
                {
                    let mut keypress_vector = keypress_vector.lock().unwrap();
                    update_keypress_vector(ev, device_alias.to_string(), &mut keypress_vector);
                }
            }
        }
    });

    return handle;
}

fn update_keypress_vector(
    ev: InputEvent,
    device_alias: String,
    keypress_vector: &mut Vec<(String, u16)>,
) {
    let value = ev.value();
    let code = ev.code();
    let alias_and_code = (device_alias, ev.code());

    if value == 0 {
        let i = keypress_vector.iter().position(|x| x.1 == code).unwrap();
        keypress_vector.remove(i);
    } else {
        if !keypress_vector.contains(&alias_and_code) {
            keypress_vector.push(alias_and_code)
        }
    }

    println!("{:?}", keypress_vector)
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
