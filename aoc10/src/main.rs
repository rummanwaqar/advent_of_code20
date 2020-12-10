use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = read_input("input/input.txt").unwrap();
    input.push(0);
    input.sort_unstable();
    input.push(input.last().unwrap() + 3);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input, 0, &mut HashMap::new()));
}

fn part1(input: &Vec<i32>) -> i32 {
    let mut n1 = 0;
    let mut n3 = 0;
    for i in 0..input.len() - 1 {
        let diff = input[i + 1] - input[i];
        if diff == 1 {
            n1 += 1;
        } else if diff == 3 {
            n3 += 1;
        }
    }
    println!("{}", n3);
    n1 * n3
}

fn part2(input: &Vec<i32>, index: usize, cache: &mut HashMap<usize, u64>) -> u64 {
    // solved with DP
    // number of ways to complete adapter chain if you are at index

    // if at the end then only one way to solve it
    if index == input.len() - 1 {
        return 1;
    }

    // use memoization to speed up
    match cache.get(&index).map(|x| x.clone()) {
        Some(result) => result,
        None => {
            let mut permutations = 0;
            // recursively call to get all permutations for next items in list
            for i in index + 1..input.len() {
                // if difference is less than 3 then chain can be simplified
                if input[i] - input[index] <= 3 {
                    permutations += part2(&input, i, cache);
                }
            }
            cache.insert(index, permutations);
            permutations
        }
    }
}

fn read_input(file_path: &str) -> std::io::Result<Vec<i32>> {
    let mut file = File::open(file_path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    Ok(data.split('\n').map(|x| x.parse().unwrap()).collect())
}
