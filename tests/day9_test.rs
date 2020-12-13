use std::fs;
use aoc20::days::day9;

#[test]
fn day9_part1() {
    let contents = fs::read_to_string("data/day9example.txt")
        .expect("Something went wrong reading the file");

    let numbers: Vec<usize> = contents.lines().map(|l| l.parse::<usize>().unwrap()).collect();

    assert_eq!(day9::part1(&numbers, 5), Some(127));
}

#[test]
fn day9_part2() {
    let contents = fs::read_to_string("data/day9example.txt")
        .expect("Something went wrong reading the file");

    let numbers: Vec<usize> = contents.lines().map(|l| l.parse::<usize>().unwrap()).collect();

    assert_eq!(day9::part2(&numbers, 127), Some((15, 47)));
}