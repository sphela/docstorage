
use clap::{Arg, App};


pub struct Args {
    pub config_path: String,
}

pub fn setup() -> Args {
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

    let config_path = matches.value_of("config").unwrap_or("default.conf").to_string();
    Args {
        config_path
    }
}
