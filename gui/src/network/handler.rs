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

pub fn rx_handler(mut stream: TcpStream, rx:  Arc<Mutex<Vec<u8>>>) {
    loop{
        std::thread::sleep(Duration::from_millis(300));
        let mut buffer = &mut [0; 1];
        stream.read(buffer);
        let mut val = rx.lock().unwrap();
        val.push(buffer[0]);
    }
}   
pub fn tx_handler(mut stream: TcpStream, tx: Arc<Mutex<Vec<u8>>>){
    loop{
        std::thread::sleep(Duration::from_millis(300));
        let mut val = tx.lock().unwrap();
        if val.len() > 0{
            for i in 0..val.len(){ 
                stream.write(&[val[i]]);
            }
        }
        val.clear();
    }
}