use input_tools::Devices;

fn main() {
    let devices = Devices::new();

    println!("Mice: ");
    for mouse in devices.mice {
        println!("{}", mouse.name);
    }

    println!("Keyboards: ");
    for keyboard in devices.keyboards {
        println!("{}", keyboard.name);
    }
}
