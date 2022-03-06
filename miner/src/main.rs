use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

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

    // start a TCP server

    let mut host: String = "0.0.0.0".to_owned();
    let port: &str = ":3333";

    host.push_str(port);

    println!("{}", &host);

    let listener = TcpListener::bind(&host).unwrap();

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
    }

    drop(listener);
}
