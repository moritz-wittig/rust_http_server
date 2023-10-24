use crate::http::{Request, Response, StatusCode};
use std::io::{Read, Write};
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
                            
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                    Response::new(
                                        StatusCode::Ok,
                                        Some("<h1>IT WORKS</h1>".to_string()),
                                    )
                                }
                                Err(e) => {
                                    println!("Failed to parse a request: {}", e);
                                    Response::new(StatusCode::BadRequest, None)
                                }
                            }; 
                            if let Err(e) = response.send(&mut stream){
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
