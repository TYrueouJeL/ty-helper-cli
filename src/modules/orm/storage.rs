use std::fs;

use crate::modules::orm::connection::Connection;

pub fn save_connections(connections: &Vec<Connection>) {
    let json = serde_json::to_string_pretty(connections).unwrap();
    let path = "connections.json";
    
    fs::write(path, json).expect("Erreur écriture fichier");
}

pub fn load_connections() -> Vec<Connection> {
    let path = "connections.json";
    let data = fs::read_to_string(path);
    match data {
        Ok(content) => serde_json::from_str(&content).unwrap(),
        Err(_) => Vec::new()
    }
}