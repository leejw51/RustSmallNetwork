extern crate protoc_rust;
use protoc_rust::Customize;
use std::env;


pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len()<=1 {
        println!("specify proto files");
        return;
    }
    let mut files:Vec<&str>=vec![];
    for i in &args {
        files.push( i.as_str());
    }
    files.remove(0);
    println!("Compile:{:?}", args);
    println!("Files:{:?}", files);

    protoc_rust::run(protoc_rust::Args {
        out_dir: ".",
        //input: &["game.proto"],
        input: files.as_slice(),
        includes: &["protos"],
        customize: Customize {
            ..Default::default()
        },
    }).expect("protoc");
}
