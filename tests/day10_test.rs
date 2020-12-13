use std::fs;
use aoc20::days::day10;

#[test]
fn day10_part1() {
    let contents = fs::read_to_string("data/day10example.txt")
        .expect("Something went wrong reading the file");

    let ratings: Vec<usize> = contents.lines().map(|l| l.parse::<usize>().unwrap()).collect();

    assert_eq!(day10::part1(&ratings), (7, 0, 5));
}

#[test]
fn day10_part1_sample2() {
    let contents = fs::read_to_string("data/day10example2.txt")
        .expect("Something went wrong reading the file");

    let ratings: Vec<usize> = contents.lines().map(|l| l.parse::<usize>().unwrap()).collect();

    assert_eq!(day10::part1(&ratings), (22, 0, 10));
}