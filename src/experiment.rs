use super::{physical_device, virtual_device};
use evdev::{uinput::VirtualDevice, InputEvent, InputEventKind, Key, MiscType, Synchronization};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
    time::SystemTime,
};

pub fn something() {
    let device_hash_map = HashMap::from([
        ("L1", "usb-0000:00:1d.0-1.5.1.4/input0"),
        ("R1", "usb-0000:00:1d.0-1.5.2/input0"),
        ("L2", "usb-0000:00:1d.0-1.5.1.2/input0"),
    ]);

    let keypress_vector: Arc<Mutex<Vec<(String, u16)>>> = Arc::new(Mutex::new([].to_vec()));

    let virtual_device = Arc::new(Mutex::new(virtual_device::new()));
    let time_now = Arc::new(Mutex::new(SystemTime::now()));

    let aliases = vec!["L1", "R1"];
    let mut handles = Vec::new();

    let mut the_hash_map = HashMap::new();
    the_hash_map.insert(
        vec![
            ("L1".to_string(), Key::KEY_LEFTCTRL.code()),
            ("R1".to_string(), Key::KEY_J.code()),
        ],
        ("R1".to_string(), Key::KEY_A.code()),
    );
    the_hash_map.insert(
        vec![
            ("L1".to_string(), Key::KEY_LEFTCTRL.code()),
            ("R1".to_string(), Key::KEY_K.code()),
        ],
        ("R1".to_string(), Key::KEY_B.code()),
    );
    the_hash_map.insert(
        vec![
            ("L1".to_string(), Key::KEY_LEFTCTRL.code()),
            ("L1".to_string(), Key::KEY_LEFTSHIFT.code()),
            ("R1".to_string(), Key::KEY_P.code()),
        ],
        ("R1".to_string(), Key::KEY_Q.code()),
    );
    the_hash_map.insert(
        vec![
            ("L1".to_string(), Key::KEY_CAPSLOCK.code()),
            ("L1".to_string(), Key::KEY_A.code()),
            ("R1".to_string(), Key::KEY_J.code()),
        ],
        ("R1".to_string(), Key::KEY_W.code()),
    );
    let the_hash_map = Arc::new(Mutex::new(the_hash_map.clone()));

    // threads
    for alias in aliases {
        println!(" {:#?}", alias);

        let handle = grab_device(
            device_hash_map.get(alias).unwrap().to_string(),
            Arc::clone(&virtual_device),
            alias.to_string(),
            Arc::clone(&the_hash_map),
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
    rules: Arc<Mutex<HashMap<Vec<(String, u16)>, (String, u16)>>>,
    keypress_vector: Arc<Mutex<Vec<(String, u16)>>>,
    time_now: Arc<Mutex<SystemTime>>,
) -> thread::JoinHandle<()> {
    let handle = thread::spawn(move || {
        let mut device = physical_device::from_path(&path);
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
                        rules.lock().unwrap().clone(),
                        Arc::clone(&virtual_device),
                    );
                }
            }
        }
    });

    return handle;
}

fn check_hashmap_do_action(
    keypress_vector: &mut Vec<(String, u16)>,
    rules: HashMap<Vec<(String, u16)>, (String, u16)>,
    virtual_device: Arc<Mutex<VirtualDevice>>,
    ev: InputEvent,
) {
    match rules.get(keypress_vector) {
        Some((_, code)) => {
            for (_, code) in keypress_vector {
                virtual_device::emit_key(&virtual_device, *code, 0);
            }
            // TODO: restore whatever comes before this shit! (like ctrl, shift, etc...)

            virtual_device::emit_key(&virtual_device, *code, ev.value());
        }
        None => virtual_device::emit_key(&virtual_device, ev.code(), ev.value()),
    }
}

fn update_keypress_vector(
    ev: InputEvent,
    device_alias: String,
    keypress_vector: &mut Vec<(String, u16)>,
    time_now: &mut SystemTime,
    rules: HashMap<Vec<(String, u16)>, (String, u16)>,
    virtual_device: Arc<Mutex<VirtualDevice>>,
) {
    //
    let alias_and_code = (device_alias, ev.code());

    match ev.value() {
        0 => {
            let i = keypress_vector
                .iter()
                .position(|x| x == &alias_and_code)
                .unwrap();
            check_hashmap_do_action(keypress_vector, rules, virtual_device, ev);
            keypress_vector.remove(i);
        }
        1 => {
            if keypress_vector.len() == 0 {
                *time_now = SystemTime::now();
            }

            // if keypress_vector.len() > 0 {
            //     let time_diff = time_now.elapsed().unwrap().as_millis();
            //     println!(
            //         "keypress_vector len() = {}, time_diff = {:?}",
            //         keypress_vector.len(),
            //         time_diff
            //     )
            // }

            keypress_vector.push(alias_and_code);
            check_hashmap_do_action(keypress_vector, rules, virtual_device, ev);
        }
        _ => {
            check_hashmap_do_action(keypress_vector, rules, virtual_device, ev);
        }
    }

    // println!("{:?} {}", keypress_vector, ev.value())
}
