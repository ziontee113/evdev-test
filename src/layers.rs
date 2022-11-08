use std::collections::HashMap;

struct KeyPressUnit {
    device_alias: String,
    code: u16,
}

struct KeyChainAction {
    device_alias: String,
    keys: Vec<KeyPressUnit>,
}

struct CommandAction {
    command: String,
}

enum Action {
    KeyChainAction(KeyChainAction),
    Command(CommandAction),
}

enum Trigger {
    KeyPress(KeyPressUnit),
    KeyChain(Vec<KeyPressUnit>),
    // Union,
}

struct Layer {
    rules: HashMap<Trigger, Action>,
}

struct LayersLibrary {
    layers: HashMap<String, Layer>,
}
