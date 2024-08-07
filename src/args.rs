use std::str::FromStr;

use clap::Parser;

use crate::keyboards::KbVersion;

impl FromStr for Mode {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "server" => Ok(Mode::Server),
            "client" => Ok(Mode::Client),
            _ => Err("Mode take one of 3 options, [server, client, info]"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Mode {
    Server,
    Client,
}

#[derive(Parser, Debug)]
#[command(version,about,long_about = None)]
pub struct AppArgs {
    /// Operation mode: [server, client].
    #[arg(short)]
    pub mode: Mode,
 
    // Device name (VendorId:DeviceId) obtain with lsusb.
    #[arg(short)]
    pub device: Option<String>,


    /// Logitch keyboard model: [G15, G19].
    #[arg(short)]
    pub keyboard: Option<KbVersion>,
}
