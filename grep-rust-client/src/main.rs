extern crate ws;

use ws::{connect, Handler, Sender, Handshake, Result, Message, CloseCode};

struct Client {
    out: Sender,
}

impl Handler for Client {

    fn on_open(&mut self, _: Handshake) -> Result<()> {
        self.out.send("Hello WebSocket!")
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("Got message: {}", msg);
        self.out.close(CloseCode::Normal)
    }    
}

fn main() {
    connect("ws://0.0.0.0:3000", |out| Client { out: out } ).unwrap()
//   connect("ws://104.196.24.172:3000", |out| Client { out: out } ).unwrap()
} 
