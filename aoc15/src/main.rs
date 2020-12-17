use std::collections::HashMap;

fn main() {
    let input = vec![14,1,17,0,3,20];
    println!("Part 1: {}", puzzle(&input, 2020));
    println!("Part 2: {}", puzzle(&input, 30000000));
}

fn puzzle(input: &Vec<i32>, n_turns: usize) -> i32 {
    let mut memory = HashMap::new();
    for index in 0..(input.len() - 1) {
        memory.insert(input[index], index);
    }

    let mut current = *input.last().unwrap();
    for i in (input.len() - 1)..(n_turns-1) {
        let diff = i - match memory.get(&current) {
            None => i,
            Some(&val) => val
        };
        memory.insert(current, i);
        current = diff as i32;
    }
    current
}
