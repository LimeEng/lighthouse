use std::net::Ipv4Addr;
use std::net::TcpListener;
use std::process;

fn main() {
    match get_any_port() {
        Some(port) => println!("{}", port),
        None => process::exit(1),
    }
}

fn get_any_port() -> Option<u16> {
    TcpListener::bind((Ipv4Addr::LOCALHOST, 0))
        .and_then(|listener| listener.local_addr())
        .map(|addr| addr.port())
        .ok()
}
