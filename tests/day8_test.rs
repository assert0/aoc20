use aoc20::days::day8;

#[test]
fn day8_parse() {
    assert_eq!(day8::Instruction::parse("nop +0"),
        day8::Instruction::new(String::from("nop"), 0)
    );
    assert_eq!(day8::Instruction::parse("acc -99"),
        day8::Instruction::new(String::from("acc"), -99)
    );
    assert_eq!(day8::Instruction::parse("jmp +4"),
        day8::Instruction::new(String::from("jmp"), 4)
    );
}
