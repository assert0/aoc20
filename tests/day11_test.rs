
use std::fs;
use aoc20::days::day11;

fn get_layout(filename: &str, part: day11::Part) -> day11::SeatLayout {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    
    day11::SeatLayout::new(&contents, part)
}

#[test]
fn day11_parse_seat() {
    let layout = get_layout("data/day11example.txt", day11::Part::Part1);
    assert!(layout.is_state(0, 0, day11::State::Open));
    assert!(layout.is_state(0, 1, day11::State::Floor));
    assert!(!layout.is_state(0, 1, day11::State::Occupied));
}

#[test]
fn day11_update() {
    let layout = get_layout("data/day11example.txt", day11::Part::Part1);
    assert_eq!(layout.next_state(0, 2), day11::State::Occupied);
}

#[test]
#[should_panic]
fn day11_adjacent_seat_count() {
    let layout = get_layout("data/day11example.txt", day11::Part::Part1);
    assert_eq!(layout.adjacent_occupied_count(0, 0), 2);
    assert_eq!(layout.adjacent_occupied_count(1, 4), 5);
    assert_eq!(layout.adjacent_occupied_count(9, 9), 2);
    assert_eq!(layout.adjacent_occupied_count(4, 4), 7);
    layout.adjacent_occupied_count(10, 10); // should panic, invalid seat
}

#[test]
fn day11_part2_occupied1() {
    let layout = get_layout("data/day11example2.txt", day11::Part::Part2);
    assert_eq!(layout.adjacent_occupied_count(4, 3), 8);
}

#[test]
fn day11_part2_occupied2() {
    let layout = get_layout("data/day11example3.txt", day11::Part::Part1);
    assert_eq!(layout.adjacent_occupied_count(1, 1), 0);
}

#[test]
fn day11_part2_occupied3() {
    let layout = get_layout("data/day11example4.txt", day11::Part::Part1);
    assert_eq!(layout.adjacent_occupied_count(3, 3), 0);
}