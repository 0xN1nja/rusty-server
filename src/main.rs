#![allow(unused)]

use std::net::{*,self};
use std::io::prelude::{*,self};
use std::env::{*,self};

fn main(){
    let port:u32=env::args().nth(1).unwrap().parse::<u32>().unwrap();
    let socket:TcpListener=TcpListener::bind(format!("127.0.0.1:{}",port)).unwrap();
    println!("Server Listening On https://127.0.0.1:{}",port);
    for i in socket.incoming(){
        let stream:TcpStream=match i{
            Ok(_stream)=>_stream,
            Err(err)=>panic!("Failed To Create Server On Port {}",port),
        };
    }
}