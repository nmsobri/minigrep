//! Main code for parsing and performing the search
use std::env;
use std::fs::File;
use std::io::Read;
use std::env::Args;
use std::error::Error;

/// Struct to hold cofiguration value
pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: Args) -> Result<Config, &'static str> {
        args.next();

        let _query = match args.next() {
            Some(v) => v,
            None => return Err("Didnt get query to search for")
        };

        let _filename = match args.next() {
            Some(v) => v,
            None => return Err("Didnt get filename for searching")
        };

        let _case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        return Result::Ok(
            Config {
                query: _query,
                filename: _filename,
                case_sensitive: _case_sensitive,
            }
        );
    }
}

/// Main code to load the file and perform the search
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

/// Search in an sensitive way
pub fn search_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()
}

/// Search in an insensitive way
pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents.lines().filter(|line| line.contains(&query)).collect()
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