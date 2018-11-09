extern crate hyper;
extern crate pretty_env_logger;
extern crate tokio;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate clap;

mod args;
mod configure;
mod server;

fn main() {
    pretty_env_logger::init();
    println!("Starting sphela!");

    let config_path = args::setup().config_path;
    let config = configure::get(&config_path);
    server::run(&config);
}

