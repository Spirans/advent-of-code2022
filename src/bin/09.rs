use std::collections::HashSet;

#[derive(PartialEq, Debug)]
struct Pair {
    direction: Direction,
    steps: u32,
}

impl Pair {
    fn new(dir: char, steps: u32) -> Self {
        let direction = match dir {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'R' => Direction::Right,
            'L' => Direction::Left,
            _ => panic!("unexpected input"),
        };
        Self { direction, steps }
    }
}

#[derive(PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    fn update(&mut self, other: &Position) {
        let x = other.x - self.x;
        let y = other.y - self.y;
        match (x.abs(), y.abs()) {
            (0, 0) | (1, 0) | (0, 1) | (1, 1) => (),
            (2, 0) => {
                self.x += other.x - x.signum() - self.x;
            }
            (0, 2) => {
                self.y += other.y - y.signum() - self.y;
            }
            (2, 1) => {
                self.x += other.x - x.signum() - self.x;
                self.y += other.y - self.y;
            }
            (1, 2) => {
                self.x += other.x - self.x;
                self.y += other.y - y.signum() - self.y;
            }
            (2, 2) => {
                self.x += other.x - x.signum() - self.x;
                self.y += other.y - y.signum() - self.y;
            }
            _ => panic!(),
        }
    }

    fn mv(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Right => self.x += 1,
            Direction::Left => self.x -= 1,
        }
    }
}

fn parse_line(input: &str) -> nom::IResult<&str, Pair> {
    nom::sequence::separated_pair(
        nom::character::complete::anychar,
        nom::character::complete::space1,
        nom::sequence::terminated(
            nom::character::complete::u32,
            nom::combinator::opt(nom::character::complete::line_ending),
        ),
    )(input)
    .map(|(rem, (direction, steps))| (rem, Pair::new(direction, steps)))
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut head = Position::new();
    let mut tail = Position::new();
    let mut set = HashSet::new();
    for line in input.lines() {
        let (_, pair) = parse_line(line).unwrap();
        for _ in 0..pair.steps {
            head.mv(&pair.direction);
            tail.update(&head);
            set.insert(tail.clone());
        }
    }
    Some(set.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut set = HashSet::new();
    let mut rope = vec![Position::new(); 10];
    for line in input.lines() {
        let (_, pair) = parse_line(line).unwrap();
        for _ in 0..pair.steps {
            rope[0].mv(&pair.direction);
            for idx in 1..rope.len() {
                let head = rope[idx - 1].clone();
                rope[idx].update(&head);
            }
            set.insert(rope.last().unwrap().clone());
        }
    }
    Some(set.len())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two("R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20"),
            Some(36)
        );
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("U 1"),
            Ok((
                "",
                Pair {
                    direction: Direction::Up,
                    steps: 1,
                },
            ))
        );
        assert_eq!(
            parse_line("R 4\n"),
            Ok((
                "",
                Pair {
                    direction: Direction::Right,
                    steps: 4
                }
            ))
        );
    }
}
