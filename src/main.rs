#![allow(dead_code)]
mod app_state;
mod lib;
mod permission;
mod routes;
mod server;

use async_std::task;
use std::env;
use std::path::PathBuf;

use crate::lib::config::load_config_file;

fn main() {
    match main_result() {
        Ok(()) => async_std::process::exit(0),
        Err(e) => {
            println!("Error:");
            println!("{:#?}", e);
            async_std::process::exit(2)
        }
    }
}

fn main_result() -> Result<(), MainError> {
    let config_file_path = match env::args().nth(1) {
        Some(path) => PathBuf::from(path),
        None => panic!("Missing PATH command line argument"),
    };

    let config = task::block_on(load_config_file(&config_file_path));

    println!("Starting server: http://{}:{}/", config.ip, config.port);
    task::block_on(server::run(config)).map_err(MainError::ServerError)?;

    Ok(())
}

#[derive(Debug)]
enum MainError {
    ServerError(tide::Error),
}
