use std::time::SystemTime;

#[derive(Debug)]
pub struct RawInputFragment {
    pub device_alias: String,
    pub code: u16,
    pub value: i32,
    pub time: SystemTime,
}

#[derive(Debug)]
pub struct RawInputStack {
    fragments: Vec<RawInputFragment>,
}

impl RawInputStack {
    pub fn new() -> Self {
        Self { fragments: vec![] }
    }

    pub fn receive(&mut self, fragment: RawInputFragment) {
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

impl RawInputStack {
    pub fn print(&self) {
        dbg!(&self.fragments);
    }

    pub fn print_combined_time(&self) {
        let total: u128 = self
            .fragments
            .iter()
            .map(|f| f.time.elapsed().unwrap().as_millis())
            .sum();

        println!("total time = {}", total);
    }

    // TODO: base on .fragments:
    // - validate input type
    // - trigger action
    //      + has rules
    //      + raw action
}
