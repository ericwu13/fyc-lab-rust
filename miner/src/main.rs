use std::thread;
use std::net::{TcpStream, Shutdown};
use std::net::{IpAddr, Ipv4Addr};
use std::io::{Read, Write};
use solana_net_utils::bind_common;

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            if size != 0 {
                println!("Received transaction");
                stream.write(&data[0..size]).unwrap();
            }
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    // Listen to the web to see if there're any transactions that need to be processed
    // Get the blockchain from the nodes
    println!("This is miner node!!");

    let host_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let port: u16 = 3333;

    // start UDP and TCP server
    if let Ok((sock, listener)) = bind_common(host_v4, port, false) {
        println!("{:?}, {:?}", sock, listener);
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection: {}", stream.peer_addr().unwrap());
                    thread::spawn(move || {
                        // connection succeeded
                        handle_client(stream);
                    });
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }   println!("{:?}", listener);
    }
}
