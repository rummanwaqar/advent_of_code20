use std::fs::File;
use std::io::Read;

fn main() {
    let input = read_input("input/input.txt").unwrap();
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

fn part1(input: &Vec<String>) -> u16 {
    let mut max = 0;
    for x in input {
        let row = binary_partitioning(&x[..7], 127) as u16;
        let col = binary_partitioning(&x[7..], 7) as u16;
        let val :u16 = row * 8 + col;
        if val > max {
            max = val;
        }
    }
    max
}

fn part2(input: &Vec<String>) -> u16 {
    let mut seats = vec![];
    for x in input {
        let row = binary_partitioning(&x[..7], 127) as u16;
        let col = binary_partitioning(&x[7..], 7) as u16;
        let seat :u16 = row * 8 + col;
        seats.push(seat);
    }

    seats.sort();
    let mut i = 0;
    for x in 1..seats.len() {
        if seats[x] != seats[i] + 1 {
            return seats[x] - 1;
        }
        i += 1;
    }
    seats[0]
}

fn binary_partitioning(input: &str, end: u8) -> u8 {
    let mut start: u8 = 0;
    let mut end = end;
    for c in input.chars() {
        if start == end {
            return start;
        }
        let middle = (end - start) / 2 + 1;
        if c == 'F' || c == 'L' {
            end -= middle;
        } else if c == 'B' || c == 'R' {
            start += middle;
        }
    }
    start
}

fn read_input(file_path: &str) -> std::io::Result<Vec<String>>{
    let mut file = File::open(file_path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let v: Vec<String> = data.split('\n').map(|x| String::from(x)).collect();
    Ok(v)
}