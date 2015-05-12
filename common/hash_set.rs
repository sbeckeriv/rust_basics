use std::collections::HashSet;

impl <T> HashSet<T> {
    pub fn new(init: Vec<T>) -> HashSet<T>{
        let mut new_hash = HashSet::new();
        for value in init {
            new_hash.insert(value);
        }
    };
}

fn main(){
    let t:HashSet<i32> = HashSet::new(vec!("a","b"));
    println!("{}", t);
}
