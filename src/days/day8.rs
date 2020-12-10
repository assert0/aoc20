use std::fs;
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction {
    operation: String,
    argument: isize,
}

impl Instruction {

    pub fn new(operation: String, argument: isize) -> Instruction {
        Instruction { operation, argument }
    }

    pub fn parse(line: &str) -> Instruction {
        // ex "acc +6"
        lazy_static! {
            static ref INSTR: Regex = Regex::new(r"^(nop|acc|jmp) ([\-\+]\d+)$").unwrap();
        }
        assert!(INSTR.is_match(&line));
        let caps = INSTR.captures(line).unwrap();
        let operation = caps.get(1).unwrap().as_str().to_string();
        let argument = caps.get(2).unwrap().as_str().parse::<isize>().unwrap();
        Instruction { operation, argument }
    }

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pc: usize,
    acc: isize,
    code: Vec<Instruction>,
    run_count: Vec<usize>
}

impl Program {

    pub fn new(code: Vec<Instruction>) -> Program {
        let count = code.len();
        Program { pc: 0, acc: 0, code: code, run_count: vec![0; count] }
    }

    // run program, returning the acc result
    pub fn run(&mut self) -> isize {
        loop {
            self.run_count[self.pc] += 1; //increment instuction's run count
            let i = &self.code[self.pc];
            //println!("pc: {} - acc: {} - {:?} ", self.pc, self.acc, i);
            match i.operation.as_str() {
                "nop" => {
                    self.pc += 1;
                },
                "acc" => {
                    self.acc += i.argument;
                    self.pc += 1;
                },
                "jmp" => {
                    self.pc = (self.pc as isize + i.argument) as usize;
                },
                _ => unreachable!("Invalid instruction: {:?}", i)
            }
            if self.complete() || self.looping() {
                break;
            }
        }
        self.acc
    }

    pub fn looping(&self) -> bool {
        self.run_count[self.pc] > 0
    }

    pub fn complete(&self) -> bool {
        self.pc >= self.code.len()
    }

    pub fn corrupt_instruction(&mut self, index: usize) {
        let mut i = &mut self.code[index];
        match i.operation.as_str() {
            "nop" => {
                i.operation = String::from("jmp")
            },
            "jmp" => {
                i.operation = String::from("nop")
            },
            _ => unreachable!("Invalid corruption: {:?}", i)
        }
    }

}

pub fn day8(args: &[String]) -> i32 {
    println!("Day 8");
    if args.len() != 1 {
        println!("Missing input file");
        return -1;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let code: Vec<Instruction> = contents.lines().map(|l| Instruction::parse(l)).collect();
    //println!("{:?}", code);
    let mut program = Program::new(code.clone());

    println!("Part 1: {:?}", program.run());

    println!("Part 2: {:?}", part2(Program::new(code.clone())));
    0
}

pub fn part2(program: Program) -> isize {
    let corrupt_locations = program.code.iter().enumerate()
                                        .filter(|(_i, c)| c.operation == "jmp" || c.operation == "nop")
                                        .map(|(i, _c)| i)
                                        .collect::<Vec<usize>>();

    for loc in corrupt_locations {
        //println!("Corrupting location {}", loc);
        let mut corrupt = program.clone();
        corrupt.corrupt_instruction(loc);

        let acc = corrupt.run();
        if corrupt.complete() {
            return acc;
        }
    }
    0
}