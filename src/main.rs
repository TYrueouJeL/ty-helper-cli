mod modules;

use modules::Module;

fn main() {
    println!("╔════════════════════════════════════╗");
    println!("║     TY Helper CLI - v0.1.0         ║");
    println!("╚════════════════════════════════════╝\n");

    loop {
        match Module::afficher_menu() {
            Ok(module) => {
                match module {
                    Module::Quitter => {
                        println!("\n👋 Au revoir !");
                        break;
                    }
                    _ => module.executer(),
                }
            }
            Err(e) => {
                eprintln!("Erreur lors de la sélection : {}", e);
                break;
            }
        }
    }
}