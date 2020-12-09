use std::fs;
use aoc20::days::day7;

#[test]
fn day7_parse_bag() {
    assert_eq!(day7::Bag::parse("1 wavy maroon bag"),
        Some(day7::Bag::new(String::from("wavy maroon"), 1))
    );
    assert_eq!(day7::Bag::parse("5 clear tan bags."),
        Some(day7::Bag::new(String::from("clear tan"), 5))
    );
    assert_eq!(day7::Bag::parse("no other bags."),
        None
    );   
}

#[test]
fn day7_parse_line() {
    assert_eq!(
        day7::Rule::parse("light lavender bags contain 1 dotted black bag, 1 wavy maroon bag, 5 pale white bags, 1 clear tan bag."),
        day7::Rule::new(
            day7::Bag::parse("1 light lavender bags").unwrap(),
            vec![
                day7::Bag::parse("1 dotted black bag").unwrap(),
                day7::Bag::parse("1 wavy maroon bag").unwrap(),
                day7::Bag::parse("5 pale white bags").unwrap(),
                day7::Bag::parse("1 clear tan bag").unwrap()
            ]
        )
    );
    assert_eq!(
        day7::Rule::parse("dotted black bags contain no other bags."),
        day7::Rule::new(
            day7::Bag::parse("1 dotted black bags").unwrap(),
            vec![]
        )
    );
}

#[test]
fn day7_part1() {
    let contents = fs::read_to_string("data/day7example.txt")
        .expect("Something went wrong reading the file");

    let rules: Vec<day7::Rule> = contents.lines().map(|l| day7::Rule::parse(l)).collect();
    assert_eq!(day7::part1("shiny gold", &rules), 4);
}

#[test]
fn day7_part2() {
    let contents = fs::read_to_string("data/day7example.txt")
        .expect("Something went wrong reading the file");

    let rules: Vec<day7::Rule> = contents.lines().map(|l| day7::Rule::parse(l)).collect();
    assert_eq!(day7::part2("shiny gold", &rules), 32);
}

#[test]
fn day7_part2_sample2() {
    let contents = fs::read_to_string("data/day7example2.txt")
        .expect("Something went wrong reading the file");

    let rules: Vec<day7::Rule> = contents.lines().map(|l| day7::Rule::parse(l)).collect();
    assert_eq!(day7::part2("shiny gold", &rules), 126);
}