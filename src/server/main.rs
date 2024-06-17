//! # Server Application
//!
//! A simple TCP server application written in Rust that handles multiple clients and broadcasts messages to all connected clients.
//!
//! ## Constants
//! - `LOCAL`: The address and port where the server listens for connections.
//! - `MSG_SIZE`: The fixed size of the message buffer in bytes.

use std::io::ErrorKind;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

/// Sleeps for a fixed duration to prevent busy-waiting.
fn sleep() {
    thread::sleep(Duration::from_millis(100));
}

/// The main function that initializes the server and handles client connections and messages.
fn main() {
    // Bind the server to the local address.
    let server = TcpListener::bind(LOCAL).expect("Listener failed to bind");

    // Set the server to non-blocking mode to continuously check for new connections.
    server
        .set_nonblocking(true)
        .expect("Failed to initialize non-blocking server");

    // Vector to hold client sockets.
    let mut clients = vec![];

    // Create a channel for message passing between threads.
    let (tx, rx) = mpsc::channel::<String>();

    // Server loop to accept new clients and handle messages.
    loop {
        // Accept a new client connection if available.
        if let Ok((mut socket, addr)) = server.accept() {
            println!("Client {} connected", addr);

            // Clone the transmitter to send messages from the client handler thread.
            let tx = tx.clone();

            // Add the new client to the list of clients.
            clients.push(socket.try_clone().expect("Failed to clone client"));

            // Spawn a new thread to handle messages from the connected client.
            thread::spawn(move || loop {
                // Create a buffer to hold the incoming message.
                let mut buff = vec![0; MSG_SIZE];

                // Read the message from the client socket.
                match socket.read_exact(&mut buff) {
                    Ok(_) => {
                        // Convert the buffer to a UTF-8 string.
                        let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        let msg = String::from_utf8(msg).expect("Invalid UTF-8 message");

                        println!("{}: {:?}", addr, msg);
                        // Send the message to the receiver through the channel.
                        tx.send(msg).expect("Failed to send message to receiver");
                    }
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {
                        println!("Closing connection with: {}", addr);
                        break;
                    }
                }

                // Sleep to prevent busy-waiting.
                sleep();
            });
        }

        // Try to receive a message from the channel.
        if let Ok(msg) = rx.try_recv() {
            // Broadcast the message to all connected clients.
            clients = clients
                .into_iter()
                .filter_map(|mut client| {
                    // Create a buffer to hold the message.
                    let mut buff = msg.clone().into_bytes();
                    buff.resize(MSG_SIZE, 0);

                    // Write the message to the client socket.
                    client.write_all(&buff).map(|_| client).ok()
                })
                .collect::<Vec<_>>();
        }

        // Sleep to prevent busy-waiting.
        sleep();
    }
}
