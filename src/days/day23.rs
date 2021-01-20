use std::fs;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrabCups {
    cups: Vec<u8>
}

impl CrabCups {

    pub fn new(cups: Vec<u8>) -> CrabCups {
        CrabCups { cups }
    }

    pub fn parse(line: &str) -> CrabCups {
        let cups = line.chars().map(|c| c.to_string().parse::<u8>().unwrap()).collect();
        CrabCups { cups }
    }

    pub fn next(&mut self) {
        let (first, pickup, mut remaining) = (self.cups.get(0).unwrap(), self.cups.get(1..4).unwrap(), self.cups.get(4..9).unwrap().to_vec());
        //println!("{:?} {:?} {:?}", first, pickup, remaining);
        let mut find = *first;
        let mut pos;
        loop {
            find -= 1;
            if find == 0 {
                find = 9;
            }
            pos = remaining.iter().position(|&i| i == find);
            if pos.is_some() {
                break;
            }
        }
        //println!("max: {:?}", pos);
        pickup.iter().rev().for_each(|&i| remaining.insert(pos.unwrap() + 1, i));
        remaining.push(*first);
        self.cups = remaining;
        //println!("{:?}", self.cups);
    }

    pub fn answer(&self) -> String {
        match self.cups.iter().position(|&n| n == 1) {
            Some(pos) => {
                String::from(format!("{}{}", 
                    CrabCups::vec2str(&self.cups[pos+1..].to_vec()), 
                    CrabCups::vec2str(&self.cups[0..pos].to_vec()))
                )
            },
            None => String::from("No answer?")
        }
    }

    pub fn vec2str(vec: &Vec<u8>) -> String {
        vec.iter().map(|i| format!("{}", i)).collect::<Vec<String>>().join("")
    }

}

pub fn day23(args: &[String]) -> i32 {
    println!("Day 23");
    if args.len() != 1 {
        println!("Missing input file");
        return -1;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut game = CrabCups::parse(&contents);
    //println!("{:?}", game);
    for _i in 0..100 {
        game.next();
    }
    println!("Part 1: {}", game.answer());
    
    0
}