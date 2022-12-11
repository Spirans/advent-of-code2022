use std::collections::{HashSet, VecDeque};

fn solve(input: &str, cap: usize) -> Option<u32> {
    let mut unique: HashSet<char> = HashSet::new();
    let mut current: VecDeque<char> = VecDeque::new();

    for (index, ch) in input.chars().enumerate() {
        if !unique.contains(&ch) {
            current.push_back(ch);
            unique.insert(ch);
            if unique.len() == cap {
                return Some((index + 1) as u32);
            }
        } else {
            while !current.is_empty() && current.front().unwrap() != &ch {
                let last = current.pop_front().unwrap();
                unique.remove(&last);
            }
            let last = current.pop_front().unwrap();
            current.push_back(last);
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, 14)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(11));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(26));
    }
}
