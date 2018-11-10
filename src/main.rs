extern crate sphela;

fn main() {
    pretty_env_logger::init();
    println!("Starting sphela!");

    let config_path = sphela::args::setup().config_path;
    let config = sphela::configure::get(&config_path);
    sphela::server::run(&config);
}

