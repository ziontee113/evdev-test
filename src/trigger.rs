#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct TriggerKeyFragment {
    device_alias: String,
    code: u16,
}

impl TriggerKeyFragment {
    pub fn new(device_alias: String, code: u16) -> Self {
        Self { device_alias, code }
    }
}

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Trigger {
    KeyPress(TriggerKeyFragment),
    KeyChain(Vec<TriggerKeyFragment>),
    // Union,
}
