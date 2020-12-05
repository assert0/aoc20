use std::fs;
use regex::{Regex, Captures};
use std::fmt;

pub fn get_passports_lines(filename: &String) -> Vec<String> {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    // normalize the input by replacing only the first occurance of a endline with a space
    let re = Regex::new(r"([^\n])\n").unwrap();
    re.replace_all(&contents, |caps: &Captures| {
            format!("{} ", &caps[1])
        })
        .lines()
        .map(|l|l.trim().to_string())
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyValue {
    key: String,
    value: String,
}

impl KeyValue {

    pub fn parse(text: &String) -> KeyValue {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"([a-z]*):([^:]*)").unwrap();
        }
        // Split out key value (ex: "iyr:2013")
        let caps = RE.captures(&text).unwrap();
        let key = caps.get(1).unwrap().as_str().to_string();
        let value = caps.get(2).unwrap().as_str().to_string();
        KeyValue { key, value }
    }

    pub fn is_valid(&self) -> bool {
        lazy_static! {
            static ref HGT: Regex = Regex::new(r"^([\d]+)(cm|in)$").unwrap();
            static ref HCL: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
            static ref ECL: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
            static ref PID: Regex = Regex::new(r"^[\d]{9}$").unwrap();
        }
        match self.key.as_str() {
            "byr" => {
                match self.value.parse::<usize>() {
                    Ok(n) => (n >= 1920 && n <= 2002),
                    Err(_) => false
                }
            },
            "iyr" => {
                match self.value.parse::<usize>() {
                    Ok(n) => (n >= 2010 && n <= 2020),
                    Err(_) => false
                }
            },
            "eyr" => {
                match self.value.parse::<usize>() {
                    Ok(n) => (n >= 2020 && n <= 2030),
                    Err(_) => false
                }
            },
            "hgt" => {
                let caps = HGT.captures(&self.value);
                match caps {
                    Some(s) => {
                        let value = s.get(1).unwrap().as_str();
                        let units = s.get(2).unwrap().as_str();
                        KeyValue::is_valid_height(value, units)
                    },
                    None => false
                }
            },
            "hcl" => {
                HCL.is_match(&self.value)
            },
            "ecl" => {
                ECL.is_match(&self.value)
            },
            "pid" => {
                PID.is_match(&self.value)
            },
            "cid" => true,
            _ => false,
        }
    }

    // validate height
    fn is_valid_height(value: &str, units: &str) -> bool {
        match value.parse::<usize>() {
            Ok(n) => {
                match units {
                    "cm" => n >= 150 && n <= 193,
                    "in" => n >= 59 && n <= 76,
                    _ => false,
                }
            },
            Err(_) => false
        }
    }

}
impl fmt::Display for KeyValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.key, self.value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Passport {
    entries: Vec<KeyValue>
}

impl Passport {

    pub fn parse(line: &String) -> Passport {
        Passport {
            entries: line.split(' ').map(|kv| KeyValue::parse(&kv.to_string())).collect()
        }
    }

    pub fn keys(&self) -> Vec<String> {
        self.entries.iter().map(|kv| kv.key.clone()).collect()
    }

    pub fn north_pole_keys() -> Vec<&'static str> {
        vec![
            "byr", // (Birth Year)
            "iyr", // (Issue Year)
            "eyr", // (Expiration Year)
            "hgt", // (Height)
            "hcl", // (Hair Color)
            "ecl", // (Eye Color)
            "pid", // (Passport ID)
            // cid not required
        ]
    }

    pub fn north_pole_keys_present(&self) -> bool {
        let keys = self.keys();
        for np in Passport::north_pole_keys() {
            if !keys.contains(&np.to_string()) {
                return false;
            }
        }
        true
    }

    pub fn is_valid(&self) -> bool {
        if !self.north_pole_keys_present() {
            return false;
        }
        for e in &self.entries {
            if !e.is_valid() {
                return false;
            }
        }
        true
    }
}


pub fn day4(args: &[String]) -> i32 {
    println!("Day 4");
    if args.len() != 1 {
        println!("Missing input file");
        return -1;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    
    let lines = get_passports_lines(filename);
    let count = lines.iter()
                    .filter(|p| Passport::parse(&p).north_pole_keys_present())
                    .count();
    println!("Part 1: {}", count);

    let count = lines.iter()
                    .filter(|p| Passport::parse(&p).is_valid())
                    .count();
    println!("Part 2: {}", count);

    0
}
