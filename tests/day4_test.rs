use aoc20::days::day4;

#[test]
fn day4_get_passports() {
    let passports = day4::get_passport_lines(&String::from("data/day4example.txt"));
    assert_eq!(passports.len(), 4);
}

#[test]
fn day4_parse() {
    let tv = day4::KeyValue::parse(&String::from("eyr:2020"));
    assert_eq!("eyr:2020", tv.to_string());
    let tv2 = day4::KeyValue::parse(&String::from("hcl:#fffffd"));
    assert_eq!("hcl:#fffffd", tv2.to_string());
}

#[test]
fn day4_keys_present() {
    let p1 = day4::Passport::parse(&String::from("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm"));
    assert_eq!(p1.north_pole_keys_present(), true);
    let p2 = day4::Passport::parse(&String::from("iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884 hcl:#cfa07d byr:1929"));
    assert_eq!(p2.north_pole_keys_present(), false);
}

#[test]
fn day4_byr() {
    let byr = day4::KeyValue::parse(&String::from("byr:2002"));
    assert_eq!(byr.is_valid(), true);
    let byr = day4::KeyValue::parse(&String::from("byr:2003"));
    assert_eq!(byr.is_valid(), false);
}

#[test]
fn day4_hgt() {
    let hgt = day4::KeyValue::parse(&String::from("hgt:60in"));
    assert_eq!(hgt.is_valid(), true);
    let hgt = day4::KeyValue::parse(&String::from("hgt:190cm"));
    assert_eq!(hgt.is_valid(), true);
    let hgt = day4::KeyValue::parse(&String::from("hgt:190in"));
    assert_eq!(hgt.is_valid(), false);
    let hgt = day4::KeyValue::parse(&String::from("hgt:190"));
    assert_eq!(hgt.is_valid(), false);
}

#[test]
fn day4_hcl() {
    let hgt = day4::KeyValue::parse(&String::from("hcl:#123abc"));
    assert_eq!(hgt.is_valid(), true);
    let hgt = day4::KeyValue::parse(&String::from("hcl:#123abz"));
    assert_eq!(hgt.is_valid(), false);
    let hgt = day4::KeyValue::parse(&String::from("hcl:123abc"));
    assert_eq!(hgt.is_valid(), false);
}

#[test]
fn day4_ecl() {
    let hgt = day4::KeyValue::parse(&String::from("ecl:brn"));
    assert_eq!(hgt.is_valid(), true);
    let hgt = day4::KeyValue::parse(&String::from("ecl:wat"));
    assert_eq!(hgt.is_valid(), false);
}

#[test]
fn day4_pid() {
    let hgt = day4::KeyValue::parse(&String::from("pid:000000001"));
    assert_eq!(hgt.is_valid(), true);
    let hgt = day4::KeyValue::parse(&String::from("pid:0123456789"));
    assert_eq!(hgt.is_valid(), false);
}
