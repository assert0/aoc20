use aoc20::days::day13;

#[test]
fn day13_example1() {
    assert_eq!(3417, day13::calc_part2(&vec![17, 1, 13, 19]));
}

#[test]
fn day13_example2() {
    assert_eq!(754018, day13::calc_part2(&vec![67, 7, 59, 61]));
}

#[test]
fn day13_example3() {
    assert_eq!(779210, day13::calc_part2(&vec![67, 1, 7, 59, 61]));
}

#[test]
fn day13_example4() {
    assert_eq!(1261476, day13::calc_part2(&vec![67, 7, 1, 59, 61]));
}

#[test]
fn day13_example5() {
    assert_eq!(1202161486, day13::calc_part2(&vec![1789, 37, 47, 1889]));
}

