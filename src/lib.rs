use std::error::Error;
use std::fs;
mod tokenizer;

pub struct Config {
    pub file_path: String,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let parsed_string = tokenizer::tokenizer(&contents);
    println!("parsed_string {}", parsed_string);

    Ok(())
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let file_path = args[1].clone();

        Ok(Config { file_path })
    }
}
