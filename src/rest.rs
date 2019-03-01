use map::{Map, Room};
use nickel::{Nickel, HttpRouter, MediaType};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use uuid::Uuid;
use Player;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::thread;

pub struct RestService {
    server : Nickel,
}

impl RestService {
    pub fn new(map : Arc<Mutex<Map>>, players : Arc<Mutex<HashMap<Uuid, Player>>>) -> Result<RestService, ()> {
        let mut server = Nickel::new();

        // List players
        server.utilize(router! {
            get "/players" => |_request, mut response| {
                response.set(MediaType::Html);
                get_players_report(players.clone())
            }
        });

        Ok(RestService { server })
    }

    pub fn start(self, port : u16) {
        self.server.listen((IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port)).unwrap();
    }
}

fn get_players_report(players : Arc<Mutex<HashMap<Uuid, Player>>>) -> String {
    let guard = players.lock().unwrap();

    let mut report = String::new();

    for (id, player) in guard.iter() {
        report.push_str(&format!("{} - {}", player.entity_info.name, player.id.to_string()));
    }

    report
}
