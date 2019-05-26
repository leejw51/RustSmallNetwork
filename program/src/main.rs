extern crate  smallnetwork;
use smallnetwork::Network::{Network};

fn main() {
    let mut m = Network::new();
    m.initialize();
    m.menu(); 
}
