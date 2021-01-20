use std::fs;
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    name: String,
    lower: (usize, usize),
    upper: (usize, usize),
}

impl Field {

    pub fn new(name: String, lower: (usize, usize), upper: (usize, usize)) -> Field {
        Field { name, lower, upper }
    }

    pub fn parse(line: &str) -> Field {
        // ex "departure location: 36-363 or 377-962"
        lazy_static! {
            static ref FIELD: Regex = Regex::new(r"^([a-z\s]+):\s+(\d+)-(\d+)\s+or\s(\d+)-(\d+)$").unwrap();
        }
        assert!(FIELD.is_match(&line));
        let caps = FIELD.captures(line).unwrap();
        let name = caps.get(1).unwrap().as_str().to_string();
        let lower = (caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                     caps.get(3).unwrap().as_str().parse::<usize>().unwrap());
        let upper = (caps.get(4).unwrap().as_str().parse::<usize>().unwrap(),
                     caps.get(5).unwrap().as_str().parse::<usize>().unwrap());
        Field { name, lower, upper }
    }

    fn check_range(range: (usize, usize), number: usize) -> bool {
        (range.0..range.1+1).contains(&number)
    }

    pub fn is_valid(&self, ticket: usize) -> bool {
        if Field::check_range(self.lower, ticket) {
            return true;
        }
        Field::check_range(self.upper, ticket) 
    }

}

pub fn parse_tickets(contents: &str) -> Vec<Vec<usize>> {
    let mut tickets = vec![];
    for (i, l) in contents.lines().enumerate() {
        if i == 0 { continue; } // skip the first line "your ticket:"
        tickets.push(l.split(",").map(|i| i.parse::<usize>().unwrap()).collect::<Vec<usize>>());
    }
    tickets
}

pub fn invalid_tickets(fields: &Vec<Field>, nearby_tickets: &Vec<Vec<usize>> ) -> Vec<usize> {
    let mut invalid: Vec<usize> = vec![];
    for t in nearby_tickets.into_iter().flatten() {
        let mut valid = false;
        for f in fields {
            if f.is_valid(*t) {
                valid = true;
                break;
            }
        }
        if !valid {
            invalid.push(*t);
        }
    }
    invalid
}

pub fn identify_fields(fields: &Vec<Field>, tickets: &Vec<Vec<usize>>) -> Vec<Option<String>> {
    let invalid = invalid_tickets(&fields, &tickets);
    
    let mut found: Vec<Vec<String>> = vec![];
    for i in 0..fields.len() {
        found.push(vec![]);
        // the number of fields should equal the number of tickets in each line
        assert_eq!(fields.len(), tickets[i].len());
        // get all the tickets in the current column removing invalid
        let column = tickets.into_iter().map(|t| t[i])
                                        .filter(|t| !invalid.contains(t))
                                        .collect::<Vec<usize>>();
        let mut valid = false;
        for f in fields {
            if column.iter().all(|t| f.is_valid(*t)) {
                found[i].push(f.name.clone());
                valid = true;
            }
        }
        assert!(valid);
    }

    // locate the correct field by elimination
    let mut result: Vec<Option<String>> = vec![None; found.len()];
    loop {
        // exit when all are found
        if result.iter().all(|r| r.is_some()) {
            break;
        }
        let mut name: Option<String> = None;
        for (i, f) in found.iter().enumerate() {
            if f.len() == 1 {
                name = Some(f[0].clone());
                result[i] = Some(f[0].clone());
            }
        }
        assert!(name.is_some());
        // remove found name
        for f in &mut found {
            let pos = f.iter().position(|n| Some(n) == name.as_ref());
            if pos.is_some() {
                f.remove(pos.unwrap());
            }
        }
    }
    result
}

pub fn day16(args: &[String]) -> i32 {
    println!("Day 16");
    if args.len() != 1 {
        println!("Missing input file");
        return -1;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let groups: Vec<&str> = contents.split("\n\n").collect();
    assert_eq!(groups.len(), 3);

    let fields: Vec<Field> = groups[0].lines().map(|l| Field::parse(l)).collect();
    //println!("{:?}", fields);

    let your_ticket = &parse_tickets(groups[1])[0]; // should only contaian 1 ticket line
    //println!("{:?}", your_tickets);

    let nearby_tickets = parse_tickets(groups[2]);
    //println!("{:?}", nearby_tickets);

    let invalid = invalid_tickets(&fields, &nearby_tickets);
    println!("Part 1: {:?}", invalid.iter().sum::<usize>());

    let mut alltickets = nearby_tickets.clone();
    alltickets.push(your_ticket.clone());

    let found = identify_fields(&fields, &alltickets);
     // get the values for "your ticket" where the field starts with "departure"
    let values = found.into_iter()
                         .enumerate().filter(|(_i, f)| f.as_ref().unwrap().find("departure") == Some(0))
                         .map(|(i, _f)| your_ticket[i])
                         .collect::<Vec<usize>>();
    
    println!("Part 2: {:?}", values.iter().product::<usize>());

   0
}