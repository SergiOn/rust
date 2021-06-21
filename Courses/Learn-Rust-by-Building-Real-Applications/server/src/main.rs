fn main() {
    let server = Server::new("127.0.0.1:8080");
    server.run();
    // println!("Hello, world!");
}

struct Server {
    addr: String,
}

impl Server {
    // fn new(addr: String) -> Server {
    //     Server {
    //         // addr: addr
    //         addr
    //     }
    // }

    fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    fn run(self) {

    }
}
