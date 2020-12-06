use aoc20::days::day6;

#[test]
fn day6_part1_answers() {
    assert_eq!(day6::part1_answers(&String::from("ab ac")), vec!['a','b','c']);
}

#[test]
fn day6_part2_answers() {
    assert_eq!(day6::part2_answers(&String::from("ab ac")), vec!['a']);
}

#[test]
fn day6_part2_count() {
    assert_eq!(day6::part2_answers(&String::from("mxrufizdpe tmowfuynajqlivhgskrcb")).len(), 5);
}
