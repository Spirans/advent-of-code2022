use std::collections::{HashMap, VecDeque};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, digit1, space1, u32 as nom_u32},
    combinator::{map, map_res},
    multi::separated_list0,
    sequence::{delimited, tuple},
    IResult,
};

#[derive(PartialEq, Debug)]
struct Move {
    count: u32,
    from: u32,
    to: u32,
}

impl Move {
    fn from(count: u32, from: u32, to: u32) -> Self {
        Self { count, from, to }
    }
}

fn parse_char(input: &str) -> IResult<&str, Option<char>> {
    map(delimited(tag("["), anychar, tag("]")), |ch| Some(ch))(input)
}

fn parse_triple_spaces(input: &str) -> IResult<&str, Option<char>> {
    map(tag("   "), |_| None)(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<Option<char>>> {
    separated_list0(tag(" "), alt((parse_char, parse_triple_spaces)))(input)
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<u32>> {
    let input = input.trim();
    separated_list0(space1, map_res(digit1, |i: &str| i.parse()))(input)
}

fn parse_moves(input: &str) -> IResult<&str, Move> {
    map(
        tuple((
            tag("move "),
            nom_u32,
            tag(" from "),
            nom_u32,
            tag(" to "),
            nom_u32,
        )),
        |(_, count, _, from, _, to)| Move::from(count, from, to),
    )(input)
}

pub fn part_one(input: &str) -> Option<String> {
    let mut raw_buckets: HashMap<u32, VecDeque<char>> = HashMap::new();
    let mut buckets: HashMap<u32, Vec<char>> = HashMap::new();

    let mut lines_iterator = input.lines();
    let mut col_numbers = Vec::new();
    while let Some(line) = lines_iterator.next() {
        let (remaining, parsed) = parse_line(line).ok()?;
        if remaining == line {
            let (_, numbers) = parse_numbers(line).ok()?;
            col_numbers = numbers.clone();
            for (index, number) in numbers.into_iter().enumerate() {
                if let Some(v) = raw_buckets.remove(&(index as u32)) {
                    buckets.insert(number, Vec::from(v));
                }
            }
            break;
        } else {
            for (index, ch) in parsed.into_iter().enumerate() {
                if let Some(v) = ch {
                    raw_buckets
                        .entry(index as u32)
                        .and_modify(|e| e.push_front(v))
                        .or_insert(VecDeque::from([v]));
                }
            }
        }
    }
    lines_iterator.next(); // empty line

    while let Some(line) = lines_iterator.next() {
        let (_, mv) = parse_moves(line).ok()?;
        for _ in 0..mv.count {
            let v = buckets.get_mut(&mv.from).unwrap().pop().unwrap();
            buckets.get_mut(&mv.to).unwrap().push(v);
        }
    }

    let mut res: Vec<char> = Vec::new();
    for col in col_numbers {
        let cargo = buckets.get_mut(&col).unwrap().pop().unwrap();
        res.push(cargo);
    }

    Some(res.iter().collect())
}

pub fn part_two(input: &str) -> Option<String> {
    let mut raw_buckets: HashMap<u32, VecDeque<char>> = HashMap::new();
    let mut buckets: HashMap<u32, Vec<char>> = HashMap::new();

    let mut lines_iterator = input.lines();
    let mut col_numbers = Vec::new();
    while let Some(line) = lines_iterator.next() {
        let (remaining, parsed) = parse_line(line).ok()?;
        if remaining == line {
            let (_, numbers) = parse_numbers(line).ok()?;
            col_numbers = numbers.clone();
            for (index, number) in numbers.into_iter().enumerate() {
                if let Some(v) = raw_buckets.remove(&(index as u32)) {
                    buckets.insert(number, Vec::from(v));
                }
            }
            break;
        } else {
            for (index, ch) in parsed.into_iter().enumerate() {
                if let Some(v) = ch {
                    raw_buckets
                        .entry(index as u32)
                        .and_modify(|e| e.push_front(v))
                        .or_insert(VecDeque::from([v]));
                }
            }
        }
    }
    lines_iterator.next(); // empty line

    while let Some(line) = lines_iterator.next() {
        let (_, mv) = parse_moves(line).ok()?;
        let mut cur_vec = VecDeque::new();
        for _ in 0..mv.count {
            let v = buckets.get_mut(&mv.from).unwrap().pop().unwrap();
            cur_vec.push_front(v);
        }
        buckets.get_mut(&mv.to).unwrap().extend(cur_vec);
    }

    let mut res: Vec<char> = Vec::new();
    for col in col_numbers {
        let cargo = buckets.get_mut(&col).unwrap().pop().unwrap();
        res.push(cargo);
    }

    Some(res.iter().collect())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }

    #[test]
    fn test_parse_char() {
        assert!(parse_char("[A").is_err());
        assert!(parse_char("A]").is_err());
        assert!(parse_char("A").is_err());
        assert_eq!(parse_char("[A]"), Ok(("", Some('A'))));
    }

    #[test]
    fn test_parse_space() {
        assert!(parse_triple_spaces("  ").is_err());
        assert_eq!(parse_triple_spaces("   "), Ok(("", None)));
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("123"), Ok(("123", vec![])));
        assert_eq!(parse_line("  [A]"), Ok(("  [A]", vec![])));
        let spaces = "   ";
        let input = format!("{spaces} [A] [B] {spaces} [C] {spaces}");
        let expected_vec = vec![None, Some('A'), Some('B'), None, Some('C'), None];
        assert_eq!(parse_line(input.as_str()), Ok(("", expected_vec)));
    }

    #[test]
    fn test_parse_moves() {
        assert!(parse_moves("move").is_err());
        assert_eq!(
            parse_moves("move 2 from 8 to 2"),
            Ok(("", Move::from(2, 8, 2)))
        );
    }

    #[test]
    fn test_parse_numbers() {
        let spaces = "   ";
        assert_eq!(
            parse_numbers(format!(" 1{spaces}2{spaces}3{spaces}4{spaces}5 ").as_str()),
            Ok(("", vec![1, 2, 3, 4, 5]))
        )
    }
}
