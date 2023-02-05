#![allow(unused)]

use std::env::{self, *};
use std::io::prelude::{self, *};
use std::net::{self, *};

fn handle_connection(socket: std::net::TcpListener, port: u32) -> std::io::Result<()> {
    for i in socket.incoming() {
        let mut stream: std::net::TcpStream = match i {
            Ok(_stream) => _stream,
            Err(err) => panic!("Failed To Create Server On Port {}", port),
        };
        let mut buffer: [u8; 1024] = [0; 1024];
        let html="<!DOCTYPE html><head><title>Rusty Server</title></head><body><h1>Hello, Rust!</h1><br><a href='https://github.com/0xN1nja' target='_blank'>Abhimanyu Sharma</a></body></html>";
        let response = format!("HTTP 1.1 200 OK\r\n\r\n{}", html);
        stream.read(&mut buffer);
        println!(
            "Request Headers :\n\n{}",
            String::from_utf8_lossy(&buffer).trim()
        );
        stream.write(response.as_bytes());
        stream.flush();
    }
    Ok(())
}

fn main() {
    let port: u32 = std::env::args().nth(1).unwrap().parse::<u32>().unwrap();
    let socket: std::net::TcpListener =
        std::net::TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    println!("Server Listening On http://localhost:{}/", port);
    handle_connection(socket, port).unwrap();
}
