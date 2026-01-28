use mysql::{PooledConn, prelude::Queryable};
use colored::*;

use crate::modules::utils::read_input;

pub fn list_tables_mysql(conn: &mut PooledConn) -> Result<(), String> {
    let tables: Vec<String> = conn
        .query("SHOW TABLES")
        .map_err(|e| format!("Erreur : {}", e))?;

    println!("=== Tables disponibles ===");
    for table in tables {
        println!("  • {}", table);
    }
    println!();

    Ok(())
}

pub fn execute_query_mysql(conn: &mut PooledConn) -> Result<(), String> {
    let query: String = read_input("Entrez votre requête SQL : ")?;

    // Différencier les SELECT des autres requêtes
    if query.trim().to_uppercase().starts_with("SELECT") {
        // Pour les requêtes SELECT, afficher les résultats
        let result: Vec<mysql::Row> = conn.query(&query)
            .map_err(|e| format!("Erreur SQL : {}", e))?;

        println!("\n=== Résultats ===");
        if result.is_empty() {
            println!("{}", "Aucun résultat".yellow());
        } else {
            for row in result {
                println!("{:?}", row);
            }
        }
        println!();
    } else {
        // Pour les INSERT, UPDATE, DELETE, CREATE, etc.
        conn.query_drop(&query)
            .map_err(|e| format!("Erreur SQL : {}", e))?;

        println!("{}", "✓ Requête exécutée avec succès !".green());
    }

    Ok(())
}