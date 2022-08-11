use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, Receiver};
use std::thread;

use crate::util::*;

pub fn join(name: String) {
    let mut stream = TcpStream::connect(ADDRESS).unwrap();
    stream.set_nonblocking(true).unwrap();

    let mut name_buffer = Vec::from(name);
    name_buffer.resize(MESSAGE_SIZE, 0);
    if stream.write(&name_buffer).is_err() {
        exit("SOMETHING BAD HAPPENED");
    }

    let (sender, receiver) = mpsc::channel();

    thread::spawn(|| receive_from_server(stream, receiver));

    loop {
        let mut message = String::new();
        io::stdin().read_line(&mut message).unwrap();
        if sender.send(message.trim().to_string()).is_err() {
            exit("SOMETHING BAD HAPPENED");
        }
    }
}

fn receive_from_server(mut server: TcpStream, receiver: Receiver<String>) {
    loop {
        let mut buffer = vec![0; MESSAGE_SIZE];
        match server.read_exact(&mut buffer) {
            Ok(_) => {
                let message = buffer_to_string(buffer);
                println!("{}", message);
            }
            Err(ref error) if error.kind() == ErrorKind::WouldBlock => (),
            Err(_) => exit("SERVER DISCONNECTED"),
        }

        if let Ok(message) = receiver.try_recv() {
            let mut buffer = message.into_bytes();
            buffer.resize(MESSAGE_SIZE, 0);
            server.write_all(&buffer).unwrap();
        }

        sleep();
    }
}
