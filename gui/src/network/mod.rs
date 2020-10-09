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
                            let nrx = rx.clone();
                            let ntx = tx.clone();
                            std::thread::spawn(|| {
                                handler::rx_handler(rx_stream, nrx);
                            });
                            std::thread::spawn(|| {
                                handler::tx_handler(tx_stream, ntx);
                            });
                            return Connection{rx, tx};
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
                    let nrx = rx.clone();
                    let ntx = tx.clone();
                    std::thread::spawn(|| {
                        handler::rx_handler(rx_stream, nrx);
                    });
                    std::thread::spawn(|| {
                        handler::tx_handler(tx_stream, ntx);
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
    
}
