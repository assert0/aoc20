use aoc20::days::day13;

fn run(result: i64, modulii: Vec<i64>) {
    let l = modulii.len() as i64;
    assert_eq!(result, day13::calc_part2(&modulii, l));
}

#[test]
fn day13_example1() {
    run(3417, vec![17, 1, 13, 19]);
}

#[test]
fn day13_example2() {
    run(754018, vec![67, 7, 59, 61]);
}

#[test]
fn day13_example3() {
    run(779210, vec![67, 1, 7, 59, 61]);
}

#[test]
fn day13_example4() {
    run(1261476, vec![67, 7, 1, 59, 61]);
}

#[test]
fn day13_example5() {
    run(1202161486, vec![1789, 37, 47, 1889]);
}

