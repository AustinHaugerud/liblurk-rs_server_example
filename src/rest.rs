use map::{Map, Room};
use nickel::{Nickel, HttpRouter, MediaType};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use uuid::Uuid;
use Player;

pub struct RestService {
    server : Nickel,
    map : Arc<Mutex<Map>>,
    players: Arc<Mutex<HashMap<Uuid, Player>>>,
}

impl RestService {
    pub fn new(map : Arc<Mutex<Map>>, players : Arc<Mutex<HashMap<Uuid, Player>>>) -> Result<RestService, ()> {
        let mut server = Nickel::new();

        // List players
        server.utilize(router! {
            get "/players" => |_request, mut response| {
                response.set(MediaType::Html);
                let players = players.lock().unwrap();
                players.len().to_string()
            }
        });

        Err(())
    }
}
