use std::fs;
use itertools::Itertools;

pub fn day1(args: &[String]) -> i32 {
    println!("Day 1");
    if args.len() != 1 {
        println!("Missing input file");
        return -1;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let entries: Vec<i32> = contents.lines().map(|l| l.parse::<i32>().unwrap()).collect();

    println!("Part 1: {:}", find1(&entries, 2020).unwrap());
    println!("Part 2: {:}", find2(&entries, 2020).unwrap());

    println!("Part 1: {:}", find(&entries, 2020, 2).unwrap());
    println!("Part 2: {:}", find(&entries, 2020, 3).unwrap());

    0
}

fn find1(entries: &Vec<i32>, sum: i32) -> Option<i32> {
    let mut r = None;
    for i in 0..entries.len()-1 {
        for j in i+1..entries.len() {
            if entries[i] + entries[j] == sum {
                r = Some(entries[i] * entries[j]);
            }
        }
    }
    r
}

fn find2(entries: &Vec<i32>, sum: i32) -> Option<i32> {
    let mut r = None;
    for i in 0..entries.len()-1 {
        for j in i+1..entries.len() {
            for k in j+1..entries.len() {
                if entries[i] + entries[j] + entries[k] == sum {
                    r = Some(entries[i] * entries[j] * entries[k]);
                }
            }
        }
    }
    r
}

fn find(entries: &Vec<i32>, sum: i32, count: usize) -> Option<i32> {
    let mut r = None;
    entries.iter().cloned().combinations(count).for_each(|c| {
        if c.iter().sum::<i32>() == sum {
            r = Some(c.iter().product::<i32>());
        }
    });
    r
}