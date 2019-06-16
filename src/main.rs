use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let (query, filename) = parse_args();

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let mut file = File::open(filename).expect(format!("Cant find file {}", filename).as_str());
    let mut content = String::new();

    file.read_to_string(&mut content).expect(format!("Cant read file {}", filename).as_str());

    println!("Content of {} is \n{}", filename, content);
}

fn parse_args() -> (String, String) {
    let args: Vec<String> = env::args().collect();
    return (args[1].clone(), args[2].clone());
}