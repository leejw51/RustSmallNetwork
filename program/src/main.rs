extern crate  smallnetwork;
extern crate smallgameserver;
use smallnetwork::Network::{Network};
use smallgameserver::GameServer::{GameServer, UserControl};
fn main() {
    let mut server = GameServer::new();
    server.initialize();
    //server.menu();
    server.startServer();

}
