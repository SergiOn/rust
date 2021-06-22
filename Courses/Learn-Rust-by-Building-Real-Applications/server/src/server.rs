use std::net::{TcpListener, SocketAddr, TcpStream};
use std::convert::{TryFrom, TryInto};
use std::io::Read;
use std::io::Error;
use crate::http::Request;

pub struct Server {
    addr: String,
}

impl Server {
    // fn new(addr: String) -> Server {
    //     Server {
    //         // addr: addr
    //         addr
    //     }
    // }

    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut tcpStream, socketAddr)) => {
                    let mut buffer = [0; 1024];
                    match tcpStream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            // Request::try_from(&buffer as &[u8]);
                            // Request::try_from(&buffer[..]);
                            // let result: &Result<Request, _> = &buffer.try_into();

                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {}
                                Err(e) => println!("Failed to parse a request: {}", e)
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish a connection: {}", e)
            }

            // let res = listener.accept();
            //
            // if res.is_err() {
            //     continue;
            // }
            //
            // let (tcpStream, socketAddr) = res.unwrap();
        }
    }
}
