use std::fs;
use aoc20::days::day3::{Geology};

fn geology() -> Geology {
    let contents = fs::read_to_string("data/day3example.txt")
            .expect("Something went wrong reading the file");
    Geology::new(&contents)
}

#[test]
fn day3_parse() {
    let geo = geology();
    assert_eq!(geo.width(), 11);
    assert_eq!(geo.height(), 11);
}

#[test]
fn day3_check_trees() {
    let geo = geology();
    assert_eq!(geo.is_tree(10, 10), true);
    assert_eq!(geo.is_tree(2, 1), true);
    assert_eq!(geo.is_tree(0, 1), false);
    assert_eq!(geo.is_tree(0, 13), true);
}

#[test]
fn day3_hit_counts() {
    let geo = geology();
    // provided slopes and hit counts
    assert_eq!(geo.hit_trees(1, 1), 2);
    assert_eq!(geo.hit_trees(1, 3), 7);
    assert_eq!(geo.hit_trees(1, 5), 3);
    assert_eq!(geo.hit_trees(1, 7), 4);
    assert_eq!(geo.hit_trees(2, 1), 2);
}