#![allow(dead_code)]
mod app_state;
mod lib;
mod permission;
mod routes;
mod server;
mod timeline;
mod types;

use argh::FromArgs;
use async_std::task;
use std::net::{AddrParseError, SocketAddr, SocketAddrV4};

const DEFAULT_ADDR: std::net::Ipv4Addr = std::net::Ipv4Addr::LOCALHOST;

fn main() {
    match main_result() {
        Ok(()) => std::process::exit(0),
        Err(e) => {
            println!("Error:");
            println!("{:#?}", e);
            std::process::exit(2)
        }
    }
}

fn main_result() -> Result<(), MainError> {
    let args: Args = argh::from_env();
    let ip = args
        .ip
        .map(|ip| ip.parse())
        .unwrap_or(Ok(DEFAULT_ADDR))
        .map_err(MainError::AddrParseError)?;
    let port = args.port.unwrap_or(3000);
    let socket = SocketAddr::V4(SocketAddrV4::new(ip, port));

    println!("Starting server: http://{}/", socket);
    task::block_on(server::run(socket)).map_err(MainError::ServerError)?;

    Ok(())
}

#[derive(FromArgs)]
/// Start server
struct Args {
    /// the port of the http server
    #[argh(option)]
    port: Option<u16>,

    /// the ip address of the http server
    #[argh(option)]
    ip: Option<String>,
}

#[derive(Debug)]
enum MainError {
    AddrParseError(AddrParseError),
    ServerError(tide::Error),
}
