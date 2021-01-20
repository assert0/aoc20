use aoc20::days::day14;

#[test]
fn day14_parse_mask() {
    let m = day14::Mask::parse("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").unwrap();
    assert_eq!(m, day14::Mask::new(0x40, 0xF_FFFF_FFBD));
}

#[test]
fn day14_parse_mem() {
    let m = day14::Mem::parse("mem[36932] = 186083").unwrap();
    assert_eq!(m, day14::Mem::new(36932, 186083));
}

#[test]
fn day14_parse_mem_mask() {
    let m = day14::MemMask::parse("mask = 000000000000000000000000000000X1001X").unwrap();
    assert_eq!(m, day14::MemMask::new(0x12, 0xF_FFFF_FFDE, vec![0, 5]));
}

#[test]
fn day14_parse_mem_mask_iter() {
    let m = day14::MemMask::parse("mask = 000000000000000000000000000000X1001X").unwrap();
    assert_eq!(m.collect::<Vec<u64>>(), vec![0, 1, 0x20, 0x21]);
}

#[test]
fn day14_parse_mem_mask_iter2() {
    let m = day14::MemMask::parse("mask = 00000000000000000000000000000000X0XX").unwrap();
    assert_eq!(m.collect::<Vec<u64>>(), vec![0, 1, 2, 3, 8, 9, 10, 11]);
}

#[test]
fn day14_parse_mem_mask_iter3() {
    let m = day14::MemMask::parse("mask = 000000000000000000000000000000X1001X").unwrap();
    let value = m.condition(42);
    assert_eq!(m.map(|fl| value | fl).collect::<Vec<u64>>(), vec![26, 27, 58, 59]);
}


