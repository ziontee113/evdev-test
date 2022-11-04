use std::thread;

use evdev::{Device, InputEventKind, Key};

pub fn something() {
    let device_paths = vec![
        "usb-0000:00:1d.0-1.5.1.4/input0", // A
        "usb-0000:00:1d.0-1.5.2/input0",   // B
        "usb-0000:00:1d.0-1.5.1.2/input0", // C
        "usb-0000:00:1d.0-1.5.1.1/input0",
        "usb-0000:00:1d.0-1.5.3/input0",
    ];

    let handle_1 = grab_device(device_paths.get(1).unwrap().to_string());
    let handle_2 = grab_device(device_paths.get(0).unwrap().to_string());

    handle_1.join().unwrap();
    handle_2.join().unwrap();
}

fn grab_device(path: String) -> thread::JoinHandle<()> {
    let handle = thread::spawn(move || {
        let mut device = get_device_from_path(&path);
        device.grab().unwrap();

        let device_path = device.physical_path().unwrap().to_string();

        loop {
            for ev in device.fetch_events().unwrap() {
                if ev.kind() == InputEventKind::Key(Key::KEY_J) {
                    println!("{}", device_path)
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
