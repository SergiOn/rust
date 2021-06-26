use std::net::TcpListener;
use std::convert::TryFrom;
use std::io::Read;
use super::http::Request;
use crate::http::{Response, StatusCode, ParseError};

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

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

    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _socket_addr)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            // Request::try_from(&buffer as &[u8]);
                            // Request::try_from(&buffer[..]);
                            // let result: &Result<Request, _> = &buffer.try_into();

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e),
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send a response: {}", e);
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
