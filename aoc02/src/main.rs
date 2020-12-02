use std::fs::File;
use std::io::Read;
use std::str::FromStr;

fn main() {
    let input = read_input("input/input.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<Policy>) {
    let mut correct = 0;
    for policy in input {
        let mut letter_count = 0;
        for c in policy.password.chars() {
            if c == policy.character {
                letter_count += 1;
            }
        }
        if letter_count >= policy.min && letter_count <= policy.max {
            correct += 1;
        }
    }
    println!("part 1: {}", correct);
}

fn part2(input: &Vec<Policy>) {
    let mut correct = 0;
    for policy in input {
        // positions are 1-indexed
        let pos1 = policy.password.chars().nth((policy.min - 1) as usize).unwrap();
        let pos2 = policy.password.chars().nth((policy.max - 1) as usize).unwrap();
        // only one of the pos1 and pos2 should contain the character
        if (pos1 == policy.character) ^ (pos2 == policy.character) {
            correct += 1;
        }
    }
    println!("part 2: {}", correct);
}

#[derive(Debug)]
struct Policy {
    min: u8,
    max: u8,
    character: char,
    password: String
}

impl FromStr for Policy {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let policy_password_split: Vec<&str> = s.split(": ").collect();
        if policy_password_split.len() == 2 {
            let range_char_split: Vec<&str> = policy_password_split[0].split(" ").collect();
            if range_char_split.len() == 2 {
                let min_max_split: Vec<&str> = range_char_split[0].split("-").collect();
                if min_max_split.len() == 2 {
                    return Ok(Policy {
                        min: min_max_split[0].parse().unwrap(),
                        max: min_max_split[1].parse().unwrap(),
                        character: range_char_split[1].parse().unwrap(),
                        password: String::from(policy_password_split[1])
                    });
                }
            }
        }
        Err(())
    }
}

fn read_input(file_name: &str) -> std::io::Result<Vec<Policy>> {
    let mut file = File::open(file_name)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let v: Vec<Policy> = data.split('\n').map(|x| x.parse().unwrap()).collect();
    Ok(v)
}