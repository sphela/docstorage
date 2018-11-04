#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate clap;

use clap::{Arg, App};
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    server: Option<ServerConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ServerConfig {
    ip: Option<String>,
    port: Option<u64>,
}

fn main() {
    println!("Starting sphela!");

    let matches = App::new("Sphela")
                          .version("0.0.1")
                          .author("Bjorn Tipling <bjorn@ambientchill.com>")
                          .about("the easy document store")
                          .arg(Arg::with_name("config")
                               .short("c")
                               .long("config")
                               .value_name("FILE")
                               .help("Sets the config file to use")
                               .takes_value(true))
                          .arg(Arg::with_name("v")
                               .short("v")
                               .multiple(true)
                               .help("Sets the log verbosity"))
                          .get_matches();

    let config_path = matches.value_of("config").unwrap_or("default.conf");
    println!("Using config at {}", config_path);
    let mut f = File::open(config_path).expect("config file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("With text:\n{}", contents);
}
