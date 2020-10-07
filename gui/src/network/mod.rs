use super::{Color, MainState, Overlay, Piece, Position, State};
use std::env;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str::from_utf8;
use std::thread;
use std::time::Duration;
pub struct Connection {
    stream: TcpStream,
}
impl Connection {
    pub fn init(color: Color) -> Self {
        match color {
            Color::White => {
                let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
                println!("Server listening on port 3333");
                for stream in listener.incoming() {
                    match stream {
                        Ok(mut stream) => {
                            stream.set_read_timeout(Some(Duration::new(0, 100000000)));
                            return Connection { stream };
                        }
                        Err(e) => {
                            panic!("Error: {}", e);
                        }
                    };
                }
            }
            Color::Black => match TcpStream::connect("localhost:3333") {
                Ok(mut stream) => {
                    stream.set_read_timeout(Some(Duration::new(0, 100000000)));
                    return Connection { stream };
                }
                Err(e) => {
                    println!("Failed to connect: {}", e);
                }
            },
            Color::None => (),
        }
        panic!("error");
    }
    pub fn get(&mut self) {
        let mut data = [0; 1];
        match self.stream.read(&mut data) {
            Ok(val) => println!("got move"),
            Err(e) => println!("nothing"),
        }
    }
    pub fn push(&mut self) {
        let mut data = [1; 1];
        let mut val: usize = self.stream.write(&mut data).unwrap();
    }
}
