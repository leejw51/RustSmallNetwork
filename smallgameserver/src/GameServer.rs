
use crate::myinfo::Information;
use std::io;
pub struct GameServer {
    version: String,
}
pub trait UserControl  {
    fn initialize(&mut self);
    fn startServer(&mut self) ;
}
//---------------------------------------------------------------------------------------------------
impl GameServer {
    pub fn new() -> Self {
        GameServer {
            version: "1.0.0".to_string(),
        }
    }
    pub fn test(&mut self) 
    {
        println!("test");
    }
    pub fn setup(&mut self) {
      //  let info = myinfo::Information {info:"1.0.0"};
        println!("game server version={}", self.version);
        self.test();
    }
    pub fn runServer(&mut self) {
        println!("server");
    }
    pub fn menu2(&mut self) {
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

impl UserControl for GameServer 
{
    fn initialize(&mut self) 
    {
        println!("UserControl initialize");
        self.setup();
    }
     fn startServer(&mut self) 
    {
        println!("UserControl StartServer");
        self.runServer();
    }
}