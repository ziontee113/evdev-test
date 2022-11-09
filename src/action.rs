#[derive(Clone, Hash, PartialEq, Eq)]
pub struct SingleKeyAction {
    device_alias: String,
    code: u16,
}

impl SingleKeyAction {
    pub fn new(device_alias: String, code: u16) -> Self {
        Self { device_alias, code }
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum Action {
    SingleKeyAction(SingleKeyAction),
    // KeyChainAction,
    // Command,
}
