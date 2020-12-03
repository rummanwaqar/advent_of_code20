use std::fs::File;
use std::io::Read;

// Map[y][x]
type Map = Vec<Vec<char>>;

fn main() {
    let input = read_map("input/input.txt").unwrap();

    println!("Part 1: {}", part1(&input, 3, 1));
    println!("Part 2: {}", part2(&input));
}

fn part1(map: &Map, right: usize, down: usize) -> u32 {
    let height = map.len();
    let width = map[0].len();

    let mut pos_x = 0;
    let mut pos_y = 0;
    let mut trees: u32 = 0;
    while pos_y < height {
        if map[pos_y][pos_x] == '#' {
            trees += 1;
        }
        pos_x = (pos_x + right) % width;
        pos_y += down;
    }

    trees
}

fn part2(map: &Map) -> u32 {
    let path1_1 = part1(&map, 1, 1);
    let path3_1 = part1(&map, 3, 1);
    let path5_1 = part1(&map, 5, 1);
    let path7_1 = part1(&map, 7, 1);
    let path1_2 = part1(&map, 1, 2);
    path1_1 * path3_1 * path5_1 * path7_1 * path1_2
}

fn read_map(file_path: &str) -> std::io::Result<Map> {
    let mut file = File::open(file_path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let array: Map = data
        .split('\n')
        .map(|x| x.chars().collect())
        .collect();
    Ok(array)
}
