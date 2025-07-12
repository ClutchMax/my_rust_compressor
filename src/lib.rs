use std::error::Error;
use std::path::Path;


use crate::huffman::{encode_bitvec_huffman, decode_bitvec_huffman};


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
    /// Constructor for the "config" struct.
    /// Parses the parameters given inline. 
    /// If the action is "compress", the config struct will have an archive name and vector of files to compress and 
    /// If ... "decompress", the config will look for archive names, the "archive name" parameter won't be used. 
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

        
        // Parses the arguments to find the action to proceed
        for param in &args {
            if param == "-d" && found_action == false {
                action = Action::Decompress;
                found_action = true;
                break;
            } else if param == "-c" && found_action == false {
                action = Action::Compress;
                found_action = true;
                break;
            }
        }


        if !found_action {
            return Err("User didn't provide an action for the program.\n 
                        -c to compress, -d to decompress.".into());
        }

        match action {
            Action::Compress => {
                for param in &args {
                    if param.contains(".zip") && !found_archive_name {
                        archive_name = param;
                        found_archive_name = true;
                    } else if param.contains(".zip") && found_archive_name { 
                        return Err("Two archive names were given. Cannot compress an archive (yet).".into());
                    } else if param == "-d" || param == "-c"{continue;}
                    else {
                        files.push(param.clone());
                    }
                }
            },
            Action::Decompress => {
                for param in &args {
                    if param.contains(".zip") {files.push(param.clone());}
                }
            }
        }

        // If no archive name is provided, if trying to decompress, throws error,
        // If trying to compress, gives the default "Archive.zip" name. 
        // if !found_archive_name {
        //     return Err("Error: user need to provide an archive name.".into());          
        // }
        
        if files.len() <= 0 {
            return Err("User must provide files to compress or decompress.".into());
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




/// Main function that runs the logic of the program, according to the `Config` parameter.
pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
    match config.action {
        Action::Compress => match config.encoding {
            EncodingMethod::Huffman => encode_bitvec_huffman(&config)?,
        },

        Action::Decompress => match config.encoding {
            EncodingMethod::Huffman => decode_bitvec_huffman(&config)?,
        },
    }
    

    Ok(())
}





