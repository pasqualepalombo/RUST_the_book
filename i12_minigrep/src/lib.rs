use std::fs;
use std::error::Error;


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //read_to_string apre un file e ottiene il contenuto
    let contents = fs::read_to_string(config.file_path)?;
    
    println!("With text:\n{contents}");

    ok(())
}

pub struct Config {
    query: String,
    file_path : String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config {query, file_path})
    }
}