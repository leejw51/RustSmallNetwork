use std::io;
use std::rc::Rc;
use std::sync::Arc;
use std::cell::Cell;
use std::cell::RefCell;
use std::sync::Weak;

pub struct Network  {
    pub version: &'static str,
    //pub onConnected: T,
    //pub onDisconnected: T, 
    pub onReceived: Box<FnMut(i32)>,
    pub onSent: Box<FnMut(i32)>,
}


//---------------------------------------------------------------------
impl Network {
    pub fn initialize(&mut self) {
        println!("initialize= rust smallnetwork library version={}", self.version);
    }   
    pub fn read(&mut self) {
        (self.onReceived)(1);
    }
    pub fn write(&mut self) {
       (self.onSent)(2);
    }
}

