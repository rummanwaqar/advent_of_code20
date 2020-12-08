use std::fs::File;
use std::io::Read;
use std::str::FromStr;

fn main() {
    let input = read_input("input/input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(program: &Vec<Instruction>) -> i32 {
    let program_output = run_program(&program);
    program_output.accumulator
}

fn part2(program: &Vec<Instruction>) -> i32 {
    let mut current_mod_index = 0;
    loop {
        // modify the program
        let mut mutable_program = (*program).clone();
        while mutable_program[current_mod_index].command == Operation::Acc {
            current_mod_index += 1;
        }
        if current_mod_index < program.len() {
            if mutable_program[current_mod_index].command == Operation::Nop {
                mutable_program[current_mod_index].command = Operation::Jmp;
            } else {
                mutable_program[current_mod_index].command = Operation::Nop;
            }
        } else {
            panic!("No valid solution found");
        }
        current_mod_index += 1;

        // test new program
        let program_output = run_program(&mutable_program);
        if !program_output.loop_detected {
            return program_output.accumulator;
        }
    }
}

fn run_program(program: &Vec<Instruction>) -> ProgramOutput {
    let mut repeats = vec![false; program.len()];
    let mut accumulator = 0;
    let mut program_counter: i32 = 0;

    while program_counter < program.len() as i32 && !repeats[program_counter as usize] {
        let instruction = &program[program_counter as usize];
        repeats[program_counter as usize] = true;
        match instruction.command {
            Operation::Acc => {
                accumulator += instruction.argument;
                program_counter += 1;
            }
            Operation::Jmp => {
                program_counter += instruction.argument;
            }
            Operation::Nop => {
                program_counter += 1;
            }
        }
    }
    ProgramOutput {
        accumulator,
        loop_detected: program_counter < program.len() as i32,
    }
}

struct ProgramOutput {
    accumulator: i32,
    loop_detected: bool,
}

fn read_input(file_path: &str) -> std::io::Result<Vec<Instruction>> {
    let mut file = File::open(file_path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let program = data
        .split('\n')
        .map(|x| Instruction::from_str(x).unwrap())
        .collect();
    Ok(program)
}

#[derive(Debug, Clone, PartialEq)]
enum Operation {
    Nop,
    Acc,
    Jmp,
}

#[derive(Debug, Clone)]
struct Instruction {
    command: Operation,
    argument: i32,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut command_argument_split = s.split(' ');
        let command = match command_argument_split.next().unwrap() {
            "acc" => Operation::Acc,
            "jmp" => Operation::Jmp,
            _ => Operation::Nop,
        };
        let argument = command_argument_split
            .next()
            .unwrap()
            .parse::<i32>()
            .unwrap();
        Ok(Self { command, argument })
    }
}
