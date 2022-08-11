use std::env;

mod client;
mod server;
mod util;

fn main() {
    util::clear_console();
    let args: Vec<String> = env::args().collect();
    if let Some(arg) = args.get(1) {
        if arg.contains(&"host".to_string()) {
            server::host();
        } else {
            client::join(arg.to_owned());
        }
    }
}
