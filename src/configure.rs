use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct RawConfig {
    server: Option<RawServerConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
struct RawServerConfig {
    port: Option<u16>,
    ipv_4_addr: Option<String>,
    doc_root: Option<String>,
}

pub struct Config {
    pub server: ServerConfig,
}

pub struct ServerConfig {
    pub addr: std::net::SocketAddr,
    pub doc_root: String,
}

pub fn get(config_path: &String) -> Config {
    println!("Using config at {}", config_path);
    let mut f = File::open(config_path).expect("config file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    let decoded_config: RawConfig = toml::from_str(contents.as_str())
        .expect("problem reading config");
    let server = decoded_config.server
        .expect("Expected server in the config");

    // Build address.
    let ipv_4_addr: String = server.ipv_4_addr
        .expect("Expected `ipv_6_addr` in the config");
    let ipv_4_addr = ipv_4_addr.split(".")
        .map(|s| s.parse::<u8>().expect("Expected ipv4 to be made from numbers"))
        .collect::<Vec<u8>>();
    let ipv_4_addr: [u8; 4] = [
        ipv_4_addr[0],
        ipv_4_addr[1],
        ipv_4_addr[2],
        ipv_4_addr[3],
    ];
    let port = server.port
        .expect("Expected `port` in the config");
    let addr = (ipv_4_addr, port).into();

    // Doc root.
    let doc_root = server.doc_root
        .expect("Expected `dock_root` in the config");

    let server = ServerConfig{ addr, doc_root };
    Config { server }
}
