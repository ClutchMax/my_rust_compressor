use std::collections::HashMap;
use std::fmt::Debug;

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
    dbg!(&nodes);

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
    
    
    print_map(&frequency_map);
    Ok(frequency_map)
}




pub fn print_map<K: Debug, V: Debug>(map: &HashMap<K, V>) {
    for (key, value) in map {
        println!("{:?}: {:?}", key, value);
    }
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


