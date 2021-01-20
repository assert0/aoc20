use std::fs;
//use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MemoryGame {
    turns: Vec::<usize>,
}

impl MemoryGame {

    pub fn new(starting: &Vec<usize>) -> MemoryGame {
        let mut turns = Vec::<usize>::new();
        starting.iter().for_each(|&s| turns.push(s));
        MemoryGame { turns }
    }

}

impl Iterator for MemoryGame {
    type Item = usize;
    
    fn next(&mut self) -> Option<usize> {
        let number = self.turns.last().unwrap();
        //println!("{:?}", self.turns);
        let mut iter = self.turns.iter()
                    .enumerate()
                    .rev()
                    .filter(|(_i, &v)| v == *number)
                    .take(2);
        let (last, prev) = (iter.next(), iter.next());
        //println!("{:?} {:?}", last, prev);
        let mut turn = 0;
        if last.is_some() && prev.is_some() {
            turn = last.unwrap().0 - prev.unwrap().0;
        }
        self.turns.push(turn);
        if self.turns.len() % 100000 == 0 {
            println!("{}", self.turns.len());
        }
        //println!("{:?}", turn);
        Some(turn)
    }
}

pub fn day15(args: &[String]) -> i32 {
    println!("Day 15");
    if args.len() != 1 {
        println!("Missing input file");
        return -1;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let numbers = contents.lines()
                          .next()
                          .unwrap()
                          .split(",")
                          .map(|i| i.parse::<usize>().unwrap())
                          .collect::<Vec<usize>>();
    //println!("{:?}", numbers);

    let g = MemoryGame::new(&numbers);
    println!("Part 1: {}", g.skip(2020 - numbers.len() - 1).next().unwrap());
    
    let g = MemoryGame::new(&numbers);
    println!("Part 2: {}", g.skip(30_000_000 - numbers.len() - 1).next().unwrap());
    
    0
}



// pub fn run(numbers: Vec<usize>, count: usize) -> usize {

// }