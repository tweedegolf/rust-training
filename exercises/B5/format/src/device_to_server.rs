use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DeviceToServer {
    pub led_status: Option<(u8, bool)>,
    pub said_hello: bool,
    // TODO add you own fields here for the CLI to handle
}
