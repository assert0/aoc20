use std::fs;
use std::cmp::max;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    name: String,
    cards: Vec<u8>
}

impl Player {

    pub fn new(name: String, cards: Vec<u8>) -> Player {
        Player { name, cards }
    }

    pub fn parse(lines: &str) -> Player {
        if let Some((first, rows)) = lines.lines().collect::<Vec<&str>>().split_first() {
            let name = first.to_string();
            let cards = rows.iter().map(|l| l.parse::<u8>().unwrap()).collect();
            return Player { name, cards };
        }
        unreachable!("Failed to parse");
    }

    pub fn has_cards(&self) -> bool {
        !self.cards.is_empty()
    }

    pub fn play_card(&mut self) -> u8 {
        self.cards.remove(0)
    }

    pub fn add_cards(&mut self, won: u8, lost: u8) {
        self.cards.push(won);
        self.cards.push(lost);
    }

    pub fn score(&self) -> usize {
        self.cards.iter().rev().enumerate().map(|(i, &c)| (i+1) * c as usize).sum()
    }

}

pub fn play_round(player1: &mut Player, player2: &mut Player) {
    let (card1, card2) = (player1.play_card(), player2.play_card());
    //println!("{} {}", card1, card2);
    if card1 > card2 {
        player1.add_cards(card1, card2);
    } else if card2 > card1 {
        player2.add_cards(card2, card1);
    } else {
        unreachable!("Invalid round");
    }
}

pub fn part1(mut player1: Player, mut player2: Player) -> usize {
    loop {
        if !player1.has_cards() || !player2.has_cards() {
            break;
        }
        play_round(&mut player1, &mut player2);
    }
    max(player1.score(), player2.score())
}

pub fn day22(args: &[String]) -> i32 {
    println!("Day 22");
    if args.len() != 1 {
        println!("Missing input file");
        return -1;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut players: Vec<Player> = contents.split("\n\n")
                                   .map(|l| Player::parse(l))
                                   .collect();
    let (player1, player2) = (players.remove(0), players.remove(0)); 
    //println!("{:?} {:?}", player1, player2);
    
    println!("Part 1: {}", part1(player1.clone(), player2.clone()));
    0
}