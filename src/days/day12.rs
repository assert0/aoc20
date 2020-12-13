use std::fs;
use regex::Regex;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    pub fn to_degrees(d: Direction) -> usize {
        match d {
            Direction::North => 0,
            Direction::East => 90,
            Direction::South => 180,
            Direction::West => 270,
        }
    }

    pub fn from_degrees(deg: isize) -> Direction {
        match deg % 360 {
            0 => Direction::North,
            90 => Direction::East,
            180 => Direction::South,
            270 => Direction::West,
            _ => panic!("Unknown degree: {}", deg)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Ship {
    position: (isize, isize),
    waypoint: (isize, isize),
    facing: Direction
}

impl Ship {

    // New ship at the origin, facing east, waypoint at 10 east 1 north
    pub fn new() -> Ship {
        Ship { position: (0, 0), waypoint: (10, 1), facing: Direction::East }
    }

    pub fn turn(&mut self, degrees: isize) {
        self.facing = Direction::from_degrees(Direction::to_degrees(self.facing) as isize + degrees);
    }

    pub fn turn_waypoint(&mut self, degrees: usize) {
        match degrees {
            0 => (),
            90 => self.waypoint = (self.waypoint.1, -1 * self.waypoint.0),
            180 => self.waypoint = (-1 * self.waypoint.0, -1 * self.waypoint.1),
            270 => self.waypoint = (-1 * self.waypoint.1, self.waypoint.0),
            _ => panic!("Unknown degree: {}", degrees),
        }

    }

    pub fn step(&mut self, instr: &Instruction) {
        let movement = match instr.action {
            'F' => Some(self.facing),
            'N' => Some(Direction::North),
            'E' => Some(Direction::East),
            'S' => Some(Direction::South),
            'W' => Some(Direction::West),
            _ => None
        };
        match movement {
            Some(s) => match s {
                Direction::North => self.position.1 += instr.value as isize,
                Direction::South => self.position.1 -= instr.value as isize,
                Direction::East => self.position.0 += instr.value as isize,
                Direction::West => self.position.0 -= instr.value as isize,
            }
            None => match instr.action {
                'L' => self.turn(360 - instr.value as isize),
                'R' => self.turn(instr.value as isize),
                _ => panic!("Invalid action {}", instr.action)
            }
        }
    }

    pub fn step_waypoint(&mut self, instr: &Instruction) {
        let move_waypoint = match instr.action {
            'N' => Some(Direction::North),
            'E' => Some(Direction::East),
            'S' => Some(Direction::South),
            'W' => Some(Direction::West),
            _ => None
        };
        match move_waypoint {
            Some(s) => match s {
                Direction::North => self.waypoint.1 += instr.value as isize,
                Direction::South => self.waypoint.1 -= instr.value as isize,
                Direction::East => self.waypoint.0 += instr.value as isize,
                Direction::West => self.waypoint.0 -= instr.value as isize,
            }
            None => match instr.action {
                'F' => {
                    self.position.0 += self.waypoint.1 * instr.value as isize;
                    self.position.1 += self.waypoint.0 * instr.value as isize;
                },
                'L' => self.turn_waypoint(360 - instr.value),
                'R' => self.turn_waypoint(instr.value),
                _ => panic!("Invalid action {}", instr.action)
            }
        }
    }

    pub fn manhattan_distance(&self) -> usize {
        (self.position.0.abs() + self.position.1.abs()) as usize
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction {
    action: char,
    value: usize,
}

impl Instruction {

    pub fn new(action: char, value: usize) -> Instruction {
        Instruction { action, value }
    }

    pub fn parse(line: &str) -> Instruction {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"([NESWLRF])(\d+)").unwrap();
        }
        let caps = RE.captures(line).unwrap();
        let action = caps.get(1).unwrap().as_str().chars().nth(0).unwrap();
        let value = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        Instruction::new(action, value)
    }
}

pub fn day12(args: &[String]) -> i32 {
    println!("Day 12");
    if args.len() != 1 {
        println!("Missing input file");
        return -1;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let instructions: Vec<Instruction> = contents.lines().map(|l| Instruction::parse(l)).collect();
    let mut ship = Ship::new();
    instructions.into_iter().for_each(|i| ship.step(&i));
    println!("Part 1: {:?}", ship.manhattan_distance());

    let instructions: Vec<Instruction> = contents.lines().map(|l| Instruction::parse(l)).collect();
    let mut ship = Ship::new();
    instructions.into_iter().for_each(|i| ship.step_waypoint(&i));
    println!("Part 2: {:?}", ship.manhattan_distance());
    0
}
