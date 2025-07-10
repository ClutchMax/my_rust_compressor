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
        let mut archive_name = "archive.zip";
        let mut found_action = false;
        let mut found_archive_name = false;
        let mut files = Vec::new();
        let encoding = EncodingMethod::Huffman;

        for param in &args {
            if param.starts_with('-'){
                match param {
                    // TODO : manage if both arguments are given (rn -d wins)
                    _ if param.contains("-d") => action = Action::Decompress,
                    _ if param.contains("-c") => action = Action::Compress,
                    _ => return Err("Wrong parameters were provided.".into())
                }
                found_action = true;
            } else if param.contains(".zip") && !found_archive_name {
                archive_name = param;
                found_archive_name = true;
            } else if param.contains(".zip") && found_archive_name {
                return Err("Two archive names were given.".into());
            } else {
                files.push(param.clone());
            }
        }

        if !found_action {
            return Err("User didn't provide an action for the program.\n 
                        -c to compress, -d to decompress.".into());
        }

        // If no archive name is provided, if trying to decompress, throws error,
        // If trying to compress, gives the default "Archive.zip" name. 
        if !found_archive_name {
            return Err("Error: user need to provide an archive name.".into());          
        }
        
        

        // Checks if provided fils in arguments exist
        for file in &files {
            if !Path::new(&file).exists() {
                return Err(format!("File {} doesn't exist.", file).into());
            }            
        }
        
        Ok(Config {
            archive_name: archive_name.into(),
            files,
            action,
            encoding,
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





