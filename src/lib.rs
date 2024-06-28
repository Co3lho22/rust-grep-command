use std::{
    error::Error,
    fs,
    env,
};

pub struct Config {
    pub query: String,
    pub file_name: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &str>{
        if args.len() < 3 {
            return Err("Error: not enough argument!!")
        }

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query: args[1].clone(),
            file_name: args[2].clone(),
            case_sensitive,
        })
    }
}

pub fn run(grep_config: Config) -> Result<(), Box<dyn Error>>{
    let contents: String = fs::read_to_string(&grep_config.file_name)?;

    let lines = if grep_config.case_sensitive {
        search(&grep_config.query, &contents)
    } else {
       search_case_insensitive(&grep_config.query, &contents)
    };

    println!("{:#?}", lines);

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
            .filter(|line| line.contains(query))
            .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    // let mut lines_with_query = Vec::new();
    let query = query.to_lowercase();

     contents.lines()
             .filter(|line| line.to_lowercase()
                                .contains(&query)
             )
             .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "safe";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents),
        );
    }

    #[test]
    fn case_sensitive() {
       let query = "three";
       let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct Three.";

       assert_eq!(
           vec!["Pick three."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec![
            "Rust:",
            "Trust me.",
            ],
            search_case_insensitive(query, contents),
            )

    }
}

