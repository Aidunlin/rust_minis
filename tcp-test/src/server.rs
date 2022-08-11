use std::io::{ErrorKind, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{self, Sender};
use std::thread;

use crate::util::*;

pub fn host() {
    let server = TcpListener::bind(ADDRESS).unwrap();
    server.set_nonblocking(true).unwrap();

    let (sender, receiver) = mpsc::channel();

    let mut clients = Vec::new();

    loop {
        if let Ok((mut client, _)) = server.accept() {
            let mut client_name = vec![0; MESSAGE_SIZE];
            match client.read_exact(&mut client_name) {
                Ok(_) => {
                    clients.push(client.try_clone().unwrap());
                    let client_name = buffer_to_string(client_name);

                    let message = format!("SERVER: {} connected", client_name);
                    println!("{}", message);
                    sender.send(message).unwrap();

                    let sender = sender.clone();
                    thread::spawn(|| handle_client(client, client_name, sender));
                }
                Err(_) => exit("SOMETHING BAD HAPPENED"),
            }
        }

        if let Ok(message) = receiver.try_recv() {
            clients = clients
                .into_iter()
                .filter_map(|mut client| {
                    let mut buffer = message.clone().into_bytes();
                    buffer.resize(MESSAGE_SIZE, 0);
                    client.write_all(&buffer).map(|_| client).ok()
                })
                .collect();
        }

        sleep();
    }
}

fn handle_client(mut client: TcpStream, client_name: String, sender: Sender<String>) {
    loop {
        let mut buffer = vec![0; MESSAGE_SIZE];
        match client.read_exact(&mut buffer) {
            Ok(_) => {
                let message = format!("{}: {}", client_name, buffer_to_string(buffer));
                println!("{}", message);
                sender.send(message).unwrap();
            }
            Err(ref error) if error.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                let message = format!("SERVER: {} disconnected", client_name);
                println!("{}", message);
                sender.send(message).unwrap();
                break;
            }
        }
        sleep();
    }
}
