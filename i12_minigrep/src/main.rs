//ci serve la funzione std::env::args della standard library di Rust che ci da un iteratore di valori.
//ci serve la funzion std::fs::read_to_string dalla standard library per leggere a file
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config($args);
    
    
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    //read_to_string apre un file e ottiene il contenuto
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    
    println!("With text:\n{contents}");
}

fn Config {
    query: String,
    file_path : String,
}

fn parse_config(args: &[String]) -> Config {
    let query = &args[1].clone();
    let file_path = &args[2].clone();

    Config {query, file_path}
}