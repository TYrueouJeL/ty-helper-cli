use inquire::Select;
use mysql::Pool;
use serde::{Deserialize, Serialize};
use crate::modules::{orm::mysql::{execute_query_mysql, list_tables_mysql}, utils::read_input};
use super::storage::{load_connections, save_connections};
use colored::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Connection {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
}

pub fn list_connections() -> Result<(), String> {
    let connections = load_connections();

    println!("=== Liste des connections ===");

    if connections.is_empty() {
        println!("{}", format!("Aucune connexion disponible").yellow());
        return Err("Aucune connexion disponible".to_string());
    }

    for (index, connection) in connections.iter().enumerate() {
        print!("[{}] ", index);
        println!("{}", format!("{} - {} - {}:{}", connection.name, connection.database, connection.host, connection.port).cyan());
    }
    
    Ok(())
}

pub fn create_connection() -> Result<(), String> {
    let mut connections = load_connections();

    let name: String = read_input("Nom de la connexion: ")?;
    
    if connections.iter().any(|c: &Connection| c.name == name) {
        println!("{}", format!("Une connexion avec le nom '{}' existe déjà !", name).red());
        return Err(format!("Une connexion avec le nom '{}' existe déjà", name));
    }

    let host: String = read_input("Host: ")?;
    let port: u16 = read_input("Port: ")?;
    let database: String = read_input("Base de données: ")?;
    let username: String = read_input("Nom d'utilisateur: ")?;
    let password: String = read_input("Mot de passe: ")?;
    
    let connection = Connection {
        name,
        host,
        port,
        database,
        username,
        password,
    };
    
    connections.push(connection);
    
    save_connections(&connections);
    
    println!("{}", format!("✓ Connexion sauvegardée avec succès !").green());
    Ok(())
}

pub fn update_connection() -> Result<(), String> {
    let connections = load_connections();

    if connections.is_empty() {
        println!("{}", format!("Aucune connexion disponible").yellow());
        return Err("Aucune connexion disponible".to_string());
    }

    let names: Vec<String> = connections.iter().map(|c| c.name.clone()).collect();
    let selected_name = Select::new("Sélectionnez une connexion :", names)
        .prompt()
        .map_err(|e| e.to_string())?;

    let mut selected_connection = connections
        .iter()
        .find(|c| c.name == selected_name)
        .cloned()
        .ok_or("Connexion non trouvée")?;

    let fields = vec!["Nom", "Host", "Port", "Database", "Username", "Password"];
    let selected_field = Select::new("Quel champ voulez-vous modifer ?", fields)
        .prompt()
        .map_err(|e| e.to_string())?;

    match selected_field {
        "Nom" => selected_connection.name = read_input("Nouveau nom : ")?,
        "Host" => selected_connection.host = read_input("Nouveau host : ")?,
        "Port" => selected_connection.port = read_input("Nouveau port : ")?,
        "Database" => selected_connection.database = read_input("Nouvelle base de données : ")?,
        "Username" => selected_connection.username = read_input("Nouveau username : ")?,
        "Password" => selected_connection.password = read_input("Nouveau mot de passe : ")?,
        _ => return Err("Champ invalide".to_string()),
    }

    let updated_connections = connections
        .into_iter()
        .map(|c| if c.name == selected_name { selected_connection.clone() } else { c })
        .collect::<Vec<_>>();

    save_connections(&updated_connections);

    println!("{}", "✓ Connexion mise à jour avec succès !".green());
    Ok(())
}

pub fn delete_connection() -> Result<(), String> {
    let mut connections = load_connections();

    if connections.is_empty() {
        println!("{}", format!("Aucune connexion disponible").yellow());
        return Err("Aucune connexion disponible".to_string());
    }

    let names: Vec<String> = connections.iter().map(|c| c.name.clone()).collect();
    let selected_name = Select::new("Sélectionnez une connexion à supprimer :", names)
        .prompt()
        .map_err(|e| e.to_string())?;

    connections.retain(|c| c.name != selected_name);

    save_connections(&connections);

    println!("{}", "✓ Connexion supprimée avec succès !".green());
    Ok(())
}

pub fn select_connection() -> Result<(), String> {
    let connections = load_connections();

    if connections.is_empty() {
        println!("{}", "Aucune connexion disponible".yellow());
        return Err("Aucune connexion disponible".to_string());
    }

    let names: Vec<String> = connections.iter().map(|c| c.name.clone()).collect();
    let selected_name = Select::new("Sélectionnez une connexion :", names)
        .prompt()
        .map_err(|e| e.to_string())?;

    let connection = connections
        .iter()
        .find(|c| c.name == selected_name)
        .ok_or("Connexion non trouvée")?;

    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        connection.username, connection.password,
        connection.host, connection.port, connection.database
    );

    let pool = Pool::new(url.as_str())
        .map_err(|e| format!("Erreur de connexion : {}", e))?;

    let mut conn = pool.get_conn()
        .map_err(|e| format!("Erreur de connexion : {}", e))?;

    println!("{}", format!("✓ Connecté à '{}'", connection.name).green());

    loop {
        let actions = vec![
            "Lister les tables",
            "Exécuter une requête",
            "Voir les informations de la base",
            "Déconnexion"
        ];
        
        let action = Select::new("Que voulez-vous faire ?", actions)
            .prompt()
            .map_err(|e| e.to_string())?;
        
        match action {
            "Lister les tables" => {
                list_tables_mysql(&mut conn)?;
            }
            "Exécuter une requête" => {
                execute_query_mysql(&mut conn)?;
            }
            "Voir les informations de la base" => {
                show_database_info(connection)?;
            }
            "Déconnexion" => {
                println!("{}", "✓ Déconnecté".green());
                break;
            }
            _ => {}
        }
    }

    Ok(())
}

fn show_database_info(connection: &Connection) -> Result<(), String> {
    println!("=== Informations ===");
    println!("Nom : {}", connection.name);
    println!("Hôte : {}", connection.host);
    println!("Port : {}", connection.port);
    println!("Base : {}", connection.database);
    println!("User : {}", connection.username);
    
    Ok(())
}