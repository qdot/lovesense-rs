extern crate liblovense;
#[macro_use]
extern crate clap;
use clap::{App};

fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("lovense-test-cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let port = matches.value_of("port").unwrap();
}
