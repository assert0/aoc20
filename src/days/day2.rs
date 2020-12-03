use std::fs;
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Policy {
    nums: (usize, usize),
    letter: char,
}

impl Policy {

    pub fn new( nums: (usize, usize), letter: char) -> Policy {
        Policy { nums, letter }
    }

    pub fn parse(policypassword: &str) -> (Policy, String) {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
        }
        // Split out the policy & password line (ex: "1-3 a: abcde")
        let caps = RE.captures(policypassword).unwrap();
        let first = caps.get(1).map_or(0, |m| m.as_str().parse::<usize>().unwrap());
        let second = caps.get(2).map_or(0, |m| m.as_str().parse::<usize>().unwrap());
        let letter = caps.get(3).map_or('?', |m| m.as_str().chars().next().unwrap());
        let password = caps.get(4).map_or("", |m| m.as_str());

        let policy = Policy::new((first, second), letter);
        (policy, password.to_string())
    }

    //The password policy indicates the lowest and highest number of times a given letter must appear
    pub fn is_valid_part1(&self, password: &String) -> bool {
        let count = password.chars().filter(|&c| c == self.letter).count() as usize;
        //(self.nums.0..self.nums.1+1).contains(&count)
        (count >= self.nums.0) && (count <= self.nums.1)
    }

    //policy actually describes two positions in the password
    pub fn is_valid_part2(&self, password: &String) -> bool {
        let m = (
            password.chars().nth(self.nums.0-1).unwrap() == self.letter,
            password.chars().nth(self.nums.1-1).unwrap() == self.letter,
        );
        // valid if there is only 1 match
        (m.0 && !m.1) || (!m.0 && m.1)
    }
}

pub fn day2(args: &[String]) -> i32 {
    println!("Day 2");
    if args.len() != 1 {
        println!("Missing input file");
        return -1;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    
    let passwords: Vec<(Policy, String)> = contents.lines().map(|l| Policy::parse(l)).collect();
    //println!("{:?}", result);
    
    println!("Part 1: {:}", passwords.iter().filter(|(p, pwd)| p.is_valid_part1(pwd)).count());
    println!("Part 2: {:}", passwords.iter().filter(|(p, pwd)| p.is_valid_part2(pwd)).count());
    0
}
