use aoc20::days::day15;

#[test]
fn day15_test1() {
    let g = day15::MemoryGame::new(&vec![0,3,6]);
    assert_eq!(436, g.skip(2016).next().unwrap());
}

#[test]
fn day15_test2() {
    let g = day15::MemoryGame::new(&vec![1,3,2]);
    assert_eq!(1, g.skip(2016).next().unwrap());
}

#[test]
fn day15_test3() {
    let g = day15::MemoryGame::new(&vec![2,1,3]);
    assert_eq!(10, g.skip(2016).next().unwrap());
}

#[test]
fn day15_test4() {
    let g = day15::MemoryGame::new(&vec![1,2,3]);
    assert_eq!(27, g.skip(2016).next().unwrap());
}

#[test]
fn day15_test5() {
    let g = day15::MemoryGame::new(&vec![2,3,1]);
    assert_eq!(78, g.skip(2016).next().unwrap());
}

#[test]
fn day15_test6() {
    let g = day15::MemoryGame::new(&vec![3,2,1]);
    assert_eq!(438, g.skip(2016).next().unwrap());
}

#[test]
fn day15_test7() {
    let g = day15::MemoryGame::new(&vec![3,1,2]);
    assert_eq!(1836, g.skip(2016).next().unwrap());
}

// #[test]
// fn day15_part2_test1() {
//     let g = day15::MemoryGame::new(&vec![0,3,6]);
//     assert_eq!(175594, g.skip(30000000-4).next().unwrap());
// }