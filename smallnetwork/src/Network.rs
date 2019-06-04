use std::io;
pub struct Network<T> where T: FnMut(u32)->u32 {
    pub version: String,
    pub callback: T,
}
impl<T> Network<T> where T: FnMut(u32)->u32 {
    /*pub fn new() -> Self {
        println!("network created");
        Network {
            version: "1.0.0".to_string(),
        }
    }*/
    pub fn initialize(&mut self) {
        println!("rust smallnetwork library version={}", self.version);
        let result=(self.callback)(1000);
        println!("calback result={}", result);
    }
   
}

