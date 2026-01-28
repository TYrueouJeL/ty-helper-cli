// Déclare tous les modules disponibles
pub mod orm;
pub mod utils;
// Ajoutez vos futurs modules ici :
// pub mod autre_module;

use inquire::Select;

/// Liste de toutes les fonctionnalités disponibles
pub enum Module {
    Orm,
    // Ajoutez vos futurs modules ici
    Quitter,
}

impl Module {
    pub fn afficher_menu() -> Result<Module, inquire::InquireError> {
        let options = vec![
            "ORM - Gestionnaire de base de données",
            // Ajoutez vos futures options ici
            "Quitter",
        ];

        let choix = Select::new("Sélectionnez un programme :", options)
            .prompt()?;

        match choix {
            "ORM - Gestionnaire de base de données" => Ok(Module::Orm),
            "Quitter" => Ok(Module::Quitter),
            _ => Ok(Module::Quitter),
        }
    }

    pub fn executer(&self) {
        match self {
            Module::Orm => orm::run(),
            Module::Quitter => println!("Au revoir !"),
        }
    }
}
