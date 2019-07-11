extern crate mio;
use std::io::prelude::*;
use mio::*;
use mio::net::{TcpListener, TcpStream};
use mio::{Poll, Ready, PollOpt, Token};
use std::env;
use std::io;
use std::str;
use std::iter;
use std::iter::repeat;
use std::clone::Clone;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::io::{Read,Write,Error};

pub struct WorkerBuffer{
    pub buf : Vec<u8>,
    pub expected_size : usize,
    pub currently_read : usize,
}

impl WorkerBuffer 
{
    pub fn new() -> WorkerBuffer
    {
        WorkerBuffer { buf: Vec::new(), expected_size: 8, currently_read: 0}
    }
}


fn process_events(e:Event,listener:& TcpListener,poll:&Poll, clients:&mut HashMap<Token, TcpStream>,mut count: usize) {

    if (e.token()==Token(0) ) {
        
        match listener.accept(){
        Ok((mut stream,addr)) =>{

            let new_token = Token(count);
            println!("new token {:?}",new_token);
            
            poll.register(&stream,new_token,Ready::readable(),PollOpt::edge()|PollOpt::oneshot()).unwrap();
            
            println!("Got a client: {:?}",stream.peer_addr().unwrap());  

            clients.insert(new_token,stream);

        }
        Err(e) => panic!("Error during connection{}",e),

        }
        
    }

  if(e.token()!=Token(0) && e.readiness().is_readable()) {

        let mut buf =[0;512];
        
        let mut t = e.token();
        let reader = clients.get_mut(&e.token()).unwrap().read(&mut buf);

            match reader{
                Ok((_)) => {
                    let size = clients.keys().len();
                   // if (size==800){
                    let bytes_no = reader.unwrap();
                    println!("No of bytes read : {:?}, {:?}",bytes_no,e.token());
                    //println!("Client no {:?}",e.token());
                  //  }
                }
                Err(e)=>{
                    println!("could not read: {}",e);
                }

            }   

        poll.reregister(&clients[&e.token()],e.token(),Ready::readable(),PollOpt::edge()|PollOpt::oneshot()).unwrap();

    
    }

    
  //  }
    
}

fn main()
{
    const server: Token = Token(0);
   //let mut worker_buffers: Vec<_> = iter::repeat_with({|| WorkerBuffer::new() { expected:512}}).take(client_len).collect();

    let args: Vec<String> = env::args().collect();
    if args.len()!=2
    {
        eprintln!("Provide the argument");
        std::process::exit(1);
    }
    let listen = &args[1];
    let addr = listen.parse().unwrap();
    let mut listener = TcpListener::bind(&addr).expect("Could not bind");

    println!("Server listening on : {}",addr);

    let mut clients = HashMap::new();

    let mut eve = Events::with_capacity(1024);

    let poll = Poll::new().unwrap();

    let mut count = 1;

    poll.register(&listener,server,Ready::readable(),PollOpt::edge()|PollOpt::oneshot()).unwrap();

    loop{

        poll.poll(&mut eve, None);

        for e in &eve
            {
                process_events(e,& listener,&poll,&mut clients,count);
                if(e.token()==Token(0))
                {
                count+=1;
                }
                
            }
        poll.reregister(&listener,server,Ready::readable(),PollOpt::edge()|PollOpt::oneshot()).unwrap(); 
        }
     
}
