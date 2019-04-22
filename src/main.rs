extern crate liblurk;
extern crate rand;
extern crate uuid;

extern crate specs;
#[macro_use]
extern crate specs_derive;

extern crate clap;

extern crate ron;

#[macro_use]
extern crate serde;

extern crate sled;

use clap::{App, Arg};

use sled::Db;
use std::net::SocketAddr;
use std::time::Duration;
use game::load::entity_loader::EntityLoader;
use std::path::Path;
use game::load::location_loader::LocationLoader;
use game::load::constants_loader::ConstantsLoader;
use std::sync::mpsc::channel;
use std::thread;
use liblurk::server::lurk_server::execute_server;
use server::Server;

mod game;
mod server;

fn main() -> Result<(), String> {
    let behavior = server::Server::new()?;

    let matches = App::new("Server example liblurk-rs")
        .version("2.0")
        .author("Austin Jenkins")
        .about("Example server using liblurk-rs.")
        .arg(
            Arg::with_name("address")
                .short("a")
                .long("address")
                .value_name("ADDR")
                .help("Address to bind server to.")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    if let Ok(addr) = matches.value_of("address").unwrap().parse::<SocketAddr>() {
        let server = Server::new()?;
        execute_server(&addr, Duration::from_millis(100), server);
    } else {
        println!("Invalid address.");
    }

    Ok(())
}
