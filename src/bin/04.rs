struct Range {
    left_bound: u32,
    right_bound: u32,
}

impl Range {
    fn fully_overlap(&self, rhs: &Range) -> bool {
        rhs.left_bound >= self.left_bound && rhs.right_bound <= self.right_bound
    }

    fn partially_overlap(&self, rhs: &Range) -> bool {
        (rhs.left_bound >= self.left_bound && rhs.left_bound <= self.right_bound)
            || (rhs.right_bound >= self.left_bound && rhs.right_bound <= self.right_bound)
    }
}

impl From<&str> for Range {
    fn from(line: &str) -> Self {
        let pair: Vec<_> = line.split("-").collect();
        Self {
            left_bound: pair[0].parse::<u32>().unwrap(),
            right_bound: pair[1].parse::<u32>().unwrap(),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let splitted: Vec<_> = line.split(",").collect();
                let left_range = Range::from(splitted[0]);
                let right_range = Range::from(splitted[1]);
                if left_range.fully_overlap(&right_range) || right_range.fully_overlap(&left_range)
                {
                    1
                } else {
                    0
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let splitted: Vec<_> = line.split(",").collect();
                let left_range = Range::from(splitted[0]);
                let right_range = Range::from(splitted[1]);
                if left_range.partially_overlap(&right_range)
                    || right_range.partially_overlap(&left_range)
                {
                    1
                } else {
                    0
                }
            })
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
