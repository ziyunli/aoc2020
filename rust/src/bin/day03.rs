extern crate aoc2020;

use aoc2020::common::read_lines;

fn count_trees(map: &Vec<String>, right: usize, down: usize) -> usize {
    let height = map.len();
    let width = map.first().unwrap().len();

    let mut row = 0;
    let mut col = 0;

    let mut count = 0;
    while row < height {
        if map[row].chars().nth(col % width).unwrap() == '#' {
            count += 1;
        }
        row += down;
        col += right;
    }
    count
}

fn main() {
    if let Ok(lines) = read_lines("../input/03.txt") {
        let mut map: Vec<String> = Vec::new();
        for line in lines {
            if let Ok(ip) = line {
                map.push(ip);
            }
        }

        // Part 1
        println!("Right {}, down {}: {}", 3, 1, count_trees(&map, 3, 1));

        // Part 2
        println!(
            "Part 2: {}",
            count_trees(&map, 1, 1)
                * count_trees(&map, 3, 1)
                * count_trees(&map, 5, 1)
                * count_trees(&map, 7, 1)
                * count_trees(&map, 1, 2)
        );
    }
}
