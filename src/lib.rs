use std::env;
use std::fs::File;
use std::io::Read;
use std::error::Error;

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Result::Err("Insufficent parameters");
        }

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        return Result::Ok(
            Config {
                query: args[1].clone(),
                filename: args[2].clone(),
                case_sensitive: case_sensitive,
            }
        );
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut file = File::open(config.filename.clone())?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;

    let results = if config.case_sensitive {
        search_sensitive(&config.query, &content)
    } else {
        search_insensitive(&config.query, &content)
    };

    for result in results {
        println!("{}", result);
    }

    Ok(())
}

pub fn search_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}


#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive\nPick three\nDuct Tape";
        assert_eq!(vec!["safe, fast, productive"], search_sensitive(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "Rust:\nsafe, fast, productive\nPick three.\nTrust me";
        assert_eq!(vec!["Rust:", "Trust me"], search_insensitive(query, contents));
    }
}