#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct SingleKeyAction {
    device_alias: String,
    code: u16,
}

impl SingleKeyAction {
    pub fn new(device_alias: String, code: u16) -> Self {
        Self { device_alias, code }
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum Action {
    SingleKeyAction(SingleKeyAction),
    // KeyChainAction,
    // Command,
}
