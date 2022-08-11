use std::{process, thread, time::Duration};

use console::Term;

pub const ADDRESS: &str = "127.0.0.1:65535";
pub const MESSAGE_SIZE: usize = 256;

pub fn clear_console() {
    Term::stdout().clear_screen().unwrap();
}

pub fn buffer_to_string(buffer: Vec<u8>) -> String {
    String::from_utf8(buffer.into_iter().take_while(|&x| x != 0).collect()).unwrap()
}

pub fn sleep() {
    thread::sleep(Duration::from_millis(100));
}

pub fn exit(message: &str) {
    println!("{}", message);
    process::exit(0);
}
