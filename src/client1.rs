  
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use std::collections::HashMap;
use std::clone::Clone;


//static NTHREADS: i32 = 7;

fn handle_client(c:&mut HashMap<i32, TcpStream>,count:&i32){
    loop{
    let mut buf = [0u8; 512];
    c.get_mut(&count).unwrap().write_all(buf.as_ref()).unwrap();
    println!("data sent {}",count);
    }
}


fn main() {
    let mut c = HashMap::new();

    for i in 0..2 {
        let mut stream = TcpStream::connect("127.0.0.1:3000").unwrap();
        c.insert(i,stream);
    }
    for i in 0..2
    {
        let count =i;
        //handle_client(&mut c,& count);
        //let x: HashMap< i32 ,TcpStream> = c.iter().clone();//.collect();
        //x.clone_from(&c);
        thread::spawn(move|| handle_client(&mut c,& count));
        // thread::spawn(|| {
        // loop {
        //         let mut buf = [0u8; 512];
        //         let mut count = i;
        //         //println!("thread {}: Sending over ", i);
        //         c.get_mut(&count).unwrap().write_all(buf.as_ref()).unwrap();

        // }    
        // });
    }
   
}
