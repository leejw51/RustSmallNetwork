
use crate::myinfo::Information;
use std::io;
use crate::game::{Chat, ChatReply};
use protobuf::error::ProtobufError;
use protobuf::{parse_from_reader, ProtobufResult, parse_from_bytes};
use protobuf::{Message};
use smallnetwork::Network::{Network};
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
        let mut m = Chat::new();
        m.set_query("hello world".to_string());
        println!("{:?}", m);
        let   b=m.write_to_bytes().unwrap();
        println!("bytes= {:?}", b);
        let chat2: Chat = parse_from_bytes(& b).unwrap();
        println!("{:?}", chat2);
    }
    pub fn setup(&mut self) {
      //  let info = myinfo::Information {info:"1.0.0"};
       // println!("game server version={}", self.version);
        self.test();

        let callback= |a| {
            println!("callback!={}",a);
            a+100
        };
        let mut p = Network{version:"1.0.0.".to_string(),
        callback: callback
        };
        p.initialize();
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
        println!("UsergControl initialize");
        self.setup();
    }
     fn startServer(&mut self) 
    {
        println!("UserControl StartServer");
        self.runServer();
    }
}