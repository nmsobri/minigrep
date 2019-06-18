use std::fs::File;
use std::io::Read;
use std::error::Error;

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Result::Err("Insufficent parameters");
        }

        return Result::Ok(
            Config {
                query: args[1].clone(),
                filename: args[2].clone(),
            }
        );
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut file = File::open(config.filename.clone())?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    println!("Content of {} is \n{}", config.filename, content);
    Ok(())
}
