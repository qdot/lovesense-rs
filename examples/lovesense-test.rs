extern crate lovesense;
#[macro_use]
extern crate clap;
use clap::{App};
use lovesense::{LovesenseDevice};

fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("lovesense-test-cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let port = matches.value_of("port").unwrap();
    println!("Connecting to {}...", port);
    let mut d = LovesenseDevice::new(port);
    let speed = value_t!(matches, "speed", u8).unwrap_or(0);
    let _ = d.set_vibrate(speed);
    if matches.is_present("info") {
        let result = d.battery_level();
        match result {
            Ok(s) => println!("Device: {}", s),
            Err(s) => println!("{}", s)
        }
    }
}
