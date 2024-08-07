use unix_ipc_rs::SocketClient;

use crate::server::Message;
use crate::error::Result;


pub struct ClientProcess {
    socket: SocketClient,
}

impl ClientProcess {
    fn new() -> Self {
        if let Ok(socket) = SocketClient::new("/tmp/logitech_modkeys.sock") {
            Self { socket }
        } else {
            println!("");
            std::process::exit(1);
        }
    }
    fn process_key_events(&mut self)->  Result<()>{
        let keyevent = self.socket.recv_blocking::<Message>();



        Ok(())
        
    }
}
