use std::fs;
use itertools::Itertools;

pub fn valid_sums(numbers: &Vec<usize>) -> Vec<usize> {
    numbers.iter()
           .combinations(2)
           .filter(|c| !c.iter().all_equal())
           .map(|c| c.into_iter().sum())
           .collect()
      
}

pub fn day9(args: &[String]) -> i32 {
    println!("Day 9");
    if args.len() != 1 {
        println!("Missing input file");
        return -1;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let numbers: Vec<usize> = contents.lines().map(|l| l.parse::<usize>().unwrap()).collect();

    let p1 = part1(&numbers, 25).unwrap();
    println!("Part 1: {}", p1);

    let p2 = part2(&numbers, p1).unwrap();
    println!("Part 2: {}", p2.0 + p2.1);
    0
}

pub fn part1(numbers: &Vec<usize>, count: usize) -> Option<usize> {
    for i in count..numbers.len() {
        let sums = valid_sums(&numbers[i-count..i].to_vec());
        if numbers[i] == 127 {
            println!("{:?}", &numbers[i-count..i].to_vec());
        }
        match sums.iter().find(|&&s| s == numbers[i]) {
            None => return Some(numbers[i]),
            _ => (),
        }
    }
    return None;
}

pub fn part2(numbers: &Vec<usize>, target: usize) -> Option<(usize, usize)> {
    for i in 0..numbers.len() {
        let mut sum = 0;
        for j in i..numbers.len() {
            if sum > target {
                break;
            }
            if sum < target {
                sum += numbers[j];
            } else {
                // matching sum
                let n = &numbers[i..j].to_vec();
                return Some((*n.iter().min().unwrap(), *n.iter().max().unwrap()));
            }
        }
    }
    return None;
}
