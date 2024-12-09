use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).expect("No input");
    let (mut left, mut right): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| {
            let mut iter = line
                .split_ascii_whitespace()
                .map(|s| s.parse::<i32>().unwrap());
            let left = iter.next().unwrap();
            let right = iter.next().unwrap();
            (left, right)
        })
        .unzip();

    left.sort();
    right.sort();

    let result: u32 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum();

    println!("part 1: {result}");
}
