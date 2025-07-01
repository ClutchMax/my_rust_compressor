use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::fmt::Debug;



pub fn parser(content: &Vec<String>) -> Result<HashMap<char, u8>, std::io::Error> {
    let mut frequency_map = HashMap::new();
    for file in content {
        let file_content = fs::read_to_string(Path::new(file))?; 

        for c in file_content.chars() {
            *frequency_map.entry(c).or_insert(0) += 1;
        }
    }
    
   Ok(frequency_map)
}



pub fn print_map<K: Debug, V: Debug>(map: &HashMap<K, V>) {
    for (key, value) in map {
        println!("{:?}: {:?}", key, value);
    }
}