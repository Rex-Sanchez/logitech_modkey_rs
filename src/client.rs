use std::fs::read_to_string;

use unix_ipc_rs::IPCSocket;

use crate::args::Mode;
use crate::error::Result;
use crate::keyboards::g15::{G15Mod, G15KeyMap, G15ModModes};
use crate::keyboards::g19::{G19KeyMap, G19Mod, G19ModModes};
use crate::keyboards::KbVersion;
use crate::server::Message;

pub struct ClientProcess {
    socket: IPCSocket,
}

impl ClientProcess {
    pub fn new() -> Self {
        if let Ok(socket) = IPCSocket::new_client("/tmp/logitech_mod_keys_rs.sock") {
            Self { socket }
        } else {
            println!("[Error] Could not open socket connection");
            std::process::exit(1);
        }
    }

    pub fn process_key_events(&mut self, keyboard: Option<KbVersion>,config_path:&str) -> Result<()> {
        let config = read_to_string(config_path)?;
        if let Some(kb) = keyboard {
            match kb {
                KbVersion::G15 => {
                    let mut k = G15Mod::new(&config);
                    loop {
                        let keyevent = self.socket.recv_blocking::<Message>()?;
                        let key = G15KeyMap::from_buffer(keyevent.0);

                        match key {
                            G15KeyMap::M1 => k.set_mode(G15ModModes::M1),
                            G15KeyMap::M2 => k.set_mode(G15ModModes::M2),
                            G15KeyMap::M3 => k.set_mode(G15ModModes::M3),
                            G15KeyMap::MR => k.set_mode(G15ModModes::M),
                            G15KeyMap::Unknown => {}
                            _ => k.send_keyevent(&key),
                        }
                    }
                }
                KbVersion::G19 => {
                    let mut k = G19Mod::new(&config);
                    loop {
                        let keyevent = self.socket.recv_blocking::<Message>()?;
                        let key = G19KeyMap::from_buffer(keyevent.0);

                        match key {
                            G19KeyMap::M1 => k.set_mode(G19ModModes::M1),
                            G19KeyMap::M2 => k.set_mode(G19ModModes::M2),
                            G19KeyMap::M3 => k.set_mode(G19ModModes::M3),
                            G19KeyMap::MR => k.set_mode(G19ModModes::M),
                            G19KeyMap::Unknown => {}
                            _ => k.send_keyevent(&key),
                        }
                    }
                }
,
            }
        } else {
            println!("[Error] You need to supply a keyboard");
        }

        Ok(())
    }
}
