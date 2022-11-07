#[derive(Debug)]
pub struct RawEventFragment {
    pub device_alias: String,
    pub code: u16,
    pub value: i32,
}

#[derive(Debug)]
pub struct RawEventStack {
    fragments: Vec<RawEventFragment>,
}

impl RawEventStack {
    pub fn new() -> Self {
        Self { fragments: vec![] }
    }

    pub fn receive(&mut self, fragment: RawEventFragment) {
        match fragment.value {
            0 => {
                let i = self.fragments.iter().position(|f| {
                    f.device_alias == fragment.device_alias && f.code == fragment.code
                });
                if i.is_some() {
                    self.fragments.remove(i.unwrap());
                }
            }
            1 => self.fragments.push(fragment),
            _ => (),
        }
    }
}

impl RawEventStack {
    pub fn print(&self) {
        dbg!(&self.fragments);
    }
}
