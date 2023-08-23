use std::collections::HashSet;


type Position = (usize, usize);

struct Minsesweeper{
    width: usize,
    height: usize,
    mine_pos: HashSet<Position>,
    flaged: HashSet<Position>,
    
}


impl Minsesweeper {
    
    fn new(width: usize, height: usize) -> Minsesweeper{
        Minsesweeper { 
            width: width, 
            height: height, 
            mine_pos: (), 
            flaged: () 
        }

    }
}


fn main() {
    println!("Hello, world!");
}