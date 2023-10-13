use crate::http::Request;
use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    adress: String,
}

impl Server {
    pub fn new(adress: String) -> Self {
        Self { adress }
    }

    pub fn run(self) {
        println!("Listening on {}", self.adress);
        let listener = TcpListener::bind(self.adress).unwrap();
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024]; // 1KB of memory allocated
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                }
                                Err(e) => {
                                    println!("Failed to parse a request: {}", e)
                                }
                            };
                        }
                        Err(e) => {
                            println!("Failed to read from connection: {e}")
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to establish a new connection: {}", e);
                }
            }

            let res = listener.accept();
            if res.is_err() {
                continue;
            }
            let (_stream, _addr) = res.unwrap();
        }
    }
}
