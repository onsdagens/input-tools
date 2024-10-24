use crate::{Devices, Keyboard, Mouse};
use windows_experiments::{Keyboard as WindowsKeyboard, Mouse as WindowsMouse};

impl Devices {
    pub fn new() -> Self {
        let mice = windows_experiments::get_devices::<WindowsMouse>();
        let keyboards = windows_experiments::get_devices::<WindowsKeyboard>();

        let mut mice_vec = vec![];
        let mut keyboards_vec = vec![];

        for mouse in mice {
            mice_vec.push(Mouse {
                name: mouse.product_name,
            });
        }

        for keyboard in keyboards {
            keyboards_vec.push(Keyboard {
                name: keyboard.product_name,
            });
        }

        Devices {
            mice: mice_vec,
            keyboards: keyboards_vec,
        }
    }
}
