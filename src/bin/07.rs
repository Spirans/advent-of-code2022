use std::{cell::RefCell, collections::HashMap, rc::Rc};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, newline, not_line_ending, space1, u32 as nom_u32},
    combinator::{map, opt},
    multi::{fold_many1, many1},
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    IResult,
};

#[derive(Debug)]
struct TreeNode {
    children: HashMap<String, Rc<RefCell<TreeNode>>>,
    n_type: NodeType,
    val: u32,
}

impl TreeNode {
    fn new(t: NodeType) -> Self {
        Self {
            children: HashMap::new(),
            n_type: t,
            val: 0,
        }
    }

    fn add_val(&mut self, v: u32) {
        self.val += v
    }
}

#[derive(Debug, PartialEq)]
enum NodeType {
    Dir,
    File(u32),
}

#[derive(Debug, PartialEq)]
enum CD {
    Root,
    Parent,
    ToNode(String),
}

impl From<&str> for CD {
    fn from(input: &str) -> Self {
        match input {
            ".." => CD::Parent,
            "/" => CD::Root,
            _ => CD::ToNode(input.to_string()),
        }
    }
}

const cmd_sign: &str = "$";

#[derive(Debug, PartialEq)]
enum Action {
    Listing(LS),
    DirChange(CD),
    LSCommand,
}

#[derive(Debug, PartialEq)]
enum LS {
    File(u32, String),
    Dir(String),
}

impl From<CD> for Action {
    fn from(data: CD) -> Self {
        Self::DirChange(data)
    }
}

fn parse_file(input: &str) -> IResult<&str, Action> {
    pair(
        terminated(nom_u32, space1),
        terminated(not_line_ending, newline),
    )(input)
    .and_then(|(rem, (size, name))| Ok((rem, Action::Listing(LS::File(size, name.to_string())))))
}

fn parse_dir(input: &str) -> IResult<&str, Action> {
    delimited(tag("dir"), preceded(space1, alpha1), newline)(input)
        .and_then(|(rem, name)| Ok((rem, Action::Listing(LS::Dir(name.to_string())))))
}

fn parse_ls(input: &str) -> IResult<&str, Action> {
    delimited(parse_cmd_prefix, tag("ls"), newline)(input)
        .and_then(|(rem, _)| Ok((rem, Action::LSCommand)))
}

fn parse_cmd_prefix(input: &str) -> IResult<&str, &str> {
    terminated(tag(cmd_sign), space1)(input)
}

fn parse_cd(input: &str) -> IResult<&str, Action> {
    terminated(
        preceded(
            preceded(parse_cmd_prefix, terminated(tag("cd"), space1)),
            alt((tag(".."), tag("/"), alpha1)),
        ),
        line_ending,
    )(input)
    .and_then(|(rem, cd)| Ok((rem, Action::DirChange(cd.into()))))
}

fn parse(input: &str) -> IResult<&str, Vec<Action>> {
    many1(alt((parse_cd, parse_ls, parse_file, parse_dir)))(input)
}

fn calc_at_most(root: Rc<RefCell<TreeNode>>, at_most: u32, acc: &mut u32) -> u32 {
    let total =
        root.borrow()
            .children
            .values()
            .fold(0, |in_acc, node| match node.borrow().n_type {
                NodeType::Dir => in_acc + calc_at_most(node.clone(), at_most, acc),
                NodeType::File(v) => in_acc + v,
            });

    if total < at_most {
        *acc += total;
    }

    total
}

fn calc_to_delete(root: Rc<RefCell<TreeNode>>, lo_bound: u32, acc: &mut u32) -> u32 {
    let total =
        root.borrow()
            .children
            .values()
            .fold(0, |in_acc, node| match node.borrow().n_type {
                NodeType::Dir => in_acc + calc_to_delete(node.clone(), lo_bound, acc),
                NodeType::File(v) => in_acc + v,
            });

    if *acc > total && total > lo_bound {
        *acc = total;
    }

    total
}

fn build_tree(input: &str) -> Rc<RefCell<TreeNode>> {
    let wrapped_root = Rc::new(RefCell::new(TreeNode::new(NodeType::Dir)));
    let mut stack = vec![wrapped_root.clone()];

    let (rem, actions) = parse(input).unwrap();
    assert!(rem.is_empty());

    for action in actions.into_iter() {
        match action {
            Action::Listing(v) => match v {
                LS::File(size, name) => {
                    stack.last().unwrap().borrow_mut().children.insert(
                        name,
                        Rc::new(RefCell::new(TreeNode::new(NodeType::File(size)))),
                    );
                }
                LS::Dir(name) => {
                    stack
                        .last()
                        .unwrap()
                        .borrow_mut()
                        .children
                        .insert(name, Rc::new(RefCell::new(TreeNode::new(NodeType::Dir))));
                }
            },
            Action::DirChange(cd) => match cd {
                CD::Root => (),
                CD::Parent => {
                    stack.pop();
                }
                CD::ToNode(v) => {
                    let node = stack.last().unwrap().borrow().children[&v].clone();
                    stack.push(node);
                }
            },
            Action::LSCommand => (),
        }
    }
    wrapped_root
}

pub fn part_one(input: &str) -> Option<u32> {
    let root = build_tree(input);

    let mut acc = 0;
    calc_at_most(root, 100000, &mut acc);
    Some(acc)
}

pub fn part_two(input: &str) -> Option<u32> {
    let root = build_tree(input);

    let mut acc = 0;
    let total_size = calc_at_most(root.clone(), 10000, &mut acc);
    let lo = total_size - (70000000 - 30000000);
    let mut acc = total_size;
    calc_to_delete(root, lo, &mut acc);
    Some(acc)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }

    #[test]
    fn test_parse_file() {
        assert_eq!(
            parse_file("111 b.txt\n"),
            Ok(("", Action::Listing(LS::File(111, "b.txt".to_string()))))
        );
        assert_eq!(
            parse_file("222 a\n"),
            Ok(("", Action::Listing(LS::File(222, "a".to_string()))))
        );
    }

    #[test]
    fn test_parse_dir() {
        assert_eq!(
            parse_dir("dir e\n"),
            Ok(("", Action::Listing(LS::Dir("e".to_string()))))
        )
    }

    #[test]
    fn test_parse_ls() {
        assert_eq!(parse_ls("$ ls\n"), Ok(("", Action::LSCommand)))
    }

    #[test]
    fn test_parse_cd() {
        assert_eq!(parse_cd("$ cd /\n"), Ok(("", Action::DirChange(CD::Root))));
        assert_eq!(
            parse_cd("$ cd ..\n"),
            Ok(("", Action::DirChange(CD::Parent)))
        );
        assert_eq!(
            parse_cd("$ cd d\n"),
            Ok(("", Action::DirChange(CD::ToNode("d".to_string()))))
        );
    }

    #[test]
    fn test_parse_cmd_prefix() {
        assert_eq!(parse_cmd_prefix("$ "), Ok(("", cmd_sign)))
    }

    #[test]
    fn test_parse() {
        let input = "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c\ndir d\n$ cd a\n$ cd ..\n";
        let (_, result) = parse(input).unwrap();
        let expected = vec![
            Action::DirChange(CD::Root),
            Action::LSCommand,
            Action::Listing(LS::Dir("a".to_string())),
            Action::Listing(LS::File(14848514, "b.txt".to_string())),
            Action::Listing(LS::File(8504156, "c".to_string())),
            Action::Listing(LS::Dir("d".to_string())),
            Action::DirChange(CD::ToNode("a".to_string())),
            Action::DirChange(CD::Parent),
        ];
        assert_eq!(result, expected);
    }
}
