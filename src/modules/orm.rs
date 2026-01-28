/// Module ORM - Gestionnaire de base de données
use inquire::{Select};

use crate::modules::orm::connection::{create_connection, delete_connection, list_connections, select_connection, update_connection};

mod connection;
mod mysql;
mod storage;

pub fn run() {
    println!("\n=== Module ORM ===\n");

    loop {
        match afficher_menu_orm() {
            Ok(true) => continue,
            Ok(false) => break,
            Err(e) => {
                eprintln!("Erreur : {}", e);
                break;
            }
        }
    }
}

fn afficher_menu_orm() -> Result<bool, String> {
    let options = vec![
        "Sélectionner une connection",
        "Lister les connections",
        "Créer une connection",
        "Modifier une connection",
        "Supprimer une connection",
        "Retour au menu principal",
    ];

    let choix = Select::new("Que voulez-vous faire ?", options)
        .prompt()
        .map_err(|e| e.to_string())?;

    match choix {
        "Sélectionner une connection" => {
            select_connection()?;
            Ok(true)
        }
        "Lister les connections" => {
            list_connections()?;
            Ok(true)
        }
        "Créer une connection" => {
            create_connection()?;
            Ok(true)
        }
        "Modifier une connection" => {
            update_connection()?;
            Ok(true)
        }
        "Supprimer une connection" => {
            delete_connection()?;
            Ok(true)
        }
        "Retour au menu principal" => Ok(false),
        _ => Ok(false),
    }
}
