mod schema;
mod server;
use argh::FromArgs;
use std::net::{AddrParseError, SocketAddrV4};
use tokio;

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
        .unwrap_or(Ok(std::net::Ipv4Addr::LOCALHOST))
        .map_err(MainError::AddrParseError)?;
    let port = args.port.unwrap_or(3000);
    let socket = SocketAddrV4::new(ip, port);

    let rt = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    rt.block_on(server::run(socket.into()))
        .map_err(MainError::ServerError)?;

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
    ServerError(std::io::Error),
}
