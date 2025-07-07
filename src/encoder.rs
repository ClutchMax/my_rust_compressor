use std::fs;
use std::path::Path;
use std::error::Error;
use std::collections::HashMap;

pub use crate::huffman_tree;


pub fn merge_string(file_paths: &Vec<String>) -> Result<String, Box<dyn Error>> {
    let mut content = String::new();
    for file_path in file_paths {
        content.push_str(&fs::read_to_string(Path::new(file_path))?); 
    }
    Ok(content)
}

pub fn add_header_to_encoded_file(map: &HashMap<char, String>, input_length: usize) -> Result<String, Box<dyn Error>> {
    let mut header = String::new();
    header.push_str(&format!("{}\n{}\n",map.len(), input_length));
    for (char, code) in map {
        header.push_str(&format!("{}:{}\n", char, code));
    }

    Ok(header)
} 

// This function reads the file provided and applies all the functions needed to compress it
pub fn encode_file(file_paths: &Vec<String>) -> Result<String, Box<dyn Error>> {
    

    let content = merge_string(file_paths)?.to_ascii_lowercase();   // For now, go to lowercase to gain more space TODO
    let freq_vec: Vec<(char, u16)> = huffman_tree::parser(&content)?
            .iter()
            .map(|(&c, &f)| (c, f))
            .collect();

    

    if freq_vec.is_empty() {
        return Err("encode_file: Cannot compress enmpy files.".into());
    }

    // Creates the huffman tree, then the code map and then the canonical code map
    let tree = huffman_tree::create_tree(freq_vec);
    huffman_tree::print_tree(&tree, 0);

    let code_map = huffman_tree::build_code_map(&tree);
    huffman_tree::print_code_map(&code_map);

    let canonical_code_map= huffman_tree::build_canonical_code(code_map);
    println!("{:?}", canonical_code_map);

    // Add header
    let mut encoded_file = add_header_to_encoded_file(&canonical_code_map, content.len())?; // TODO : use more optimized structure than string

    for char in content.chars() {
        encoded_file.push_str(canonical_code_map.get(&char).ok_or("Encode_file: the encoded character isn't in the map.")?);
    }


    Ok(encoded_file)
}