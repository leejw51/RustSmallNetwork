extern crate  smallnetwork;
extern crate smallgameserver;
use smallnetwork::Network::{Network};
use smallgameserver::GameServer::{GameServer};
fn main() {
    let mut m = GameServer::new();
    m.initialize();
    m.menu(); 
}
