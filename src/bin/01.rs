use std::{cmp::Reverse, collections::BinaryHeap};

pub fn part_one(input: &str) -> Option<u32> {
    let (res, _) = input.lines().fold((0, 0), |(res, acc), line| {
        if line.is_empty() {
            (res.max(acc), 0)
        } else {
            (res, acc + line.parse::<u32>().unwrap())
        }
    });
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut heap = BinaryHeap::new();
    let mut local_sum = 0;
    for line in input.lines() {
        if line.is_empty() {
            heap.push(Reverse(local_sum));
            local_sum = 0;
        } else {
            local_sum += line.parse::<u32>().unwrap();
        }
        if heap.len() > 3 {
            heap.pop();
        }
    }
    if local_sum > 0 {
        heap.push(Reverse(local_sum));
        if heap.len() > 3 {
            heap.pop();
        }
    }
    Some(heap.iter().fold(0, |acc, Reverse(i)| acc + i))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
