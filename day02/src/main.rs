use regex::Regex;
use std::io::{self, BufRead};

fn main() {
    let mut valid_count = 0;
    let mut valid_count_new = 0;
    for line in io::stdin().lock().lines() {
        let policy = parse_line(&line.unwrap()).unwrap();

        let count = policy.password.matches(&policy.character).count();
        if policy.num1 <= count && policy.num2 >= count {
            valid_count += 1;
        }

        let mut matching = 0;
        if policy.password.chars().collect::<Vec<char>>()[policy.num1 - 1] == policy.character.chars().collect::<Vec<char>>()[0] {
            matching += 1;
        }
        if policy.password.chars().collect::<Vec<char>>()[policy.num2 - 1] == policy.character.chars().collect::<Vec<char>>()[0] {
            matching += 1;
        }
        if matching == 1 {
            valid_count_new += 1;
        }
    }

    println!("part 1: {}", valid_count);
    println!("part 2: {}", valid_count_new);
}

fn parse_line(s: &str) -> Option<PwPolicy> {
    let re = Regex::new(r"^([0-9]+)-([0-9]+) ([a-z]): (\w+)$").unwrap();
    let captures = re.captures(s)?;
    let num1: usize = captures.get(1)?.as_str().parse().ok()?;
    let num2: usize = captures.get(2)?.as_str().parse().ok()?;
    let character: String = captures.get(3)?.as_str().to_string();
    let password: String = captures.get(4)?.as_str().to_string();

    Some(PwPolicy {
        num1,
        num2,
        character,
        password,
    })
}

struct PwPolicy {
    num1: usize,
    num2: usize,
    character: String,
    password: String,
}
