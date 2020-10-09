use super::super::{Color, MainState, Overlay, Piece, Position, State};
use std::env;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str::from_utf8;
use std::thread;
use std::time::Duration;
use std::sync::{
    Mutex,
    Arc,
};

pub fn rx_handler(stream: TcpStream, rx:  Arc<Mutex<Vec<u8>>>) {
    ()
}   
pub fn tx_handler(stream: TcpStream, tx: Arc<Mutex<Vec<u8>>>){
    loop{
        std::thread::sleep(Duration::from_millis(300));
        let mut val = tx.lock().unwrap();
        println!("{:?}", val);
    }
}