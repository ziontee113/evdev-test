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

    pub fn handle_incoming_input(&mut self, fragment: RawInputFragment) {
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

        self.parse_input();
        // self.detect_union();
    }

    fn parse_input(&self) {
        //
    }

    #[allow(dead_code)]
    fn detect_union(&self) {
        let total: u128 = self
            .fragments
            .iter()
            .map(|f| f.time.elapsed().unwrap().as_millis())
            .sum();

        // TODO: Union detector based on this
        if total < 30 && self.fragments.len() > 1 {
            println!("----------------------");
            println!("total time = {}", total);
            dbg!(&self.fragments);
        }
    }
}

impl RawInputStack {
    #[allow(dead_code)]
    pub fn print(&self) {
        dbg!(&self.fragments);
    }
}
