use aoc20::days::day12;

#[test]
fn day12_parse() {
    let i = day12::Instruction::parse("F10");
    assert_eq!(i, day12::Instruction::new('F', 10));
}

#[test]
fn day12_parse_turn() {
    let i = day12::Instruction::parse("R90");
    assert_eq!(i, day12::Instruction::new('R', 90));
}

