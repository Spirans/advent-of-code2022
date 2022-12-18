use std::collections::{HashMap, HashSet};

use once_cell::sync::Lazy;

static MAPPING: Lazy<HashMap<char, u32>> = Lazy::new(build_mapping);

fn build_mapping() -> HashMap<char, u32> {
    ('a'..='z').chain('A'..='Z').zip(1..=52).collect()
}

fn calculate(s: String) -> u32 {
    let middle = s.len() / 2;
    let (left, right) = s.split_at(middle);
    let left_set: HashSet<char> = left.chars().collect();
    let right_set: HashSet<char> = right.chars().collect();

    left_set
        .intersection(&right_set)
        .map(|i| *MAPPING.get(i).unwrap())
        .sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .into_iter()
            .map(|i| calculate(i.to_string()))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut s = 0;
    let mut chunk = Vec::new();
    for line in input.lines() {
        chunk.push(line);
        if chunk.len() < 3 {
            continue;
        }
        let first_set: HashSet<char> = chunk[0].chars().collect();
        let second_set: HashSet<char> = chunk[1].chars().collect();
        let third_set: HashSet<char> = chunk[2].chars().collect();
        let mut res = Vec::new();
        for i in first_set.iter() {
            if second_set.contains(i) && third_set.contains(i) {
                res.push(*MAPPING.get(i).unwrap())
            }
        }
        if res.len() > 1 {
            println!("{:?}: {:?}", chunk, res);
            panic!("bad amount of elements")
        }
        s += res[0];
        chunk.clear();
    }
    Some(s)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
