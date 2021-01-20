use std::ops::Range;
use std::fs;
use std::fmt;
use std::collections::HashMap;
use itertools::iproduct;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum State {
    Active,
    Inactive,
}

impl State {
    pub fn parse(c: char) -> State {
        match c {
            '#' => State::Active,
            '.' => State::Inactive,
            _ => unreachable!("Invalid option"),
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            State::Active => '#',
            State::Inactive => '.',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Dimensions {
    Three,
    Four,
}

#[derive(Debug, Clone)]
pub struct PocketDimension {
    map: HashMap<(isize, isize, isize, isize), State>,
    length: isize,
    dimensions: Dimensions
}

impl PocketDimension {

    pub fn new(input: &String, dimensions: Dimensions) -> PocketDimension {
        let init: Vec<Vec<State>> = input.lines()
                    .map(|l| l.chars().map(|c| State::parse(c)).collect())
                    .collect();
        assert_eq!(init.len(), init[0].len());
        let length = init.len() as isize;
        let mut map = HashMap::new();
        input.lines().zip(PocketDimension::get_range(length))
             .for_each(|(l, y)| 
                l.chars().zip(PocketDimension::get_range(length))
                 .for_each(|(c, x)| match map.insert((0, 0, y, x), State::parse(c)) {_ => ()} )
             );
        PocketDimension { map, length, dimensions }
    }

    pub fn get_range(v: isize) -> Range<isize> {
        Range { start: 0-v/2, end: v-v/2 }
    }

    pub fn range(&self) -> Range<isize> {
        PocketDimension::get_range(self.length)
    }

    pub fn get(&self, w: isize, z: isize, y: isize, x: isize) -> State {
        match self.map.get(&(w, z, y, x)) {
            Some(s) => s.clone(),
            _ => State::Inactive
        }
    }

    pub fn adjacent_active_count(&self, w: isize, z: isize, y: isize, x: isize) -> usize {
        let count = match self.dimensions {
            Dimensions::Three => iproduct!(-1..2, -1..2, -1..2)
                                    .map(|(dz, dy, dx)| self.get(0, z+dz, y+dy, x+dx))
                                    .filter(|&b| b == State::Active)
                                    .count(),
            Dimensions::Four => iproduct!(-1..2, -1..2, -1..2, -1..2)
                                    .map(|(dw, dz, dy, dx)| self.get(w+dw, z+dz, y+dy, x+dx))
                                    .filter(|&b| b == State::Active)
                                    .count()
        };
        // don't count the current cube
        match self.get(w, z, y, x) {
            State::Active => count - 1,
            _ => count
        }
    }

    // determine position's next state
    pub fn next_state(&self, w: isize, z: isize, y: isize, x: isize) -> State {
        let count = self.adjacent_active_count(w, z, y, x);
        match self.get(w, z, y, x) {
            State::Active => {
                match count {
                    2 | 3 => State::Active,
                    _ => State::Inactive
                }
            },
            State::Inactive => {
                match count {
                    3 => State::Active,
                    _ => State::Inactive
                }
            },
        }
    }

    pub fn run_round(&mut self) {
        let mut next = HashMap::new();
        self.length += 2;
        match self.dimensions {
            Dimensions::Three => iproduct!(self.range(), self.range(), self.range())
                                    .for_each(|(z, y, x)| match next.insert((0, z, y, x), self.next_state(0, z, y, x)) {_=>()}),
            Dimensions::Four => iproduct!(self.range(), self.range(), self.range(), self.range())
                                    .for_each(|(w, z, y, x)| match next.insert((w, z, y, x), self.next_state(w, z, y, x)) {_=>()})
        };
        self.map = next;
    }

    pub fn count(&self) -> usize {
        iproduct!(self.range(), self.range(), self.range(), self.range())
            .filter(|&(w, z, y, x)| self.get(w, z, y, x) == State::Active)
            .count()
    }

}

impl fmt::Display for PocketDimension {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = Vec::new();
        match self.dimensions {
            Dimensions::Three => {
                for z in self.range() {
                    output.push(format!("z={}\n", z));
                    for y in self.range() {
                        for x in self.range() {
                            output.push(format!("{}", self.get(0, z, y, x)));
                        }
                        output.push(String::from("\n"));
                    }
                }
            },
            Dimensions::Four => {
                iproduct!(self.range(), self.range()).for_each(|(w, z)| {
                    output.push(format!("z={}, w={}\n", z, w));
                    for y in self.range() {
                        for x in self.range() {
                            output.push(format!("{}", self.get(w, z, y, x)));
                        }
                        output.push(String::from("\n"));
                    }
                });
            }
        };
        write!(f, "{}", output.join(""))
    }
}

pub fn day17(args: &[String]) -> i32 {
    println!("Day 17");
    if args.len() != 1 {
        println!("Missing input file");
        return -1;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut p = PocketDimension::new(&contents, Dimensions::Three);
    for _r in 0..6 {
        p.run_round();
    }
    println!("Part 1: {}", p.count());

    let mut p = PocketDimension::new(&contents, Dimensions::Four);
    for _r in 0..6 {
        p.run_round();
    }
    println!("Part 2: {}", p.count());

    0
}