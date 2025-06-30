use std::error::Error;
//use std::fs;
use std::path::Path;

pub struct Config {
    pub archive_name: String,
    pub files: Vec<String>,
   
}


impl Config {
    pub fn build(mut args: Vec<String>) -> Result<Config, String> {
        if args.len() <= 2 {
            return Err("Not enough arguments.".into());
        }
    
        args.remove(0); // Remove first element, program name

        

        // TODO Chech if final name contains a compressed file extension
        let archive_name = args.pop().unwrap(); // Can use unwrap because Vector can't have less than 3 arguments


        // Checks if provided fils in arguments exist
        for file in args.clone() {
            if !Path::new(&file).exists() {
                return Err(format!("File {} doesn't exist.", file));
            }
        }

        Ok(Config {
            archive_name,
            files: args,
        })
    }
}

pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
    Ok(())
}
