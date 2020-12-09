use std::fs;
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bag {
    color_code: String,
    count: usize,
}

impl Bag {

    pub fn new(color_code: String, count: usize) -> Bag {
        Bag{ color_code, count }
    }

    pub fn parse(text: &str) -> Option<Bag> {
        // Split out bag info (ex: "1 dotted black bag")
        lazy_static! {
            static ref BAG: Regex = Regex::new(r"^(\d+) (\w+ \w+) bags?\.?$").unwrap();
        }
        if !BAG.is_match(&text) {
            return None;
        }
        let caps = BAG.captures(text).unwrap();
        let count = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let color_code = caps.get(2).unwrap().as_str().to_string();
        Some(Bag{ color_code, count })
    }

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
    outer: Bag,
    inner: Vec<Bag>,
}

impl Rule {

    pub fn new(outer: Bag, inner: Vec<Bag>) -> Rule {
        Rule { outer, inner }
    }

    pub fn parse(line: &str) -> Rule {
        let p: Vec<&str> = line.split(" contain ").collect();
        assert_eq!(p.len(), 2);
        Rule { 
            outer: Bag::parse(&format!("1 {}", p[0])).unwrap(), // Workaround to add a '1' count to the outer bag
            inner: p[1].split(", ")
                       .map(|t| Bag::parse(t))
                       .filter(|x| x.is_some())
                       .map(|x| x.unwrap())
                       .collect()
        }
    }
}

pub fn day7(args: &[String]) -> i32 {
    println!("Day 7");
    if args.len() != 1 {
        println!("Missing input file");
        return -1;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let rules: Vec<Rule> = contents.lines().map(|l| Rule::parse(l)).collect();
    println!("Rules {:?}", rules.len());

    // Part 1
    println!("Part 1: {}", part1("shiny gold", &rules));

    // Part 2
    println!("Part 2: {}", part2("shiny gold", &rules));
    0
}

pub fn part1(color_code: &str, rules: &Vec<Rule>) -> usize {
    let result: Vec<(String, bool)> = 
        rules.iter()
             .map(|r| (r.outer.color_code.to_string(), contains_bag(color_code, &r, &rules)))
             .collect();
    // count the bags that can contain (excluding itself, the reson for the -1)
    result.iter().filter(|(_c, r)| *r).count() - 1
}

pub fn contains_bag(color_code: &str, current: &Rule, rules: &Vec<Rule>) -> bool {
    if color_code == &current.outer.color_code {
        return true;
    }
    for i in &current.inner {
        // find the bag rule
        for r in rules {
            if r.outer.color_code == i.color_code {
                // recursively traverse tree
                if contains_bag(&color_code, r, rules) {
                    return true;
                }
            }
        }
    }
    false
}

pub fn part2(color_code: &str, rules: &Vec<Rule>) -> usize {
    let r = rules.iter().find(|r| color_code == &r.outer.color_code);
    if r.is_some() {
        // count the bags (excluding itself, the reson for the -1)
        return inner_bag_count(r.unwrap(), rules) - 1;
    }
    0
}

pub fn inner_bag_count(current: &Rule, rules: &Vec<Rule>) -> usize {
    let mut bag_count = 1;
    for i in &current.inner {
        // find the bag rule
        for r in rules {
            if r.outer.color_code == i.color_code {
                // recursively traverse tree
                bag_count += i.count * inner_bag_count(r, rules);
            }
        }
    }
    bag_count
}