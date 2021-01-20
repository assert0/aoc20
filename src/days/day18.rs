use std::fs;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Part {
    Part1,
    Part2,
}

// Code modified from the 'yard' crate https://docs.rs/crate/yard

/// RPNToken enum define a Operator and Operand variants.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RPNToken {
    Operator(Operator),
    Operand(i64),
}

/// Operator enum define the allowed operations.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Operator {
    PLUS,
    MULTIPLY,
    LPAREN,
    RPAREN,
}

impl Operator {
    /// return the associated value of an Operator variant.
    pub fn value(&self, part: Part) -> u32 {
        match part {
            Part::Part1 => match *self {
                Operator::LPAREN | Operator::RPAREN => 0,
                Operator::PLUS => 1,
                Operator::MULTIPLY=> 1,
            },
            Part::Part2 => match *self {
                Operator::LPAREN | Operator::RPAREN => 0,
                Operator::PLUS => 2,
                Operator::MULTIPLY=> 1,
            },
        }
    }

    /// try to convert a char to an Operator variant.
    // Until std::convert::TryFrom stabilizes
    pub fn try_from_char(c: char) -> Option<Operator> {
        Some(match c {
            '+' => Operator::PLUS,
            '*' => Operator::MULTIPLY,
            '(' => Operator::LPAREN,
            ')' => Operator::RPAREN,
            _ => return None,
        })
    }
}

/// parse try to convert char into RPNToken
/// it strip whitespace and support negative operations.
pub fn parse(code: &str, part: Part) -> Result<Vec<RPNToken>, String> {
    let tokens = code.chars().filter(|c| !c.is_whitespace());
    let mut output: Vec<RPNToken> = Vec::new();
    let mut stack: Vec<Operator> = Vec::new();
    let mut num: String = String::new();
    let mut neg = true;

    for tok in tokens {
        if tok.is_numeric() {
            num.push(tok);
            neg = false;
        } else {
            if tok == '-' && neg {
                num.push('-');
                neg = false;
                continue;
            }
            if !num.is_empty() {
                let rpnt = RPNToken::Operand(num.parse::<i64>().map_err(|err| err.to_string())?);
                output.push(rpnt);
                num.clear();
            }

            match Operator::try_from_char(tok) {
                Some(Operator::LPAREN) => {
                    stack.push(Operator::LPAREN);
                    neg = true;
                },
                Some(Operator::RPAREN) => {
                    while let Some(v) = stack.pop() {
                        if v == Operator::LPAREN {
                            break
                        }
                        assert_ne!(v, Operator::RPAREN);
                        output.push(RPNToken::Operator(v));
                    }
                },
                Some(tokop) => {
                    while {
                        if let Some(&qe) = stack.last() {
                            tokop.value(part) <= qe.value(part)
                        } else {
                            false
                        }
                    } {
                        output.push(RPNToken::Operator(stack.pop().unwrap()));
                    }
                    stack.push(tokop);
                    neg = true;
                },
                None => return Err(format!("Unexpected character: {}", tok)),
            }
        }
    }

    if !num.is_empty() {
        let rpnt = RPNToken::Operand(num.parse::<i64>().map_err(|err| err.to_string())?);
        output.push(rpnt);
    }

    while let Some(v) = stack.pop() {
        output.push(RPNToken::Operator(v));
    }

    Ok(output)
}

pub fn eval(tokens: &[RPNToken]) -> i64 {
    let mut stack: Vec<i64> = Vec::new();
    for t in tokens {
        match *t {
            RPNToken::Operator(Operator::PLUS) => {
                let n1 = stack.pop().expect("Unbalanced addition");
                let n2 = stack.pop().expect("Unbalanced addition");
                stack.push(n2 + n1);
            },
            RPNToken::Operator(Operator::MULTIPLY) => {
                let n1 = stack.pop().expect("Unbalanced multiplication");
                let n2 = stack.pop().expect("Unbalanced multiplication");
                stack.push(n2 * n1);
            },
            RPNToken::Operator(Operator::LPAREN) => panic!("Stray ( in eval"),
            RPNToken::Operator(Operator::RPAREN) => panic!("Stray ) in eval"),
            RPNToken::Operand(v) => stack.push(v),
        }
    }

    *stack.last().unwrap()
}

pub fn evaluate(code: &str, part: Part) -> i64 {
    match parse(code, part) {
        Ok(tokens) => eval(&tokens),
        Err(_e) => 0,
    }
}

pub fn day18(args: &[String]) -> i32 {
    println!("Day 18");
    if args.len() != 1 {
        println!("Missing input file");
        return -1;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let results = contents.lines().map(|l| evaluate(l, Part::Part1)).collect::<Vec<i64>>();
    println!("Part 1: {}", results.iter().sum::<i64>());

    let results = contents.lines().map(|l| evaluate(l, Part::Part2)).collect::<Vec<i64>>();
    println!("Part 2: {}", results.iter().sum::<i64>());

    0
}