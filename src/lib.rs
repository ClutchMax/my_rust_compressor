use std::error::Error;
use std::path::Path;


use crate::huffman::{decode_file_huffman, encode_file_huffman};


pub mod huffman; 

pub enum Action {
    Compress,
    Decompress,
}

pub enum EncodingMethod {
    Huffman,
}

pub struct Config {
    pub archive_name: String,
    pub files: Vec<String>,
    pub action: Action,
    pub encoding: EncodingMethod,
   
}


impl Config {
    pub fn build(mut args: Vec<String>) -> Result<Config, Box<dyn Error>> {
        if args.len() <= 2 {
            return Err("Not enough arguments.".into());
        }
    
        args.remove(0); // Remove first element, program name
        let mut action: Action = Action::Compress; // Setting useless defaut value else compiler isn't happy
        let mut archive_name = "Archive.zip";
        let mut found_action = false;
        let mut found_archive_name = false;

        for param in &args {
            if param.starts_with('-'){
                match param {
                    _ if param.contains("-d") => action = Action::Decompress,
                    _ if param.contains("-c") => action = Action::Compress,
                    _ => return Err("Wrong parameters were provided.".into())
                }
                found_action = true;
            } else if param.contains(".zip") {
                // TODO : continuer d'implémenter le parsing des arguments
            }
        }

        if !found_action {
            return Err("User didn't provide an action for the program.\n 
                        -c to compress, -d to decompress.".into());
        }

        
        let archive_name = args.pop().unwrap(); // Can use unwrap because Vector can't have less than 3 arguments
        if !archive_name.ends_with(".zip") {            // TODO : add other extensions allowed
            return Err("Archive name doesn't have a correct extension, or wasn't provided as last argument.".into());
        }

        // Checks if provided fils in arguments exist
        for file in args.clone() {
            if !Path::new(&file).exists() {
                return Err(format!("File {} doesn't exist.", file).into());
            }            
        }
        
        Ok(Config {
            archive_name,
            files: args,
            action,
            encoding: EncodingMethod::Huffman,
        })
    }
}





pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
    match config.action {
        Action::Compress => match config.encoding {
            EncodingMethod::Huffman => encode_file_huffman(&config)?,
        },

        Action::Decompress => match config.encoding {
            EncodingMethod::Huffman => decode_file_huffman(&config)?,
        },
    }
    

    Ok(())
}





