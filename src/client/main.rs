//! # Client Application
//!
//! A simple TCP client application written in Rust that connects to a server, sends messages, and receives broadcasts.
//!
//! ## Constants
//! - `LOCAL`: The address and port where the client connects to the server.
//! - `MSG_SIZE`: The fixed size of the message buffer in bytes.

use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

/// The main function that connects to the server and handles message sending and receiving.
fn main() {
    // Connect to the server.
    let mut client = TcpStream::connect(LOCAL).expect("Stream failed to connect");

    // Set the client to non-blocking mode to continuously check for messages.
    client
        .set_nonblocking(true)
        .expect("Failed to initialize non-blocking client");

    // Create a channel for message passing between threads.
    let (tx, rx) = mpsc::channel::<String>();

    // Spawn a new thread to handle incoming and outgoing messages.
    thread::spawn(move || loop {
        // Create a buffer to hold the incoming message.
        let mut buff = vec![0; MSG_SIZE];

        // Read the message from the server.
        match client.read_exact(&mut buff) {
            Ok(_) => {
                // Convert the buffer to a UTF-8 string.
                let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                // let msg = String::from_utf8(msg).expect("Invalid UTF-8 message");

                println!("Message recv: {:?}", msg);
            }
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("Connection with server was terminated");
                break;
            }
        }

        // Try to receive a message from the channel.
        match rx.try_recv() {
            Ok(msg) => {
                // Create a buffer to hold the message.
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);

                // Write the message to the server.
                client.write_all(&buff).expect("Writing to socket failed");
                println!("Message sent {:?}", msg);
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break,
        }

        // Sleep to prevent busy-waiting.
        thread::sleep(Duration::from_millis(100));
    });

    println!("Write a message: ");

    // Main loop to read user input and send messages to the server.
    loop {
        let mut buff = String::new();
        io::stdin()
            .read_line(&mut buff)
            .expect("Reading from stdin failed");
        let msg = buff.trim().to_string();
        if msg.to_lowercase() == ":quit" || tx.send(msg).is_err() {
            break;
        }
    }

    println!("Exiting chat program!")
}
