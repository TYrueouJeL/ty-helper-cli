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

    let result = conn.query_iter(&query)
        .map_err(|e| format!("Erreur SQL : {}", e))?;

    let count = result.count();
    println!("{}", format!("✓ {} ligne(s) affectée(s)", count).green());

    Ok(())
}