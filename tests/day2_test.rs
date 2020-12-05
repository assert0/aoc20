use std::fs;
use aoc20::days::day2::Policy;

fn passwords() -> Vec<(Policy, String)> {
    let contents = fs::read_to_string("data/day2example.txt")
            .expect("Something went wrong reading the file");
    contents.lines().map(|l| Policy::parse(l)).collect()
}

#[test]
fn day2_parse() {
    let (p, pwd) = Policy::parse("1-3 a: abcde");
    assert_eq!(p, Policy::new((1, 3), 'a'));
    assert_eq!(pwd, String::from("abcde"));
}

#[test]
fn day2_part1() {
    let result: Vec<bool> = passwords().iter().map(|(p, pwd)| p.is_valid_part1(pwd)).collect();
    assert_eq!(result, vec![true, false, true]);
}

#[test]
fn day2_part2() {
    let result: Vec<bool> = passwords().iter().map(|(p, pwd)| p.is_valid_part2(pwd)).collect();
    assert_eq!(result, vec![true, false, false]);
}