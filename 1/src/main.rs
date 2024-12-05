use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    solution_1("data/input_1.txt");
}

fn solution_1(filename: &str) {
    let file = File::open(filename).expect("File not found");
    let reader = BufReader::new(file);

    let mut numbers_left = Vec::new();
    let mut numbers_right = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let numbers: Vec<u32> = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        assert_eq!(numbers.len(), 2);
        numbers_left.push(numbers[0]);
        numbers_right.push(numbers[1]);
    }
    numbers_left.sort_unstable();
    numbers_right.sort_unstable();

    let mut total_diff: u32 = 0;
    for (left, right) in numbers_left.iter().zip(numbers_right.iter()) {
        total_diff += left.abs_diff(*right);
    }
    println!("{}", total_diff);
}
