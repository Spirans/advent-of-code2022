use nom::{
    bytes::complete::take,
    character::complete::{line_ending, u8 as nom_u8},
    combinator::map_parser,
    multi::many1,
    sequence::terminated,
    IResult,
};

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<u8>>> {
    many1(terminated(
        many1(map_parser(take(1u8), nom_u8)),
        line_ending,
    ))(input)
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn is_out_of_bounds(table: &Vec<Vec<u8>>, row: usize, col: usize) -> bool {
    row >= table.len() || col >= table[row].len()
}

fn backtrack_visible(
    table: &Vec<Vec<u8>>,
    started_row: usize,
    started_col: usize,
    row: usize,
    col: usize,
    direction: &Direction,
) -> bool {
    if is_out_of_bounds(table, row, col) {
        return true;
    };

    if (row != started_row || col != started_col)
        && table[row][col] >= table[started_row][started_col]
    {
        return false;
    }

    let (new_row, new_col) = match direction {
        Direction::Down => (row.checked_add(1), Some(col)),
        Direction::Up => (row.checked_sub(1), Some(col)),
        Direction::Right => (Some(row), col.checked_add(1)),
        Direction::Left => (Some(row), col.checked_sub(1)),
    };

    if new_row.is_none() || new_col.is_none() {
        return true;
    }

    backtrack_visible(
        table,
        started_row,
        started_col,
        new_row.unwrap(),
        new_col.unwrap(),
        direction,
    )
}

fn backtrack_scenic(
    table: &Vec<Vec<u8>>,
    started_row: usize,
    started_col: usize,
    row: usize,
    col: usize,
    direction: &Direction,
) -> u32 {
    if row == 0
        || row == table.len() - 1
        || col == 0
        || col == table[row].len() - 1
        || ((row != started_row || col != started_col)
            && table[row][col] >= table[started_row][started_col])
    {
        return match direction {
            Direction::Up => started_row - row,
            Direction::Down => row - started_row,
            Direction::Left => started_col - col,
            Direction::Right => col - started_col,
        } as u32;
    }

    let (new_row, new_col) = match direction {
        Direction::Down => (row + 1, col),
        Direction::Up => (row - 1, col),
        Direction::Right => (row, col + 1),
        Direction::Left => (row, col - 1),
    };

    backtrack_scenic(table, started_row, started_col, new_row, new_col, direction)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, table) = parse_input(input).unwrap();

    let directions = vec![
        Direction::Down,
        Direction::Up,
        Direction::Right,
        Direction::Left,
    ];

    let mut res = 0;
    for row in 0..table.len() {
        for col in 0..table[row].len() {
            for direction in directions.iter() {
                if backtrack_visible(&table, row, col, row, col, direction) {
                    res += 1;
                    break;
                }
            }
        }
    }
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, table) = parse_input(input).unwrap();

    let directions = vec![
        Direction::Down,
        Direction::Up,
        Direction::Right,
        Direction::Left,
    ];

    let mut max_scenic = 0;
    for row in 0..table.len() {
        for col in 0..table[row].len() {
            let cur_max = directions.iter().fold(1u32, |acc, direction| {
                acc * backtrack_scenic(&table, row, col, row, col, direction)
            });
            max_scenic = max_scenic.max(cur_max);
        }
    }
    Some(max_scenic)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }

    #[test]
    fn test_parse_input() {
        let input = "30373\n25512\n";
        let (_, result) = parse_input(input).unwrap();
        let expected = vec![vec![3, 0, 3, 7, 3], vec![2, 5, 5, 1, 2]];
        assert_eq!(result, expected);
    }
}
