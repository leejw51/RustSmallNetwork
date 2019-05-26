use std::io;
pub struct GameServer {
    version: String,
}
impl GameServer {
    pub fn new() -> Self {
        GameServer {
            version: "1.0.0".to_string(),
        }
    }
    pub fn initialize(&mut self) {
        println!("game server version={}", self.version)
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

