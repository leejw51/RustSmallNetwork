extern crate  smallnetwork;
extern crate smallgameserver;
use std::sync::Arc;
use std::cell::RefCell;
use std::sync::Weak;
use smallnetwork::Network::{Network};
use smallgameserver::GameServer::{GameServer, UserControl, Program};
fn main() {
    let dummy= |a:i32|{println!("print={}", a);};
    let net= Network{
        version:"1.0.1",
        onReceived: Box::new(|a| {println!("onRecieved {}",a);}),
        onSent:Box::new(|a| {println!("onSent {}", a);}), 
    };
    let mut server= Box::new(GameServer{
        version:"1.0.0",
        net:net,
    });
    server.run();
}
