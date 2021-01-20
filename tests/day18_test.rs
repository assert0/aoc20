use aoc20::days::day18;

#[test]
fn day18_test_part1() {
    assert_eq!(71, day18::evaluate("1 + 2 * 3 + 4 * 5 + 6", day18::Part::Part1));
    assert_eq!(51, day18::evaluate("1 + (2 * 3) + (4 * (5 + 6))", day18::Part::Part1));
    assert_eq!(26, day18::evaluate("2 * 3 + (4 * 5)", day18::Part::Part1));
    assert_eq!(437, day18::evaluate("5 + (8 * 3 + 9 + 3 * 4 * 3)", day18::Part::Part1));
    assert_eq!(12240, day18::evaluate("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", day18::Part::Part1));
    assert_eq!(13632, day18::evaluate("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", day18::Part::Part1));
}

#[test]
fn day18_test_part2() {
    assert_eq!(231, day18::evaluate("1 + 2 * 3 + 4 * 5 + 6", day18::Part::Part2));
    assert_eq!(51, day18::evaluate("1 + (2 * 3) + (4 * (5 + 6))", day18::Part::Part2));
    assert_eq!(46, day18::evaluate("2 * 3 + (4 * 5)", day18::Part::Part2));
    assert_eq!(1445, day18::evaluate("5 + (8 * 3 + 9 + 3 * 4 * 3)", day18::Part::Part2));
    assert_eq!(669060, day18::evaluate("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", day18::Part::Part2));
    assert_eq!(23340, day18::evaluate("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", day18::Part::Part2));
}
