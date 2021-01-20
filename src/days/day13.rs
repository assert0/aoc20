use std::fs;
use crate::days::utils;

#[derive(Debug, Clone)]
pub struct Schedule {
    depart: i64,
    buses: Vec<i64>,
}

impl Schedule {

    pub fn parse(contents: &str) -> Schedule {
        let mut lines = contents.lines();
        let depart = lines.next().unwrap().parse::<i64>().unwrap();
        let buses = lines.next().unwrap()
                         .replace("x","1")
                         .split(",")
                         .map(|b| b.parse::<i64>().unwrap())
                         .collect();
        Schedule { depart, buses }
    }

}

pub fn calc_part1(schedule: &Schedule) -> i64 {
    let wait_times: Vec<(i64, i64)> = 
        schedule.buses
            .iter()
            .filter(|&b| *b != 1)
            .map(|&b| (b, (0..).step_by(b as usize)
                        .skip_while(|&i| i < schedule.depart)
                        .next().unwrap() - schedule.depart) )
            .collect();
    let min_wait = wait_times.iter().min_by_key(|(_b, i)| i).unwrap();
    min_wait.0 * min_wait.1
}

// Part 2 requires the Chinese Remainder Theorem https://crypto.stanford.edu/pbc/notes/numbertheory/crt.html
pub fn calc_part2(modulii: &Vec<i64>) -> i64 {
    let residues: Vec<i64> = (0..modulii.len() as i64).collect();
 
    let remainder = utils::chinese_remainder(&residues, &modulii).unwrap();
    modulii.iter().product::<i64>() - remainder
}

pub fn day13(args: &[String]) -> i32 {
    println!("Day 13");
    if args.len() != 1 {
        println!("Missing input file");
        return -1;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let schedule = Schedule::parse(&contents);
    //println!("{:?}", schedule);

    println!("Part 1: {:?}", calc_part1(&schedule));

    println!("Part 2: {:?}", calc_part2(&schedule.buses));

    0
}
