use std::collections::HashMap;
use std::fmt::Debug;
use std::path::Path;
use std::error::Error;
use std::fs::{self, read_to_string};


// ---------- Tree functions -----------

#[derive(Debug)]
pub enum Tree {
    Branch(Box<Tree>, u16, Box<Tree>),
    Leaf(char, u16),
}

impl Tree {
    pub fn extract_val(&self) -> u16 {
        match *self {
            Tree::Branch(_,val,_) => val,
            Tree::Leaf(_, val)    => val,
        }
    }
}


// Takes ownership of two trees and return a parent node of them, which holds ownership of the two children
pub fn create_node_of_2_mins (left: Tree, right: Tree) -> Tree {
    let sum_of_freq = left.extract_val() + right.extract_val();
    Tree::Branch(Box::new(left), sum_of_freq, Box::new(right))
}


// Builds the huffman tree by popping the two least frequent characters from the vector 'freq_vec' and adds them to the main tree by calling create_node_of_2_mins
// Then adds the resulting node to the vector. Returns the top root which owns all the tree data
pub fn create_tree (freq_vec: Vec<(char, u16)>) -> Tree {
    let mut nodes : Vec<Tree> = freq_vec.into_iter().map(|(c, f)| Tree::Leaf(c, f)).collect(); // Creates a vector of Tree

    while nodes.len() > 1 {
        nodes.sort_by_key(|tree| std::cmp::Reverse(tree.extract_val()));
        let left = nodes.pop().unwrap(); 
        let right = nodes.pop().unwrap();    // can't crash because len >= 2

        
        let newnode = create_node_of_2_mins(left, right);
        nodes.push(newnode);
    }
    nodes.pop().expect("Error in building the tree")
}


pub fn parser(content: &String) -> Result<HashMap<char, u16>, std::io::Error> {
    let mut frequency_map = HashMap::new();
   


    for c in content.chars() {
        *frequency_map.entry(c).or_insert(0) += 1;
    }
    
    
    //print_map(&frequency_map);
    Ok(frequency_map)
}



// Takes the root node of a tree and builds a hashmap giving the binary path in the tree to reach each character
pub fn build_code_map(tree: &Tree) -> HashMap<char, Vec<bool>> {
    let mut code_map = HashMap::new();
    build_code_map_rec(tree, Vec::new(), &mut code_map);
    code_map
}


// Logical function for 'build_code_map' that recursively runs the tree, writing the path to each character in a map
// (going left = 0 = false, right = 1 = true).
fn build_code_map_rec(tree: &Tree, path: Vec<bool>, map: &mut HashMap<char, Vec<bool>>) {
    match tree {
        Tree::Leaf(ch, _) => {
            map.insert(*ch, path);
        }
        Tree::Branch(left, _, right) => {
            let mut left_path = path.clone();
            left_path.push(false); // left = 0
            build_code_map_rec(left, left_path, map);

            let mut right_path = path;
            right_path.push(true); // right = 1
            build_code_map_rec(right, right_path, map);
        }
    }
}


// Takes a map containing each character as key and their binary path in the huffman tree as value, and builds
// a canonical vector containing TODO 
pub fn build_canonical_code(code_map: HashMap<char, Vec<bool>>) -> HashMap<char, String>  {
    let mut canonical_vec: Vec<(char, usize)> = Vec::new();
    for (c, vec) in code_map{
        canonical_vec.push((c, vec.len()));
    }

    // Compares by frequency (second elt of tupe), and if equal, compare by char to order alphabetically
    canonical_vec.sort_by(|a,b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));

    let mut canonical_map = HashMap::new();
    let mut code: u32 = 0;
    let mut prev_len = 0;

    for &(ch, bit_len) in &canonical_vec {
        // Shift left if bit_len increased (append zeros)
        if bit_len > prev_len {
            code <<= bit_len - prev_len;
        } else if bit_len < prev_len {
            panic!("Canonical vector not sorted correctly by bit length!");
        }

        // Format code with leading zeroes
        let code_str = format!("{:0bit_len$b}", code, bit_len = bit_len);
        canonical_map.insert(ch, code_str);

        // Increment code for the next symbol
        code += 1;
        prev_len = bit_len;
    }

    canonical_map
}




// --------------- Encoding ----------------
pub struct EncodedFile {
    pub encoded_name: String,
    pub encoded_content: String,
}

impl EncodedFile {
    pub fn build(file: &String, map: &HashMap<char, String>) -> Result<EncodedFile, Box<dyn Error>> {
       // Keeps only the name of the file without the leading path
        let name: &str;
        match file.rsplit_once('/')  {
            None => name = file,
            Some(n) => name = n.1,
        }

        let mut encoded_name = String::new();
        for char in name.chars() {
            encoded_name.push_str(map.get(&char).ok_or("Encode_file: the encoded character isn't in the map.")?);
        }

        let mut encoded_content = String::new();
        let content = read_to_string(Path::new(&file))?;
        for char in content.chars() {
            encoded_content.push_str(map.get(&char).ok_or("Encode_file: the encoded character isn't in the map.")?);
        }

        
        Ok(EncodedFile { encoded_name, encoded_content })
    }
}

// Function that manages the build of the canonical map by callinx auxiliary functions
pub fn build_canonical_map_from_string(content: &String) -> Result<HashMap<char,String>, Box<dyn Error>> {
    let freq_vec: Vec<(char, u16)> = parser(&content)?
            .iter()
            .map(|(&c, &f)| (c, f))
            .collect();

    

    if freq_vec.is_empty() {
        return Err("encode_file: Cannot compress enmpy files.".into());
    }

    // Creates the huffman tree, then the code map and then the canonical code map
    let tree = create_tree(freq_vec);
    //print_tree(&tree, 0);

    let code_map = build_code_map(&tree);
    //print_code_map(&code_map);

    let canonical_code_map= build_canonical_code(code_map);
    //println!("{:?}", canonical_code_map);

    Ok(canonical_code_map)
}


pub fn merge_string(file_paths: &Vec<String>) -> Result<String, Box<dyn Error>> {
    let mut content = String::new();
    for file_path in file_paths {      
        // Push name first to make sure the characters in the file name also have a compression symbol
        content.push_str(file_path);                          
        content.push_str(&fs::read_to_string(Path::new(file_path))?);                                   // Jump line to separate files
    }
    
    Ok(content)
}


pub fn add_huffman_header(map: &HashMap<char, String>, files_count: usize) -> Result<String, Box<dyn Error>> {
    let mut header = String::new();

    // In order: size of map, number of files
    header.push_str(&format!("{}\n{}\n", map.len(), files_count));
    for (char, code) in map {
        header.push_str(&format!("{}:{}\n", char, code));
    }
    

    Ok(header)
} 



// This function reads the file provided and applies all the functions needed to compress it
// TODO : add impl of separation of files in the archive
pub fn encode_file_huffman(config: &super::Config) -> Result<(), Box<dyn Error>> {
    
    let file_paths = &config.files;

    let merged_content = merge_string(file_paths)?;
    let merged_canonical_map = build_canonical_map_from_string(&merged_content)?;
    
    let mut final_encoded_file: String = add_huffman_header(&merged_canonical_map, file_paths.len())?;
    

    for file in file_paths {
        let encoded_file = EncodedFile::build(file, &merged_canonical_map)?;
        final_encoded_file.push_str(&format!("{}&{}\n", &encoded_file.encoded_name, &encoded_file.encoded_content));
        println!("Encoded {} in {}.", file, &config.archive_name);
   }
    
    

    fs::write(Path::new(&config.archive_name), final_encoded_file)?;
    Ok(())
}


// --------- Decoder functions -----------

pub fn read_canonical_map(encoded_text: &String, map_size: usize) -> Result<(HashMap<String,char>, bool), Box<dyn Error>> {
    let mut canonical_map = HashMap::new();
    let mut lines = encoded_text.lines();
    let mut contains_backspace = false;
    lines.next();
    lines.next();

    for _i in 0..map_size {
        let line = lines.next().ok_or("Missing line in character map")?;

        // If the character is ':' or '\n', special case to avoid crash
        if line.starts_with("::") {
            let mut tmp: String = line.into();
            tmp.drain(..2);
            canonical_map.insert(tmp, ':');
            continue;
        } else if line.is_empty(){
            let tmpline = lines.next().ok_or("Missing line in character map")?;
            let mut tmp: String = tmpline.into();
            tmp.drain(..1);
            canonical_map.insert(tmp, '\n');
            contains_backspace = true;
            continue;
        }

        let mut parts = line.splitn(2, ':');

        let ch_str = parts.next().ok_or("Missing character in map entry")?;
        let code = parts.next().ok_or("Missing code in map entry")?;

        let ch = ch_str.chars().next().ok_or("Empty character in map")?;

        canonical_map.insert(code.to_string(), ch);        
    }
    
    Ok((canonical_map, contains_backspace))
}



// TODO : implement struct so that those functions can be methods
pub fn decode_file_huffman(config: &super::Config) -> Result<(), Box<dyn Error>> {
    let file_paths = &config.files;

    // Decompress one by one each archive
    for file in file_paths {

        let mut encoded_text = String::new();
        encoded_text.push_str(&read_to_string(file)?);
        

        let mut lines = encoded_text.lines();

        // Read and parse map size
        let char_map_size: usize = lines
            .next()
            .ok_or("Missing character map size line")?
            .trim()
            .parse()?;


        let _file_count: usize = lines
            .next()
            .ok_or("Missing files count line")?
            .trim()
            .parse()?;

        
        let (canonical_map, contains_backspace) = read_canonical_map(&encoded_text, char_map_size)?; //read_canonical_map(&config.files);

        // Skips the canonical map lines
        for _ in 0..char_map_size {
            lines.next();
        }
        if contains_backspace {
            lines.next();
        }

        

        
        

        for line in lines {
            let mut buffer = String::new();
            let mut file_name = String::new();
            let mut file_content = String::new();
            
            let mut reached_separator: bool = false;
            for bit in line.chars() {
                //println!("Inner For iteration");
                if bit == '&' {reached_separator = true; continue;}
                buffer.push(bit);

                if let Some(&ch) = canonical_map.get(&buffer) {
                    if !reached_separator {
                        file_name.push(ch);
                        buffer.clear();
                    } else {
                        file_content.push(ch);
                        buffer.clear(); // reset for next code
                    }
                }
            }
            
            fs::write(Path::new(&file_name), file_content)?;
            println!("Decompressed {}", &file_name);
        }
        
        
    }
    Ok(())
}





// ------- Debug functions ---------



pub fn print_tree(tree: &Tree, indent: usize) {
    let pad = "  ".repeat(indent);

    match tree {
        Tree::Leaf(ch, freq) => {
            println!("{}Leaf('{}': {})", pad, ch, freq);
        }
        Tree::Branch(left, freq, right) => {
            println!("{}Branch({})", pad, freq);
            print_tree(left, indent + 1);
            print_tree(right, indent + 1);
        }
    }
}



pub fn print_code_map(code_map: &HashMap<char, Vec<bool>>) {
    for (ch, code) in code_map {
        let bits: String = code.iter().map(|b| if *b { '1' } else { '0' }).collect();
        println!("'{}': {}", ch, bits);
    }       
}


pub fn print_map<K: Debug, V: Debug>(map: &HashMap<K, V>) {
    for (key, value) in map {
        println!("{:?}: {:?}", key, value);
    }
}
