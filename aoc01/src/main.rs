use std::fs::File;
use std::io::Read;
use std::collections::HashSet;
use std::iter::FromIterator;


fn main() {
    let input = read_input().unwrap();

    part1(&input);
    part2(&input);
}

// two sum problem
fn part1(input: &Vec<u16>) {
    let set: HashSet<u16> = HashSet::from_iter(input.clone());
    // search through hash set for a complement in main set
    for x in &set {
        let difference = 2020 - x;
        if set.get(&difference).is_some() {
            let num1 = *x;
            let num2 = 2020 - num1;
            let result = num1 as u32 * num2 as u32;
            println!("{} * {} = {}", num1, num2, result);
            break;
        }
    }
}

// three sum problem
fn part2(input: &Vec<u16>) {
    let mut sorted_input = input.clone();
    sorted_input.sort();

    for i in 0..sorted_input.len() {
        let mut left = i + 1;
        let mut right = sorted_input.len() - 1;
        while left < right {
            let sum = sorted_input[i] + sorted_input[left] + sorted_input[right];
            if sum == 2020 {
                let a = sorted_input[i] as u32;
                let b = sorted_input[left] as u32;
                let c = sorted_input[right] as u32;
                println!("{} * {} * {} = {}", a, b, c, a * b * c);
                return
            } else if sum < 2020 {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }
}

fn read_input() -> std::io::Result<Vec<u16>>{
    let mut file = File::open("input/input.txt")?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let v: Vec<u16> = data.split('\n').map(|x| x.parse().unwrap()).collect();
    Ok(v)
}