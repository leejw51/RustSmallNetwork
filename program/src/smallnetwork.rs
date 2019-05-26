use std::io;
pub struct SmallNetwork {
    version: String,
}
impl SmallNetwork {
    pub fn new() -> Self {
        SmallNetwork {
            version: "1.0.0".to_string(),
        }
    }
    pub fn initialize(&mut self) {
        println!("rust smallnetwork library verson={}", self.version)
    }
    pub fn menu(&mut self) {
        let mut user = String::new();
        loop {
            let mut input = String::new();
            if user == "" {
                println!("1. start server");
                println!("2. start client");
                println!("q. exit");
            }
            io::stdin()
                .read_line(&mut input)
                .map(|e| {
                    print!("OK>{}", input);
                })
                .unwrap();
            user = input.trim().to_string();
            if user == "q" {
                println!("bye");
                break;
            }
        }
    }

}

