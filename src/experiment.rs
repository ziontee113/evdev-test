use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
    time::SystemTime,
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

    let keypress_vector: Arc<Mutex<Vec<(String, u16)>>> = Arc::new(Mutex::new([].to_vec()));

    let virtual_device = Arc::new(Mutex::new(new_virtual_keyboard()));
    let time_now = Arc::new(Mutex::new(SystemTime::now()));

    let aliases = vec!["L1", "R1"];
    let mut handles = Vec::new();

    let rules = vec![
        vec![
            ("L1".to_string(), Key::KEY_LEFTCTRL.code()),
            ("R1".to_string(), Key::KEY_J.code()),
        ],
        vec![
            ("L1".to_string(), Key::KEY_LEFTCTRL.code()),
            ("R1".to_string(), Key::KEY_K.code()),
        ],
        vec![
            ("L1".to_string(), Key::KEY_LEFTCTRL.code()),
            ("L1".to_string(), Key::KEY_LEFTSHIFT.code()),
            ("R1".to_string(), Key::KEY_P.code()),
        ],
    ];

    // threads
    for alias in aliases {
        println!(" {:#?}", alias);

        let handle = grab_device(
            device_hash_map.get(alias).unwrap().to_string(),
            Arc::clone(&virtual_device),
            alias.to_string(),
            rules.to_vec(),
            Arc::clone(&keypress_vector),
            Arc::clone(&time_now),
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
    rules: Vec<Vec<(String, u16)>>,
    keypress_vector: Arc<Mutex<Vec<(String, u16)>>>,
    time_now: Arc<Mutex<SystemTime>>,
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
                    update_keypress_vector(
                        ev,
                        device_alias.to_string(),
                        &mut keypress_vector.lock().unwrap(),
                        &mut time_now.lock().unwrap(),
                        rules.to_vec(),
                    );
                }
            }
        }
    });

    return handle;
}

fn all_match(keypress_vector: &mut Vec<(String, u16)>, rules: Vec<Vec<(String, u16)>>) {
    if keypress_vector.len() == 0 {
        return;
    }

    let mut all_match = false;
    for rule in rules {
        if rule.len() == keypress_vector.len() {
            let mut matches = 0;
            let mut rule_fragment_index = 0;
            for rule_fragment in rule {
                if &rule_fragment == keypress_vector.get(rule_fragment_index).unwrap() {
                    matches += 1;
                    if matches == keypress_vector.len() {
                        all_match = true;
                        break;
                    }
                }
                rule_fragment_index += 1;
            }
        }
    }

    if all_match {
        println!("all match! {:?}", keypress_vector);
    }
}

fn update_keypress_vector(
    ev: InputEvent,
    device_alias: String,
    keypress_vector: &mut Vec<(String, u16)>,
    time_now: &mut SystemTime,
    rules: Vec<Vec<(String, u16)>>,
) {
    all_match(keypress_vector, rules);

    let alias_and_code = (device_alias, ev.code());

    match ev.value() {
        0 => {
            let i = keypress_vector
                .iter()
                .position(|x| x == &alias_and_code)
                .unwrap();
            keypress_vector.remove(i);
        }
        1 => {
            if keypress_vector.len() == 0 {
                *time_now = SystemTime::now();
            }

            if keypress_vector.len() > 0 {
                let time_diff = time_now.elapsed().unwrap().as_millis();
                println!(
                    "keypress_vector len() = {}, time_diff = {:?}",
                    keypress_vector.len(),
                    time_diff
                )
            }

            keypress_vector.push(alias_and_code);
        }
        _ => (),
    }

    // println!("{:?} {}", keypress_vector, ev.value())
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
