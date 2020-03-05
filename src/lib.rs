use std::fs;
use std::error::Error;


pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {

    let contents = fs::read_to_string(&config.filename)?;

    println!("file content:\n{}", contents);

    Ok(())
}


pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &str> {
        
        if args.len() < 3 {
            return Err("Not enough args");
        }

        Ok(Config {query: args[1].clone(), filename: args[2].clone()})
    }
}