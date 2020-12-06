use std::fs;
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Seat {
    id: usize,
}

impl Seat {

    pub fn parse(line: &str) -> Seat {
        lazy_static! {
            static ref SEAT: Regex = Regex::new(r"^[FB]{7}[LR]{3}").unwrap();
        }
        assert!(SEAT.is_match(&line));
        let mut id = 0;
        let bits: Vec<bool> = line.chars().map(|c| Seat::mapping(c)).collect();
        for b in bits {
            id <<= 1;
            if b {
                id |= 1
            }
        }
        Seat { id }
    }

    fn mapping(c: char) -> bool {
        match c {
            'B' | 'R' => true,
            _ => false
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

}

pub fn day5(args: &[String]) -> i32 {
    println!("Day 5");
    if args.len() != 1 {
        println!("Missing input file");
        return -1;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut ids: Vec<usize> = contents.lines().map(|l| Seat::parse(l).id()).collect();
    ids.sort();
    println!("Part 1: {}", ids.last().unwrap());
    
    // Find the open seat between the min and max seat IDs
    let mut i = *ids.first().unwrap();
    for id in &ids {
        while i < *id {
            println!("Part 2: {}", i);
            i += 1;
        }
        i += 1;
    }
    // a single missing seat can also be found by summing 
    // the all available seat IDs and substract the sum of filled seats.
    let rsum: usize = (*ids.first().unwrap()..*ids.last().unwrap()+1).sum(); 
    println!("Part 2: {:?}", rsum - ids.into_iter().sum::<usize>());
    0
}
