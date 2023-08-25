use std::{collections::HashSet, fmt::{Display, Write}};
use rand::{self, Rng};


type Position = (usize, usize);

//new type to give when a player clicks on the field.
enum OpenResult {
    Mine,
    NoMine(u8),
}

#[derive(Debug)]
pub struct Minesweeper{
    width: usize,
    height: usize,
    mine_pos: HashSet<Position>,
    opened_fields: HashSet<Position>,
    flaged: HashSet<Position>,
    
}

impl Display for Minesweeper{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in 1..=self.width{
            for y in 1..=self.height  {
                let pos = (x, y);

                //if it's not opened
                if !self.opened_fields.contains(&pos){

                    //check for flagged
                    if self.flaged.contains(&pos){ 
                        f.write_str("🚩 ")?;

                    }else {
                        f.write_str("🟪 ")?;
                    }

                    
                }else {
                    //if it's opened
                    if self.mine_pos.contains(&pos){
                        //contains a mine
                        f.write_str("💣 ")?;
                    }else {
                        //doesn't contains a mine
                        write!(f, " {} ", self.neighboring_mines(pos))?;
                    }
                }
            }
            f.write_str("\n")?;
        }

        Ok(())
    }
}

impl Minesweeper {
    
    pub fn new(width: usize, height: usize, mine_count: usize) -> Minesweeper{
        let mut mines = HashSet::new();
        let mut rng = rand::thread_rng();

        while mines.len() < mine_count {
            mines.insert((rng.gen_range(1..=width), rng.gen_range(1..=height)));
        }

        Minesweeper { 
            width: width, 
            height: height, 
            mine_pos: mines, 
            opened_fields:HashSet::new(),
            flaged: HashSet::new() 
        }
    }

    fn neighboring_mines(&self, position: Position) -> u8 {
        let mut mines_arround = 0;

        let (a, b) = (position.0, position.1);

        for x  in -1..2{
            for y in -1..2{
                if self.mine_pos.contains(&((a as isize + x) as usize, (b as isize + y) as usize)){
                    mines_arround+=1
                }
            }
        }
        mines_arround

    }

    fn open_field(&mut self, position: Position) -> Option<OpenResult> {

        //don't open flagged fields
        if self.flaged.contains(&position){
            return None;
        }

        self.opened_fields.insert(position);

        let is_mine = self.mine_pos.contains(&position);
        
        //opened position containes a mine
        if is_mine{
            Some(OpenResult::Mine)

        //opened position doesn't containes a mine
        }else {
            Some(OpenResult::NoMine(self.neighboring_mines(position)))
        }
    }

    fn toggle_flag(&mut self, position: Position) {
        if self.opened_fields.contains(&position){
            return;
        }

        if self.flaged.contains(&position){
            self.flaged.remove(&position);
        }else {
            self.flaged.insert(position);
        }


        
    }
}

#[cfg(test)]
mod test{
    use crate::minesweeper::Minesweeper;

    #[test]
    fn test() {
        let mut ms = Minesweeper::new(10, 10, 10);
        ms.open_field((10,10));
        ms.toggle_flag((4,8));
        ms.open_field((4,8));
        ms.toggle_flag((4,8));

        println!("{}", ms)
    }


}

