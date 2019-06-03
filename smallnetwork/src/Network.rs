use std::io;
pub struct Network {
    version: String,
}
impl Network {
    pub fn new() -> Self {
        println!("network created");
        Network {
            version: "1.0.0".to_string(),
        }
    }
    pub fn initialize(&mut self) {
        println!("rust smallnetwork library version={}", self.version)
    }
   
}

