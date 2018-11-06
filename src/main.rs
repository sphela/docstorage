#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate pretty_env_logger;

extern crate clap;

use clap::{Arg, App};
use std::fs::File;
use std::io::prelude::*;

use hyper::{Body, Response, Server};
use hyper::service::service_fn_ok;
use hyper::rt::{self, Future};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    server: Option<ServerConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ServerConfig {
    port: Option<u16>,
    ipv_4_addr: Option<String>,
}

fn main() {
    pretty_env_logger::init();
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
    let decoded_config: Config = toml::from_str(contents.as_str()).expect("problem reading config");
    let server = decoded_config.server.expect("Expected server in the config");
    let ipv_4_addr: String = server.ipv_4_addr.expect("Expected `ipv_6_addr` in the config");
    println!("what is this: {:?}", ipv_4_addr);
    let ipv_4_addr = ipv_4_addr.split(".")
        .map(|s| s.parse::<u8>().expect("Expected ipv4 to be made from numbers"))
        .collect::<Vec<u8>>();
    let ipv_4_addr: [u8; 4] = [
        ipv_4_addr[0],
        ipv_4_addr[1],
        ipv_4_addr[2],
        ipv_4_addr[3],
    ];
    let port = server.port.expect("Expected `port` in the config");
    println!("and what is this {:?} and port {:?}", ipv_4_addr, port);

    let addr = (ipv_4_addr, port).into();

    const PHRASE: &'static [u8] = b"Hello World!";

    let new_service = || {
             service_fn_ok(|_| {
            Response::new(Body::from(PHRASE))
        })
    };

    let server = Server::bind(&addr)
        .serve(new_service)
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);

    rt::run(server);
}
