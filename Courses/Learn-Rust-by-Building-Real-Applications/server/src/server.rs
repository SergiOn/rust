use std::net::TcpListener;
use std::convert::TryFrom;
use std::io::{Write, Read};
use super::http::Request;
use crate::http::{Response, StatusCode};

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
                Ok((mut stream, _socket_addr)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            // Request::try_from(&buffer as &[u8]);
                            // Request::try_from(&buffer[..]);
                            // let result: &Result<Request, _> = &buffer.try_into();

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                    // write!(stream, "HTTP/1.1 404 Not Found\r\n\r\n");
                                    // let response = Response::new(StatusCode::NotFound, None);
                                    Response::new(
                                        StatusCode::Ok,
                                        Some("<h1>It works</h1>".to_string())
                                    )
                                }
                                Err(e) => {
                                    println!("Failed to parse a request: {}", e);
                                    Response::new(StatusCode::BadRequest, None)
                                }
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
