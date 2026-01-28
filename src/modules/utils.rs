use std::io::{self, Write};
use std::str::FromStr;
use std::fmt::Display;

pub fn read_input<T>(prompt: &str) -> Result<T, String> 
where
    T: FromStr,
    T::Err: Display,
{
    loop {
        print!("{}", prompt);
        io::stdout().flush().map_err(|e| e.to_string())?;
        
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| e.to_string())?;
        
        match input.trim().parse::<T>() {
            Ok(value) => return Ok(value),
            Err(e) => println!("❌ Erreur: {}. Veuillez réessayer.", e),
        }
    }
}