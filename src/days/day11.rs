use std::fs;
use std::fmt;
use itertools::iproduct;

pub const MAX_MAGNITUDE: isize = 1_000;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum State {
    Open,
    Occupied,
    Floor,
}

impl State {
    pub fn parse(c: char) -> State{
        match c {
            'L' => State::Open,
            '#' => State::Occupied,
            '.' => State::Floor,
            _ => unreachable!("Invalid option"),
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            State::Open => 'L',
            State::Occupied => '#',
            State::Floor => '.',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, Clone)]
pub struct SeatLayout {
    map: Vec<Vec<State>>
}

impl SeatLayout {

    pub fn new(input: &String) -> SeatLayout {
        let map: Vec<Vec<State>> = input.lines()
                    .map(|l| l.chars().map(|c| State::parse(c)).collect())
                    .collect();
        SeatLayout { map }
    }

    pub fn width(&self) -> usize {
        self.map[0].len()
    }

    pub fn height(&self) -> usize {
        self.map.len()
    }

    pub fn is_state(&self, y: isize, x: isize, state: State) -> bool {
        match self.get(y, x) {
            Some(s) => s == state,
            _ => false
        }
    }

    pub fn is_valid_position(&self, y: isize, x: isize) -> bool {
        x >= 0 && y >=0 &&
            x < self.width() as isize && y < self.height() as isize
    }

    pub fn get(&self, y: isize, x: isize) -> Option<State> {
        if self.is_valid_position(y, x) {
            return Some(self.map[y as usize][x as usize]);
        }
        None
    }

    pub fn is_occupied(&self, position: (isize, isize), vector: (isize, isize), max_magnitude: isize) -> bool {
        for m in 1..max_magnitude+1 {
            match self.get(position.0 + vector.0 * m, position.1 + vector.1 * m) {
                Some(s) => match s {
                    State::Occupied => return true,
                    State::Open => break,
                    State::Floor => continue,
                },
                None => break,
            };
        }
        false
    }

    pub fn adjacent_occupied_count(&self, y: usize, x: usize, max_magnitude: isize) -> usize {
        lazy_static! {
            static ref VECTORS: Vec<(isize, isize)> = vec![
                (0, 1), (0, -1), (1, 0), (-1, 0),
                (1, 1), (1, -1), (-1, 1), (-1, -1)
            ];
        }
        assert!(self.is_valid_position(y as isize, x as isize));
        VECTORS.iter()
               .map(|&vector| self.is_occupied((y as isize, x as isize), vector, max_magnitude))
               .filter(|&b| b)   
               .count()
    }

    pub fn state_count(positions: &Vec<Option<State>>, state: State) -> usize {
        positions.iter().filter(|&p| *p == Some(state))
                 .count()
    }

    // determine seat's next state
    pub fn next_state_part1(&self, y: usize, x: usize) -> State {
        let count = self.adjacent_occupied_count(y, x, 1);
        match self.map[y][x] {
            State::Open => {
                if count == 0 {
                    State::Occupied
                } else {
                    State::Open
                }
            },
            State::Occupied => {
                if count >= 4 {
                    State::Open
                } else {
                    State::Occupied
                }
            },
            State::Floor => {
                State::Floor
            },
        }
    }

    // determine seat's next state
    pub fn next_state_part2(&self, y: usize, x: usize) -> State {
        let count = self.adjacent_occupied_count(y, x, MAX_MAGNITUDE);
        match self.map[y][x] {
            State::Open => {
                if count == 0 {
                    State::Occupied
                } else {
                    State::Open
                }
            },
            State::Occupied => {
                if count >= 5 {
                    State::Open
                } else {
                    State::Occupied
                }
            },
            State::Floor => {
                State::Floor
            },
        }
    }

    pub fn run_round_part1(&mut self) -> usize {
        let mut next = self.map.clone();
        iproduct!(0..self.height(), 0..self.width())
            .for_each(|(y, x)| next[y][x] = self.next_state_part1(y, x));
        let changed_count = iproduct!(0..self.height(), 0..self.width())
                    .map(|(y, x)| self.map[y][x] != next[y][x])
                    .filter(|&b| b)
                    .count();
        self.map = next;
        changed_count
    }

    pub fn run_round_part2(&mut self) -> usize {
        let mut next = self.map.clone();
        iproduct!(0..self.height(), 0..self.width())
            .for_each(|(y, x)| next[y][x] = self.next_state_part2(y, x));
        let changed_count = iproduct!(0..self.height(), 0..self.width())
                    .map(|(y, x)| self.map[y][x] != next[y][x])
                    .filter(|&b| b)
                    .count();
        self.map = next;
        changed_count
    }

    pub fn count(&self, state: State) -> usize {
        iproduct!(0..self.height(), 0..self.width())
            .map(|(y, x)| self.map[y][x] == state)
            .filter(|&b| b)
            .count()
    }

}

impl fmt::Display for SeatLayout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = Vec::new();
        for y in 0..self.height() {
            for x in 0..self.width() {
                output.push(format!("{}", self.map[y][x]));
            }
            output.push(String::from("\n"));
        }
        write!(f, "{}", output.join(""))
    }
}


pub fn day11(args: &[String]) -> i32 {
    println!("Day 11");
    if args.len() != 1 {
        println!("Missing input file");
        return -1;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut layout = SeatLayout::new(&contents);
    loop {
        //println!("{}", layout);
        if layout.run_round_part1() == 0 {
            break;
        }
    }
    println!("Part 1: {}", layout.count(State::Occupied));

    layout = SeatLayout::new(&contents);
    loop {
        //println!("{}", layout);
        if layout.run_round_part2() == 0 {
            break;
        }
    }
    println!("Part 2: {}", layout.count(State::Occupied));
    0
}
