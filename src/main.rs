extern crate liblurk;
extern crate rand;
extern crate uuid;

/*#[macro_use]
extern crate nickel;*/

extern crate specs;

extern crate clap;

use liblurk::server::lurk_server::Server;
use clap::{App, Arg};
use std::net::SocketAddr;
use std::time::Duration;

mod game;
mod server;

fn main() {

    let behavior = server::Server;

    let matches = App::new("Server example liblurk-rs")
        .version("2.0")
        .author("Austin Jenkins")
        .about("Example server using liblurk-rs.")
        .arg(Arg::with_name("address")
            .short("a")
            .long("address")
            .value_name("ADDR")
            .help("Address to bind server to.")
            .takes_value(true)
            .required(true))
        .get_matches();


    if let Ok(addr) = matches.value_of("address").unwrap().parse::<SocketAddr>() {
        if let Ok(mut server) = Server::create(
            addr,
            Duration::from_secs(30),
            Duration::from_millis(100),
            1000,
            behavior
        ) {
            println!("Starting server.");
            server.start();
        } else {
            println!("Failed to start server.");
        }
    }
    else {
        println!("Invalid address.");
    }
}
