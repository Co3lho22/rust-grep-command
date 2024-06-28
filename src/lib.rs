use std::{
    error::Error,
    fs,
};

pub struct Config {
    pub query: String,
    pub file_name: String,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &str>{
        if args.len() < 3 {
            return Err("Error: not enough argument!!")
        }

        Ok(Config {
            query: args[1].clone(),
            file_name: args[2].clone(),
        })
    }
}

pub fn run(grep_config: Config) -> Result<(), Box<dyn Error>>{
    let contents: String = fs::read_to_string(grep_config.file_name)?;
    println!("{}", contents);

    Ok(())
}

