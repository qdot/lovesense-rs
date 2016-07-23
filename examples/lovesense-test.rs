extern crate lovesense;
#[macro_use]
extern crate clap;
use clap::{App};
use lovesense::{LovesenseDevice};

macro_rules! cmd {
    (
        $a: expr, $b:expr, $c:expr, $d:expr
    ) => {
        if $a.is_present($b) {
            match $c {
                Ok(s) => println!($d, s),
                Err(s) => println!("{}", s)
            }
        }
    }
}


fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("lovesense-test-cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let port = matches.value_of("port").unwrap();
    println!("Connecting to {}...", port);
    let mut d = LovesenseDevice::new(port);

    cmd!(matches, "info", d.device_type(), "Device: {}");
    cmd!(matches, "battery", d.battery_level(), "Battery Level: {}");
    cmd!(matches, "status", d.device_status(), "Status: {}");
    cmd!(matches, "power_off", d.power_off(), "Device powered off: {}");
    cmd!(matches, "change_direction", d.change_rotation_direction(), "Rotation Direction Changed: {}");

    if matches.is_present("speed") {
        let speed = value_t!(matches, "speed", u8).unwrap_or(0);
        match d.set_vibrate(speed) {
            Ok(_) => println!("Speed set to {}", speed),
            Err(s) => println!("{}", s)
        }
    }

    if matches.is_present("rotate") {
        let speed = value_t!(matches, "rotate", u8).unwrap_or(0);
        match d.set_rotate(speed) {
            Ok(_) => println!("Rotate set to {}", speed),
            Err(s) => println!("{}", s)
        }
    }
}
