use map::Map;
use nickel::Request;
use nickel::{MediaType, Nickel};
use std::collections::HashMap;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use Player;
use std::str::FromStr;

pub struct RestService {
    server: Nickel,
}

impl RestService {
    pub fn new(
        map: Arc<Mutex<Map>>,
        players: Arc<Mutex<HashMap<Uuid, Player>>>,
    ) -> Result<RestService, ()> {
        let mut server = Nickel::new();

        let pplayers = players.clone();
        let pmap = map.clone();
        server.utilize(router! {
            get "/players" => |request, mut response| {
                response.set(MediaType::Html);
                get_players_report(pplayers.clone(), pmap.clone(), request)
            }
        });

        let rplayers = players.clone();
        let rmap = map.clone();
        server.utilize(router! {
            get "/rooms" => |_request, mut response| {
                response.set(MediaType::Html);
                get_rooms_report(rplayers.clone(), rmap.clone())
            }
        });

        Ok(RestService { server })
    }

    pub fn start(self, port: u16) {
        self.server
            .listen((IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port))
            .unwrap();
    }
}

fn get_player_report(player_id : &Uuid, players : Arc<Mutex<HashMap<Uuid, Player>>>, map : Arc<Mutex<Map>>) -> String {

    let players_guard = players.lock().unwrap();
    let map_guard = map.lock().unwrap();

    if let Some(player) = players_guard.get(player_id) {
        let player_section = format!("\
            <h3>Player</h3>
            <table>
                <tr>
                    <th>Name</th>
                    <th>Description</th>
                </tr>
                <tr>
                    <td>{}</td>
                    <td>{}</td>
                </tr>
            </table>
        ", player.entity_info.name,  player.entity_info.desc);

        let stat_section = format!("\
            <h3>Stats</h3>
            <table>
                <tr>
                    <th>Attack</th>
                    <th>Defense</th>
                    <th>Regen</th>
                    <th>Health</th>
                    <th>Gold</th>
                </tr>
                <tr>
                    <td>{}</td>
                    <td>{}</td>
                    <td>{}</td>
                    <td>{}</td>
                    <td>{}</td>
                </tr>
            </table>
        ", player.entity_info.attack, player.entity_info.defense, player.entity_info.regen, player.entity_info.health, player.entity_info.gold);

        let status_section = format!("\
            <h3>Status</h3>
            <table>
                <tr>
                    <th>Alive</th>
                    <th>Is Monster</th>
                    <th>Join Battle</th>
                    <th>Started</th>
                </tr>
                <tr>
                    <td>{}</td>
                    <td>{}</td>
                    <td>true</td>
                    <td>{}</td>
                </tr>
            </table>
        ", player.entity_info.alive, player.entity_info.monster, player.started.to_string());

        let location_section = {
            if let Some(room) = map_guard.get_room(&player.entity_info.location) {
                format!("\
                    <h3>Location<Location>
                    <table>
                        <tr>
                            <th>Name</th>
                            <th>Number</th>
                        </tr>
                        <tr>
                            <td>{}</td>
                            <td>{}</td>
                        </tr>
                    </table>
                ", room.get_name(), room.get_number())
            }
            else {
                "<h3>No location</h3>".to_string()
            }
        };

        player_section + &stat_section + &status_section + &location_section
    }
    else {
        return "No such player exists.\n".to_string()
    }
}

fn get_all_players_report(
    players: Arc<Mutex<HashMap<Uuid, Player>>>) -> String {
    let guard = players.lock().unwrap();

    let mut report = String::new();

    for (_, player) in guard.iter() {
        report.push_str(&format!("{}: {}", player.entity_info.name, player.id.to_string()));
    }

    report
}

fn get_rooms_report(players: Arc<Mutex<HashMap<Uuid, Player>>>, map: Arc<Mutex<Map>>) -> String {
    "Rooms report".to_string()
}

fn get_players_report(
    players: Arc<Mutex<HashMap<Uuid, Player>>>,
    map: Arc<Mutex<Map>>,
    request: &mut Request,
) -> String {
    if let Some(player_id) = request.param("id") {
        println!("Player report");
        if let Ok(uuid) = Uuid::from_str(player_id) {
            get_player_report(&uuid, players, map)
        }
        else {
            "Invalid id.\n".to_string()
        }
    } else {
        println!("Players report.");
        get_all_players_report(players, map)
    }
}
