use std::{os::unix::net::UnixStream, time::Duration};

use libusb::{Context, DeviceHandle};
use serde::{Deserialize, Serialize};
use unix_ipc_rs::SocketServer;

use crate::keyboards::{g15::KeyMapG15, DevConfig, KbVersion};

impl<'a> ServerDevice<'a> {
    fn print_and_kill(s: impl Into<String>) -> ! {
        println!("{}", s.into());
        std::process::exit(1);
    }
    pub fn new(ctx: &'a Context, kb: Option<KbVersion>) -> crate::error::Result<Self> {
        println!("[Info] Obtaining configuration.");
        if let Some(kb) = kb {
            let config = kb.server_config();
            println!("[Info] Searching for a {} keyboard.", config.friendly_name);

            for dev in ctx.devices()?.iter() {
                let ds = dev.device_descriptor()?;
                let id = format!("{:04x}:{:04x}", ds.vendor_id(), ds.product_id());

                if id == config.dev_name {
                    println!(
                        "[Info] Found a {} keyboard with id: {}",
                        config.friendly_name, id
                    );

                    println!("[Info] Opening connection to keyboard.");
                    let mut h = dev.open()?;
                    println!("[Info] Connection Opened.");

                    match dev.config_descriptor(config.configuration - 1) {
                        Ok(cd) => {
                            for i in 0..cd.num_interfaces() {
                                println!("[Info] Detaching Kernel drivers for interface: {}.", i);
                                if let Err(_) = h.detach_kernel_driver(i as u8) {
                                    println!(
                                        "[Info] Kernel driver for interface {} already detached.",
                                        i
                                    );
                                }
                            }
                        }
                        Err(e) => Self::print_and_kill(format!(
                            "[Error] Invalid config descriptor: {}\n{:?}",
                            config.configuration - 1,
                            e
                        )),
                    }

                    println!(
                        "[Info] Setting active configuration to: {}",
                        config.configuration
                    );
                    if let Err(e) = h.set_active_configuration(config.configuration) {
                        Self::print_and_kill(format!(
                            "[Error] Could not set active configuration: {}\n{e:?}.",
                            config.configuration
                        ));
                    }

                    println!("[Info] Claiming interface: {}", config.interface);
                    if let Err(e) = h.claim_interface(config.interface) {
                        Self::print_and_kill(format!(
                            "[Error] Could not claim interface {}\n{e:?}",
                            config.interface
                        ));
                    }

                    println!(
                        "[Info] Interface {} successfully claimed. ",
                        config.interface
                    );
                    println!("[Info] Setting up socket connection");

                    if let Ok(mut server) =
                        unix_ipc_rs::SocketServer::new("/tmp/logitech_mod_keys_rs.sock")
                    {
                        println!("[Info] Socket connection created");
                        return Ok(Self {
                            handler: h,
                            config,
                            socket: server,
                        });
                    } else {
                        println!("[Error] Could not open socket connection");
                        std::process::exit(1);
                    }
                }
            }
            Self::print_and_kill(format!(
                "[Error] Logitech {} Keyboard was not found.",
                config.friendly_name
            ));
        } else {
            Self::print_and_kill(format!(
                "[Error] You need to provide a keyboard model with -k arg [G15, G19]."
            ));
        }
    }

    pub fn read_interrupt(&mut self) -> [u8; 8] {
        let mut buf: [u8; 8] = [0; 8];
        let _ = self
            .handler
            .read_interrupt(self.config.endpoint, &mut buf, Duration::default());
        buf
    }
    pub fn read_interrupt_looped(&mut self) -> ! {
        println!("[Info] Waiting for keyevents");
        loop {
            let mut buf: [u8; 8] = [0; 8];
            let _ =
                self.handler
                    .read_interrupt(self.config.endpoint, &mut buf, Duration::default());

            self.socket.send::<Message>(Message(buf));
            println!("[Info] KeyEvent: {:#?} ", KeyMapG15::from_buffer(buf));
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message([u8; 8]);

pub struct ServerDevice<'a> {
    pub handler: DeviceHandle<'a>,
    pub config: DevConfig,
    pub socket: SocketServer,
}
