
use std::fs;
use aoc20::days::day11;

fn get_layout(filename: &str) -> day11::SeatLayout {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    
    day11::SeatLayout::new(&contents)
}

#[test]
fn day11_parse_seat() {
    let layout = get_layout("data/day11example.txt");
    assert!(layout.is_state(0, 0, day11::State::Open));
    assert!(layout.is_state(0, 1, day11::State::Floor));
    assert!(!layout.is_state(0, 1, day11::State::Occupied));
}

#[test]
fn day11_update() {
    let layout = get_layout("data/day11example.txt");
    assert_eq!(layout.next_state_part1(0, 2), day11::State::Occupied);
}

#[test]
#[should_panic]
fn day11_adjacent_seat_count() {
    let layout = get_layout("data/day11example.txt");
    assert_eq!(day11::SeatLayout::state_count(&layout.adjacent(0, 0, 1), day11::State::Open), 2);
    assert_eq!(day11::SeatLayout::state_count(&layout.adjacent(1, 4, 1), day11::State::Open), 5);
    assert_eq!(day11::SeatLayout::state_count(&layout.adjacent(9, 9, 1), day11::State::Open), 2);
    assert_eq!(day11::SeatLayout::state_count(&layout.adjacent(4, 4, 1), day11::State::Open), 7);
    layout.adjacent(10, 10, 1); // should panic, invalid seat
}

#[test]
fn day11_part2_occupied1() {
    let layout = get_layout("data/day11example2.txt");
    assert_eq!(layout.occupied_count_part2(4, 3), 8);
}

#[test]
fn day11_part2_occupied2() {
    let layout = get_layout("data/day11example3.txt");
    assert_eq!(layout.occupied_count_part2(1, 1), 0);
}

#[test]
fn day11_part2_occupied3() {
    let layout = get_layout("data/day11example4.txt");
    assert_eq!(layout.occupied_count_part2(3, 3), 0);
}