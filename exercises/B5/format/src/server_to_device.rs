use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ServerToDevice {
    pub set_led_status: Option<(u8, bool)>,
    pub send_acc_data: bool,
    pub say_hello: bool,
    // TODO add you own commands here for the device to handle
}
