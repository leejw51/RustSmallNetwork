
use crate::myinfo::Information;
use std::io;
use crate::game::{Chat, ChatReply};
use protobuf::error::ProtobufError;
use protobuf::{parse_from_reader, ProtobufResult, parse_from_bytes};
use protobuf::{Message};
use smallnetwork::Network::{Network};
use std::sync::Arc;
use std::cell::RefCell;
use std::sync::Weak;



pub struct GameServer {
     pub version: &'static str ,
     pub net: Network,
}


pub trait UserControl  {
    fn initialize(&mut self);
    fn startServer(&mut self) ;
}

pub trait Program {
    fn run(&mut self);
}


impl Program for GameServer {
    fn run(&mut self) {
        (self.net.onReceived)(1);
        (self.net.onSent)(2);
    }
}
impl GameServer {
/*    pub fn test(&mut self) 
    {
        println!("test");
        let mut m = Chat::new();
        m.set_query("hello world".to_string());
        println!("{:?}", m);
        let   b=m.write_to_bytes().unwrap();
        println!("bytes= {:?}", b);
        let chat2: Chat = parse_from_bytes(& b).unwrap();
        println!("{:?}", chat2);
    }*/
    pub fn setup(&mut self) {
    //    self.test();
     
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


impl  UserControl for GameServer
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


