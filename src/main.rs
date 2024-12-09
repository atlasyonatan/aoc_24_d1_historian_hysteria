use std::io::{stdin, Read};

use itertools::Itertools;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).expect("No input");

    let (mut left, mut right): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| {
            let mut iter = line
                .split_ascii_whitespace()
                .map(|s| s.parse::<u32>().unwrap());
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

    let occurance_map = right
        .into_iter()
        .into_grouping_map_by(|n| *n)
        .fold(0usize, |acc, _, _| acc + 1);

    let result: usize = left
        .into_iter()
        .filter_map(|l| occurance_map.get(&l).and_then(|&r| Some(r * l as usize))) //similarity scores
        .sum();

    println!("part 2: {result}");
}
