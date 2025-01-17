//ci serve la funzione std::env::args della standard library di Rust che ci da un iteratore di valori.
//ci serve la funzion std::fs::read_to_string dalla standard library per leggere a file
use std::env;
use std::process;

use minigrep::Config

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    
    
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
