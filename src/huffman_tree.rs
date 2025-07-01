#[derive(Debug)]
pub enum Tree<T> {
    Branch(Box<Tree<T>>, T, Box<Tree<T>>),
    Leaf(T, char),
}

impl<T: Copy> Tree<T> {
    pub fn extract_val(&self) -> T {
        match *self {
            Tree::Branch(_,val,_) => val,
            Tree::Leaf(val, _)       => val,
        }
    }
}

pub fn create_node_of_2_mins<T: Copy + std::ops::Add<Output = T>>(freq_map: HashMap<char, T>) -> Tree<T> {
    let left = 
    let sum_of_freq = left.extract_val() + right.extract_val();
    Tree::Branch(Box::new(left), sum_of_freq, Box::new(right))
}