mod smallnetwork;
use smallnetwork::SmallNetwork;
fn main() {
    let mut m = SmallNetwork::new();
    m.initialize();
    m.menu(); 
}
