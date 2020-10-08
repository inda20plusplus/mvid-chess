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

pub fn rx_handler(stream: TcpStream, rx: mpsc::Sender ) {
    ()
}   
pub fn tx_handler(stream: TcpStream, tx: mpsc::Receiver){
    ()
}