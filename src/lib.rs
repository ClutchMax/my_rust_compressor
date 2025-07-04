use std::error::Error;
use std::path::Path;

pub mod huffman_coding;
pub mod huffman_tree;


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

    // Creates a frequency map with the parser function, then turns it into a vector of tuple for the algorithm
    let freq_vec: Vec<(char, u16)> = huffman_coding::parser(&config.files)?
    .iter()
    .map(|(&c, &f)| (c, f))
    .collect();

    if freq_vec.is_empty() {
        return Err("Trying to compress empty files.".into());
    }

    // Creates the huffman tree, then the code map and then the canonical code map
    let tree = huffman_tree::create_tree(freq_vec);
    huffman_tree::print_tree(&tree, 0);

    let code_map = huffman_tree::build_code_map(&tree);
    huffman_tree::print_code_map(&code_map);

    let canonical_code_map = huffman_tree::build_canonical_code(code_map);
    println!("{:?}", canonical_code_map);
    



    Ok(())
}





