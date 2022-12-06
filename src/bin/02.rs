use std::cmp::Ordering;

#[derive(PartialEq, Clone, Copy)]
enum LeftSide {
    A,
    B,
    C,
}

impl From<String> for LeftSide {
    fn from(s: String) -> Self {
        match s.as_str() {
            "A" => LeftSide::A,
            "B" => LeftSide::B,
            "C" => LeftSide::C,
            _ => panic!("undefined input"),
        }
    }
}

impl LeftSide {
    fn get_according_to_strategy(self, other: RightSide) -> RightSide {
        match (self, other) {
            (LeftSide::A, RightSide::X) => RightSide::Z,
            (LeftSide::A, RightSide::Y) => RightSide::X,
            (LeftSide::A, RightSide::Z) => RightSide::Y,
            (LeftSide::B, RightSide::X) => RightSide::X,
            (LeftSide::B, RightSide::Y) => RightSide::Y,
            (LeftSide::B, RightSide::Z) => RightSide::Z,
            (LeftSide::C, RightSide::X) => RightSide::Y,
            (LeftSide::C, RightSide::Y) => RightSide::Z,
            (LeftSide::C, RightSide::Z) => RightSide::X,
        }
    }
}

impl PartialOrd<RightSide> for LeftSide {
    fn partial_cmp(&self, other: &RightSide) -> Option<std::cmp::Ordering> {
        match (other, self) {
            (RightSide::X, LeftSide::A) => Some(Ordering::Equal),
            (RightSide::X, LeftSide::B) => Some(Ordering::Less),
            (RightSide::X, LeftSide::C) => Some(Ordering::Greater),
            (RightSide::Y, LeftSide::A) => Some(Ordering::Greater),
            (RightSide::Y, LeftSide::B) => Some(Ordering::Equal),
            (RightSide::Y, LeftSide::C) => Some(Ordering::Less),
            (RightSide::Z, LeftSide::A) => Some(Ordering::Less),
            (RightSide::Z, LeftSide::B) => Some(Ordering::Greater),
            (RightSide::Z, LeftSide::C) => Some(Ordering::Equal),
        }
    }
}

impl std::cmp::PartialEq<RightSide> for LeftSide {
    fn eq(&self, other: &RightSide) -> bool {
        match self.partial_cmp(other) {
            Some(Ordering::Equal) => true,
            _ => false,
        }
    }

    fn ne(&self, other: &RightSide) -> bool {
        !self.eq(other)
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum RightSide {
    X,
    Y,
    Z,
}

impl RightSide {
    fn to_point(&self) -> u32 {
        match self {
            RightSide::X => 1,
            RightSide::Y => 2,
            RightSide::Z => 3,
        }
    }
}

impl From<String> for RightSide {
    fn from(s: String) -> Self {
        match s.as_str() {
            "X" => RightSide::X,
            "Y" => RightSide::Y,
            "Z" => RightSide::Z,
            _ => panic!("undefined input"),
        }
    }
}

impl PartialOrd<LeftSide> for RightSide {
    fn partial_cmp(&self, other: &LeftSide) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (RightSide::X, LeftSide::A) => Some(Ordering::Equal),
            (RightSide::X, LeftSide::B) => Some(Ordering::Less),
            (RightSide::X, LeftSide::C) => Some(Ordering::Greater),
            (RightSide::Y, LeftSide::A) => Some(Ordering::Greater),
            (RightSide::Y, LeftSide::B) => Some(Ordering::Equal),
            (RightSide::Y, LeftSide::C) => Some(Ordering::Less),
            (RightSide::Z, LeftSide::A) => Some(Ordering::Less),
            (RightSide::Z, LeftSide::B) => Some(Ordering::Greater),
            (RightSide::Z, LeftSide::C) => Some(Ordering::Equal),
        }
    }
}

impl std::cmp::PartialEq<LeftSide> for RightSide {
    fn eq(&self, other: &LeftSide) -> bool {
        match self.partial_cmp(other) {
            Some(Ordering::Equal) => true,
            _ => false,
        }
    }

    fn ne(&self, other: &LeftSide) -> bool {
        !self.eq(other)
    }
}

struct Game {
    total_score: u32,
}

impl Game {
    fn new() -> Self {
        Self { total_score: 0 }
    }

    fn round(&mut self, l: LeftSide, r: RightSide) {
        let score = match r.partial_cmp(&l) {
            Some(Ordering::Less) => 0,
            Some(Ordering::Equal) => 3,
            Some(Ordering::Greater) => 6,
            None => 0,
        };
        self.total_score += score + r.to_point();
    }

    fn total_score(self) -> u32 {
        self.total_score
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut game = Game::new();
    for line in input.lines() {
        let pair: Vec<_> = line.split(" ").collect();
        let left_side = String::from(pair[0]);
        let right_side = String::from(pair[1]);
        game.round(left_side.into(), right_side.into())
    }
    Some(game.total_score())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut game = Game::new();
    for line in input.lines() {
        let pair: Vec<_> = line.split(" ").collect();
        let left_side = String::from(pair[0]);
        let right_side = String::from(pair[1]);
        let left: LeftSide = left_side.into();
        let right = left.get_according_to_strategy(right_side.into());
        game.round(left, right);
    }
    Some(game.total_score())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
