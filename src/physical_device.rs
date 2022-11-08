use evdev::{Device, InputEvent, InputEventKind};

pub trait InputEventKindCheck {
    fn is_type_key(&self) -> bool;
}

impl InputEventKindCheck for InputEvent {
    fn is_type_key(&self) -> bool {
        match &self.kind() {
            InputEventKind::Key(_) => true,
            _ => false,
        }
    }
}

pub fn from_path(path: &str) -> Device {
    let devices = evdev::enumerate().map(|t| t.1).collect::<Vec<_>>();
    let mut device_index = 0;
    for (i, d) in devices.iter().enumerate() {
        if d.physical_path().is_some() {
            if d.physical_path().unwrap() == path {
                device_index = i;
                break;
            }
        }
    }
    devices.into_iter().nth(device_index).unwrap()
}

#[allow(dead_code)]
pub fn print_paths() {
    let devices = evdev::enumerate().map(|t| t.1).collect::<Vec<_>>();
    for d in devices.iter() {
        println!("{}", d.physical_path().unwrap());
    }
}
