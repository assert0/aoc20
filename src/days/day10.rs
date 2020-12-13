use std::fs;

pub fn day10(args: &[String]) -> i32 {
    println!("Day 10");
    if args.len() != 1 {
        println!("Missing input file");
        return -1;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let ratings: Vec<usize> = contents.lines().map(|l| l.parse::<usize>().unwrap()).collect();

    let p1 = part1(&ratings);
    println!("{:?}", p1);
    println!("Part 1: {:?}", p1.0 * p1.2);
    0
}

pub fn part1(ratings: &Vec<usize>) -> (usize, usize, usize) {
    let mut sorted = ratings.clone();
    sorted.sort();
    // built-in adapter is always 3 higher than the highest adapter, so
    // seed the 3 difference by 1 
    let mut counts = ( 0, 0, 1 );
    let mut jolts = 0;
    for r in sorted {
        match r - jolts {
            3 => counts.2 += 1,
            2 => counts.1 += 1,
            1 => counts.0 += 1,
            _ => assert!(false, "Invalid difference")
        }
        jolts = r;
    }
    counts
}