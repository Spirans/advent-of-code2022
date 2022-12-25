use std::collections::HashSet;

use nom::IResult;

#[derive(PartialEq, Debug)]
enum Token {
    Noop,
    Addx(i32),
}

fn parse_noop(input: &str) -> IResult<&str, Token> {
    nom::sequence::terminated(
        nom::bytes::complete::tag("noop"),
        nom::combinator::opt(nom::character::complete::line_ending),
    )(input)
    .map(|(rem, _)| (rem, Token::Noop))
}

fn parse_addx(input: &str) -> IResult<&str, Token> {
    nom::sequence::separated_pair(
        nom::bytes::complete::tag("addx"),
        nom::character::complete::space1,
        nom::sequence::terminated(
            nom::character::complete::i32,
            nom::combinator::opt(nom::character::complete::line_ending),
        ),
    )(input)
    .map(|(rem, (_, v))| (rem, Token::Addx(v)))
}

fn parse_line(input: &str) -> IResult<&str, Token> {
    nom::branch::alt((parse_noop, parse_addx))(input)
}

fn check_pos(x: i32, crt: i32) -> bool {
    HashSet::from([x - 1, x, x + 1]).contains(&crt)
}

fn print_if_full(chars: &mut Vec<char>) {
    if chars.len() != 40 {
        return;
    }
    let line: String = chars.clone().into_iter().collect();
    dbg!(line);
    chars.clear();
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut res = 0;
    let mut x = 1;
    let mut cycle = 0;
    let cycles = HashSet::from([20, 60, 100, 140, 180, 220]);
    for line in input.lines() {
        let (_, token) = parse_line(line).unwrap();
        cycle += 1;
        if cycles.contains(&cycle) {
            res += x * cycle;
        }
        match token {
            Token::Noop => (),
            Token::Addx(v) => {
                cycle += 1;
                if cycles.contains(&cycle) {
                    res += x * cycle;
                }
                x += v;
            }
        }
    }
    Some(res)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut chars = Vec::new();
    let mut x = 1;
    let mut crt = 0;
    for line in input.lines() {
        let (_, token) = parse_line(line).unwrap();
        if check_pos(x, crt) {
            chars.push('#')
        } else {
            chars.push('.')
        }
        print_if_full(&mut chars);
        crt += 1;
        crt %= 40;

        match token {
            Token::Noop => (),
            Token::Addx(v) => {
                if check_pos(x, crt) {
                    chars.push('#')
                } else {
                    chars.push('.')
                }
                print_if_full(&mut chars);
                crt += 1;
                crt %= 40;
                x += v;
            }
        }
    }

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }

    #[test]
    fn test_parse_noop() {
        assert_eq!(parse_noop("noop\n"), Ok(("", Token::Noop)))
    }

    #[test]
    fn test_parse_addx() {
        assert_eq!(parse_addx("addx -5\n"), Ok(("", Token::Addx(-5))))
    }
}
