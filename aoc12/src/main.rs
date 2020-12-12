use std::fs::File;
use std::io::Read;
use std::str::FromStr;

fn main() {
    let input = read_input("input/input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Vec<Command>) -> u64 {
    let mut ship = [0., 0.];
    let mut angle = 0.;
    for command in input {
        match command.action {
            Action::N => ship[1] += command.argument,
            Action::S => ship[1] -= command.argument,
            Action::E => ship[0] += command.argument,
            Action::W => ship[0] -= command.argument,
            Action::L => angle += command.argument,
            Action::R => angle -= command.argument,
            Action::F => {
                ship[0] += command.argument * angle.to_radians().cos();
                ship[1] += command.argument * angle.to_radians().sin();
            }
        }
    }
    (ship[0].abs() + ship[1].abs()) as u64
}

fn part2(input: &Vec<Command>) -> u64 {
    let mut ship = [0., 0.];
    let mut waypoint = [10., 1.];
    for command in input {
        match command.action {
            Action::N => waypoint[1] += command.argument,
            Action::S => waypoint[1] -= command.argument,
            Action::E => waypoint[0] += command.argument,
            Action::W => waypoint[0] -= command.argument,
            Action::L => waypoint = rotate_point(&waypoint, command.argument),
            Action::R => waypoint = rotate_point(&waypoint, -command.argument),
            Action::F => {
                ship[0] += waypoint[0] * command.argument;
                ship[1] += waypoint[1] * command.argument;
            }
        }
    }
    (ship[0].abs() + ship[1].abs()) as u64
}

fn rotate_point(point: &[f64; 2], angle: f64) -> [f64; 2] {
    let rad = angle.to_radians();
    [
        rad.cos() * point[0] - rad.sin() * point[1],
        rad.sin() * point[0] + rad.cos() * point[1],
    ]
}

#[derive(Debug)]
enum Action {
    N,
    S,
    E,
    W,
    L,
    R,
    F,
}

#[derive(Debug)]
struct Command {
    action: Action,
    argument: f64,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let action = match s.chars().nth(0).unwrap() {
            'F' => Some(Action::F),
            'N' => Some(Action::N),
            'S' => Some(Action::S),
            'E' => Some(Action::E),
            'W' => Some(Action::W),
            'L' => Some(Action::L),
            'R' => Some(Action::R),
            _ => None,
        }
        .unwrap();
        let argument: f64 = s.chars().skip(1).collect::<String>().parse().unwrap();
        Ok(Self { action, argument })
    }
}

fn read_input(file_path: &str) -> std::io::Result<Vec<Command>> {
    let mut file = File::open(file_path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    Ok(data
        .split('\n')
        .map(|x| x.parse::<Command>().unwrap())
        .collect())
}
