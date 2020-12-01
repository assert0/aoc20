use std::fs;

pub fn day1(args: &[String]) -> i32 {
    println!("Day 1");
    if args.len() != 1 {
        println!("Missing input file");
        return -1;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let entries: Vec<i32> = contents.lines().map(|l| l.parse::<i32>().unwrap()).collect();
    for i in 0..entries.len()-1 {
        for j in i+1..entries.len() {
            if entries[i] + entries[j] == 2020 {
                println!("Part 1: {:}", entries[i] * entries[j]);
            }
        }
    }

    for i in 0..entries.len()-1 {
        for j in i+1..entries.len() {
            for k in j+1..entries.len() {
                if entries[i] + entries[j] + entries[k] == 2020 {
                    println!("Part 2: {:}", entries[i] * entries[j] * entries[k]);
                }
            }
        }
    }
    //println!("Part 2: sum {:}", part2(&masses));
    return 0;
}

