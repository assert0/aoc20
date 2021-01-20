use std::fs;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mask {
    initial: u64,
    mask: u64,
}

impl Mask {

    pub fn new(initial: u64, mask: u64) -> Mask {
        Mask { initial, mask }
    }

    pub fn parse(line: &str) -> Option<Mask> {
        // ex "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"
        lazy_static! {
            static ref MASK: Regex = Regex::new(r"^mask\s=\s([X01]{36})$").unwrap();
        }
        if !MASK.is_match(&line) {
            return None;
        }
        let caps = MASK.captures(line).unwrap();
        let m = caps.get(1).unwrap().as_str();
        let initial = u64::from_str_radix(&m.replace("X", "0"), 2).unwrap();
        let mask = u64::from_str_radix(&m.replace("1", "0").replace("X", "1"), 2).unwrap();
        Some(Mask { initial, mask })
    }

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mem {
    location: u64,
    value: u64,
}

impl Mem {

    pub fn new(location: u64, value: u64) -> Mem {
        Mem { location, value }
    }

    pub fn parse(line: &str) -> Option<Mem> {
        // ex "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"
        lazy_static! {
            static ref MEM: Regex = Regex::new(r"^mem\[(\d+)\]\s=\s(\d+)$").unwrap();
        }
        if !MEM.is_match(&line) {
            return None;
        }
        let caps = MEM.captures(line).unwrap();
        let location = caps.get(1).unwrap().as_str().parse::<u64>().unwrap();
        let value = caps.get(2).unwrap().as_str().parse::<u64>().unwrap();
        Some(Mem { location, value })
    }

}

pub fn part1(contents: String) -> u64 {
    let mut memory = HashMap::<u64, u64>::new();
    let mut lines = contents.lines(); 
    let mut nextline = lines.next();
    loop {
        if nextline.is_none() {
            break;
        }
        let mask = Mask::parse(nextline.unwrap());
        if mask.is_none() {
            break;
        }
        //println!("{:?}", mask);
        loop {
            nextline = lines.next();
            if nextline.is_none() {
                break;
            }
            let mem = Mem::parse(nextline.unwrap());
            if mem.is_none() {
                break; // is a mask line
            }
            let mk = mask.clone().unwrap();
            let mm = mem.clone().unwrap();
            memory.insert(mm.location, mk.initial + (mm.value & mk.mask));
            //println!("{:?}", mem);
        }
    }
    memory.iter().map(|(_, v)| v).sum()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemMask {
    initial: u64,
    mask: u64,
    floating: Vec<u8>,
    curr: u64
}

impl MemMask {

    pub fn new(initial: u64, mask: u64, floating: Vec<u8>) -> MemMask {
        MemMask { initial, mask, floating, curr: 0 }
    }

    pub fn parse(line: &str) -> Option<MemMask> {
        // ex "mask = 000000000000000000000000000000X1001X"
        lazy_static! {
            static ref MASK: Regex = Regex::new(r"^mask\s=\s([X01]{36})$").unwrap();
        }
        if !MASK.is_match(&line) {
            return None;
        }
        let caps = MASK.captures(line).unwrap();
        let m = caps.get(1).unwrap().as_str();
        let initial = u64::from_str_radix(&m.replace("X", "0"), 2).unwrap();
        let mask = u64::from_str_radix(&m.replace("0", "1").replace("X", "0"), 2).unwrap();
        let floating = MemMask::bits_to_positions(
                u64::from_str_radix(&m.replace("1", "0").replace("X", "1"), 2).unwrap());
        Some(MemMask { initial, mask, floating, curr: 0 })
    }

    pub fn condition(&self, location: u64) -> u64 {
        (location | self.initial) & self.mask
    }

    fn bits_to_positions(mut positions: u64) -> Vec<u8> {
        let mut v = Vec::new();
        let mut p: u8 = 0;
        while positions > 0 {
            if positions & 1 == 1 {
                v.push(p);
            }
            positions >>= 1;
            p += 1;
        }
        v
    }

}

impl Iterator for MemMask {
    type Item = u64;
    
    fn next(&mut self) -> Option<u64> {
        if self.curr >= 2u64.pow(self.floating.len() as u32) {
            return None;
        }
        let mut val: u64 = self.curr;
        let mut curr_mask: u64 = 0;
        for p in &self.floating {
            curr_mask |= (val & 1) << p;
            val >>= 1;
        }
        self.curr += 1;
        Some(curr_mask)
    }
}


pub fn part2(contents: String) -> u64 {
    let mut memory = HashMap::<u64, u64>::new();
    let mut lines = contents.lines(); 
    let mut nextline = lines.next();
    loop {
        if nextline.is_none() {
            break;
        }
        let mask = MemMask::parse(nextline.unwrap());
        if mask.is_none() {
            break;
        }
        //println!("{:?}", mask);
        loop {
            nextline = lines.next();
            if nextline.is_none() {
                break;
            }
            let mem = Mem::parse(nextline.unwrap());
            if mem.is_none() {
                break; // is a mask line
            }
            let mut mk = mask.clone().unwrap();
            let mm = mem.clone().unwrap();
            let initloc = mk.condition(mm.location);
            while let Some(loc) = mk.next() {
                memory.insert(initloc | loc, mm.value);
                //println!("{:?} {:?}", initloc | loc, mm.value);
            }
        }
    }
    memory.iter().map(|(_, v)| v).sum()
}

pub fn day14(args: &[String]) -> i32 {
    println!("Day 14");
    if args.len() != 1 {
        println!("Missing input file");
        return -1;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("Part 1: {}", part1(contents.clone()));

    println!("Part 2: {}", part2(contents.clone()));

    0
}
