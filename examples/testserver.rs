#![allow(warnings)]

use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

// TODO trait or struct for the server demo?
// TODO blocking or sync --> how does the official impl solve this issue?
// server encapsulates a tcp stream to listen and write to

fn main() {
    //    let endpoint = format!("127.0.0.1:{}", router::PORT_BASE);
    //    let listener = TcpListener::bind(endpoint.as_str()).unwrap();
    //    println!("listening started, ready to accept");
    //
    //    for stream in listener.incoming() {
    //        thread::spawn(|| {
    //            let mut stream = stream.unwrap();
    //            stream.write(b"Hello World\r\n").unwrap();
    //            let mut content = String::new();
    //            stream.read_to_string(&mut content);
    //            println!("{}", content);
    //        });
    //    }
}
