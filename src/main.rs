mod game;
mod deck;
mod network;

use tokio::runtime::Runtime;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut rt = Runtime::new().unwrap();

    if args.len() < 2 {
        eprintln!("Usage: {} [server|client] [options]", args[0]);
        return;
    }

    match args[1].as_str() {
        "server" => {
            if args.len() != 3 {
                eprintln!("Usage: {} server [starting_cash]", args[0]);
                return;
            }
            let starting_cash: u64 = args[2].parse().unwrap();
            rt.block_on(network::start_server("127.0.0.1:8080", starting_cash));
        }
        "client" => {
            if args.len() != 3 {
                eprintln!("Usage: {} client [name]", args[0]);
                return;
            }
            let name = args[2].clone();
            rt.block_on(network::start_client("127.0.0.1:8080", name));
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
        }
    }
}
