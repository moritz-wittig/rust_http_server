use crate::http::{ParseError, Request, Response, StatusCode};
use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    adress: String,
}

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}
impl Server {
    pub fn new(adress: String) -> Self {
        Self { adress }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.adress);
        let listener = TcpListener::bind(self.adress).unwrap();
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024]; // 1KB of memory allocated
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e),
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
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
