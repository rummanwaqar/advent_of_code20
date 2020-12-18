use crate::Instruction::{Mask, Write};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

fn main() {
    let program = read_input("input/input.txt").unwrap();
    println!("Part 1: {}", part1(&program));
    println!("Part 2: {}", part2(&program));
}

fn part1(program: &Vec<Instruction>) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut zero_mask = 0;
    let mut one_mask = 0;
    for instruction in program {
        match instruction {
            Mask(mask) => {
                zero_mask = 0;
                one_mask = 0;
                for (i, c) in mask.chars().rev().enumerate() {
                    match c {
                        '0' => {
                            zero_mask ^= 1 << i;
                        }
                        '1' => {
                            one_mask ^= 1 << i;
                        }
                        _ => {}
                    }
                }
            }
            Write(write_command) => {
                let result = (write_command.data & !zero_mask) | one_mask;
                memory.insert(write_command.address, result);
            }
        }
    }
    memory.values().sum()
}

fn part2(program: &Vec<Instruction>) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut current_mask = "";
    for instruction in program {
        match instruction {
            Mask(mask) => {
                current_mask = mask;
            }
            Write(write_command) => {
                let new_addresses = generate_address_with_mask(current_mask, write_command.address);
                for address in new_addresses {
                    memory.insert(address, write_command.data);
                }
            }
        }
    }
    memory.values().sum()
}

fn generate_address_with_mask(mask: &str, initial_address: u64) -> Vec<u64> {
    let mut floating: Vec<u64> = Vec::new();
    let mut new_address = 0;
    for (index, bit) in mask.chars().rev().enumerate() {
        if bit == 'X' {
            floating.push(index as u64);
        } else if bit == '1' {
            new_address += 1 << index;
        } else if bit == '0' {
            new_address += initial_address & (1 << index);
        }
    }
    address_from_floating(new_address, floating.as_slice())
}

fn address_from_floating(address: u64, floating: &[u64]) -> Vec<u64> {
    if floating.len() == 0 {
        return vec![address];
    }
    let (first, rest) = floating.split_first().unwrap();
    let mut result = address_from_floating(address, rest);
    result.append(&mut address_from_floating(address + (1 << first), rest));
    result
}

#[derive(Debug)]
struct WriteCommand {
    address: u64,
    data: u64,
}

#[derive(Debug)]
enum Instruction {
    Mask(String),
    Write(WriteCommand),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" = ");
        return if s.contains("mask") {
            Ok(Self::Mask(split.nth(1).unwrap().to_string()))
        } else {
            let address: u64 = split
                .next()
                .unwrap()
                .split('[')
                .nth(1)
                .unwrap()
                .split(']')
                .next()
                .unwrap()
                .parse()
                .unwrap();
            let data: u64 = split.next().unwrap().parse().unwrap();
            Ok(Self::Write(WriteCommand { address, data }))
        };
    }
}

fn read_input(file_path: &str) -> std::io::Result<Vec<Instruction>> {
    let mut file = File::open(file_path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let mut program: Vec<Instruction> = Vec::new();
    for line in data.split('\n') {
        program.push(line.parse().unwrap());
    }
    Ok(program)
}
