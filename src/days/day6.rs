use std::fs;
use itertools::Itertools;
use regex::{Regex, Captures};

pub fn get_answer_lines(filename: &String) -> Vec<String> {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    // normalize the input by replacing only the first occurance of an endline with a space
    let re = Regex::new(r"([^\n])\n").unwrap();
    re.replace_all(&contents, |caps: &Captures| {
            format!("{} ", &caps[1])
        })
        .lines()
        .map(|l|l.trim().to_string())
        .collect()
}

pub fn day6(args: &[String]) -> i32 {
    println!("Day 6");
    if args.len() != 1 {
        println!("Missing input file");
        return -1;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    
    let answer_lines = get_answer_lines(filename);

    // Part 1
    let unique: Vec<usize> = answer_lines.iter().map(|a| part1_answers(a).len()).collect();
    println!("Part 1: {}", unique.iter().sum::<usize>());
    
    // Part 2
    let group_ans: Vec<usize> = answer_lines.iter().map(|a| part2_answers(&a).len()).collect();
    println!("Part 2: {:?}", group_ans.iter().sum::<usize>());
    0
}

pub fn part1_answers(line: &String) -> Vec<char> {
    line.replace(" ", "").chars().unique().collect()
}

pub fn part2_answers(line: &String) -> Vec<char> {
    static ALPHA: &str = "abcdefghijklmnopqrstuvwxyz";
    let mut result = vec![true; 26];
    for a in line.split(' ') {
        for (i, c) in ALPHA.chars().enumerate() {
            if !a.contains(c) {
                result[i] = false;
            }
        }
    }
    // it isn't really necessary to convert the bool array back to the
    // actual chars, but want to be consistent with the part1_answers return
    result.into_iter()
          .enumerate()
          .filter(|&(_i, b)| b)
          .map(|(i, _b)| ALPHA.chars().nth(i).unwrap())
          .collect()
}