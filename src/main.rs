use std::{
    env,
    process
};

use minigrep::{
    self,
    Config
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let grep_config: Config = minigrep::Config::new(&args).unwrap_or_else(|err|{
        println!("{}", err);
        process::exit(1);
    });

    if let Err(err) = minigrep::run(grep_config){
        println!("{}", err);
        process::exit(1);
    }
}

