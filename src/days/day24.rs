use std::fs;
use itertools::iproduct;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DirectionSequence {
    input: Vec<char>,
    pos: usize,
}

impl DirectionSequence {
   
    pub fn new(directions: &str) -> DirectionSequence {
        let pos = 0;
        let input = directions.to_string().chars().collect();
        DirectionSequence { input, pos }
    }

}

impl Iterator for DirectionSequence {
    type Item = Direction;
    
    fn next(&mut self) -> Option<Direction> {
        if self.pos >= self.input.len() {
            return None;
        }
        self.pos += 1;
        match self.input[self.pos-1] {
            'e' => return Some(Direction::East),
            'w' => return Some(Direction::West),
            'n' => {
                self.pos += 1;
                match self.input[self.pos-1] {
                    'e' => return Some(Direction::NorthEast),
                    'w' => return Some(Direction::NorthWest),
                    _ => (),
                };
            },
            's' => {
                self.pos += 1;
                match self.input[self.pos-1] {
                    'e' => return Some(Direction::SouthEast),
                    'w' => return Some(Direction::SouthWest),
                    _ => (),
                };
            },
            _ => (),
        };
        unreachable!();
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HexGrid {
    map: Vec<Vec<Color>>,
    size: usize,
}

impl HexGrid {
   
    pub fn new(size: usize) -> HexGrid {
        let map = vec![vec![Color::White; size]; size];
        HexGrid { map, size }
    }

    pub fn center(&self) -> (usize, usize) {
        (self.map.len()/2, self.map[0].len()/2)

    }

    pub fn flip(&mut self, tile: (usize, usize)) {
        self.map[tile.0][tile.1] = match self.map[tile.0][tile.1] {
            Color::Black => Color::White,
            Color::White => Color::Black,
        };
    }

    pub fn get_tile(&self, tile: (usize, usize), direction: Direction) -> Option<Color> {
        let t = (tile.0 as isize, tile.1 as isize);
        let pos: (isize, isize) = match direction {
            Direction::East => (t.0, t.1+1),
            Direction::West => (t.0, t.1-1),
            Direction::NorthEast => (t.0-1, t.1+1),
            Direction::NorthWest => (t.0-1, t.1),
            Direction::SouthEast => (t.0+1, t.1),
            Direction::SouthWest => (t.0+1, t.1-1),
        };
        if pos.0 < 0 || pos.1 < 0 || pos.0 >= self.size as isize || pos.1 >= self.size as isize {
            return None;
        } 
        Some(self.map[pos.0 as usize][pos.1 as usize].clone())
    }

    pub fn adjacent(&self, tile: (usize, usize)) -> Vec<Option<Color>> {
        let dirs = vec![Direction::East, Direction::West, Direction::NorthEast,
                        Direction::NorthWest, Direction::SouthEast, Direction::SouthWest];
        dirs.into_iter().map(|d| self.get_tile(tile, d)).collect()
    }

    pub fn update(&mut self, sequence: Vec<Direction>) {
        let mut pos = self.center();
        for d in sequence {
            pos = match d {
                Direction::East => (pos.0, pos.1+1),
                Direction::West => (pos.0, pos.1-1),
                Direction::NorthEast => (pos.0-1, pos.1+1),
                Direction::NorthWest => (pos.0-1, pos.1),
                Direction::SouthEast => (pos.0+1, pos.1),
                Direction::SouthWest => (pos.0+1, pos.1-1),
            };
        }
        self.flip(pos);
    }

    pub fn count(&self, color: Color) -> usize {
        self.map.iter().flatten().filter(|&c| *c==color).count()
    }

    pub fn next_state(&self, tile: (usize, usize)) -> Color {
        let adjacent_black_count = self.adjacent(tile).iter().filter(|&c| *c==Some(Color::Black)).count();
        match self.map[tile.0][tile.1] {
            Color::Black => match adjacent_black_count {
                1 | 2 => Color::Black,
                _ => Color::White,
            }
            Color::White => match adjacent_black_count {
                2 => Color::Black,
                _ => Color::White,
            }
        }
    }

    pub fn next_day(&mut self) -> usize {
        let mut next = self.map.clone();
        iproduct!(1..self.size-1, 1..self.size-1)
            .for_each(|(y, x)| next[y][x] = self.next_state((y, x)));
        self.map = next;
        self.count(Color::Black)
    }

}

pub fn day24(args: &[String]) -> i32 {
    println!("Day 24");
    if args.len() != 1 {
        println!("Missing input file");
        return -1;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    
    let sequences: Vec<Vec<_>>  = contents.lines().map(|l| DirectionSequence::new(l).into_iter().collect()).collect(); 
    //println!("{:?}", sequences);
    let mut hg = HexGrid::new(150);
    for s in sequences {
        hg.update(s);
    }
    println!("Part 1: {}", hg.count(Color::Black));
    let mut count = 0;
    for _d in 1..101 {
        count = hg.next_day()
        //println!("Day {} {:?}", d, count);
    }
    println!("Part 2: {}", count);

    0
}