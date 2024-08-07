#![allow(unused)]
use std::{
    fmt::Display,
    fs::remove_file,
    io::{Read, Write},
    os::unix::net::{UnixListener, UnixStream},
    str::FromStr,
    thread::{sleep, spawn},
    time::Duration,
};

use args::{AppArgs, Mode};
use clap::Parser;
use keyboards::DevConfig;
use libusb::{Context, Device, DeviceHandle};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use server::ServerDevice;

mod args;
mod client;
mod error;
mod keyboards;
mod server;




fn main() -> crate::error::Result<()> {
       let ctx: Context = libusb::Context::new().unwrap();

       let args = AppArgs::parse();

       match args.mode {
           Mode::Server => ServerDevice::new(&ctx, args.keyboard)?.read_interrupt_looped(),
           Mode::Client => todo!(),
       }

    
    Ok(())
}


