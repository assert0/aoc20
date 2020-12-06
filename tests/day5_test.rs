use aoc20::days::day5;

// BFFFBBFRRR: row 70, column 7, seat ID 567.
// FFFBBBFRRR: row 14, column 7, seat ID 119.
// BBFFBBFRLL: row 102, column 4, seat ID 820.

#[test]
fn day5_parse_id() {
    let seat = day5::Seat::parse(&String::from("BFFFBBFRRR"));
    assert_eq!(seat.id(), 567);
    let seat = day5::Seat::parse(&String::from("FFFBBBFRRR"));
    assert_eq!(seat.id(), 119);
    let seat = day5::Seat::parse(&String::from("BBFFBBFRLL"));
    assert_eq!(seat.id(), 820);
}

