  
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;


static NTHREADS: i32 = 750;

fn main() {
    let mut vec = Vec::new();

    for i in 0..NTHREADS {
        let mut stream = TcpStream::connect("127.0.0.1:3000").unwrap();
        vec.push(stream);
    }
    // loop {
    //             let msg = format!("the answer is {}", i);
    //             let mut buf = [0u8; 512];

    //             //println!("thread {}: Sending over ", i);
    //             stream.write_all(buf.as_ref()).unwrap();

    // }
}
