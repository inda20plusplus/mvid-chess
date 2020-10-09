use super::{Color, MainState, Overlay, Piece, Position, State};
use std::env;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str::from_utf8;
use std::thread;
use std::time::Duration;
use std::sync::{
    Mutex,
    Arc,
    mpsc,
};
mod handler;
pub struct Connection {
    pub tx: Arc<Mutex<Vec<u8>>>,
    pub rx: Arc<Mutex<Vec<u8>>>
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
                            let mut rx_stream = stream;
                            let mut tx_stream = rx_stream.try_clone().unwrap();
                            let rx = Arc::new(Mutex::new(vec![]));
                            let tx = Arc::new(Mutex::new(vec![]));
                            std::thread::spawn(|| {
                                handler::rx_handler(rx_stream, tx.clone());
                            });
                            std::thread::spawn(|| {
                                handler::tx_handler(rx_stream, rx.clone());
                            });
                            return Connection {tx, rx};
                        }
                        Err(e) => {
                            panic!("Error: {}", e);
                        }
                    };
                }
            }
            Color::Black => match TcpStream::connect("localhost:3333") {
                Ok(mut stream) => {
                    let mut tx_stream = stream;
                    let mut rx_stream = tx_stream.try_clone().unwrap();
                    let rx = Arc::new(Mutex::new(vec![]));
                    let tx = Arc::new(Mutex::new(vec![]));
                    std::thread::spawn(|| {
                        handler::rx_handler(rx_stream, tx.clone());
                    });
                    std::thread::spawn(|| {
                        handler::tx_handler(rx_stream, rx.clone());
                    });
                    return Connection {tx, rx};
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
        match self.r_stream.read(&mut data) {
            Ok(val) => println!("got move"),
            Err(e) => println!("nothing"),
        }
    }
    pub fn push(&mut self) {
        let mut data = [1; 1];
        let mut val: usize = self.w_stream.write(&mut data).unwrap();
    }
}
