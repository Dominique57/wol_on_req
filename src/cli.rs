use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub names: Vec<Name>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Name {
    pub name: String,
    pub mac_address: macaddr::MacAddr,
    pub ip_address: std::net::Ipv4Addr,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to the configuration file
    #[arg(short, long)]
    pub file: String,
}
