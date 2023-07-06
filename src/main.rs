#![cfg_attr(not(debug_assertions), deny(warnings))]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::cargo)]
#![forbid(non_ascii_idents)]
#![allow(clippy::uninlined_format_args)]

use std::env::ArgsOs;

use clap::{command, value_parser, Arg};

#[derive(Clone)]
enum Operation {
    Divide,
    Multiply,
    Add,
    Subtract,
}

impl std::fmt::Debug for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Divide => write!(f, "/"),
            Self::Multiply => write!(f, "*"),
            Self::Add => write!(f, "+"),
            Self::Subtract => write!(f, "-"),
        }
    }
}

#[derive(Clone)]
struct Step {
    op1: u32,
    op2: u32,
    operation: Operation,
}

impl std::fmt::Debug for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:?} {}", self.op1, self.operation, self.op2)
    }
}

fn parse_cli_from(args: ArgsOs) -> Result<(u32, Vec<u32>), color_eyre::Report> {
    let expected_result = "expected_result";
    let board_value = "board_value";

    let x = command!()
        .arg(
            Arg::new(expected_result)
                .index(1)
                .value_parser(value_parser!(u32)),
        )
        .arg(
            Arg::new(board_value)
                .index(2)
                .num_args(6)
                .trailing_var_arg(true)
                .value_parser(value_parser!(u32)),
        );

    let matches = x.try_get_matches_from(args)?;

    let expected_result = *matches.get_one::<u32>(expected_result).unwrap();

    let board = matches
        .get_many(board_value)
        .unwrap()
        .copied()
        .collect::<Vec<u32>>();

    Ok((expected_result, board))
}

fn parse_cli() -> Result<(u32, Vec<u32>), color_eyre::Report> {
    parse_cli_from(std::env::args_os())
}

fn main() -> Result<(), color_eyre::Report> {
    let (expected, v) = parse_cli()?;

    let permutations = build_permutations_r(&v)
        .into_iter()
        .map(|p| (p, vec![]))
        .collect::<Vec<(Vec<u32>, Vec<Step>)>>();

    let solutions = solve(permutations, expected);

    let ff = solutions
        .into_iter()
        .min_by(|x, y| usize::cmp(&x.len(), &y.len()));

    println!("{:?}", ff);

    Ok(())
}

fn solve(mut permutations: Vec<(Vec<u32>, Vec<Step>)>, expected: u32) -> Vec<Vec<Step>> {
    let mut solutions = vec![];
    while let Some((mut permutation, path)) = permutations.pop() {
        let last = match permutation.pop() {
            Some(l) => {
                if l == expected {
                    solutions.push(path);
                    continue;
                }
                l
            },
            None => {
                continue;
            },
        };

        let Some(second_last ) = permutation.pop() else {
            continue;
        };

        if let Some(x) = try_solve(last, second_last, Operation::Divide, &permutation, &path) {
            permutations.push(x);
        }
        if let Some(x) = try_solve(last, second_last, Operation::Multiply, &permutation, &path) {
            permutations.push(x);
        }
        if let Some(x) = try_solve(last, second_last, Operation::Add, &permutation, &path) {
            permutations.push(x);
        }
        if let Some(x) = try_solve(last, second_last, Operation::Subtract, &permutation, &path) {
            permutations.push(x);
        }
    }

    solutions
}

fn try_solve(
    op1: u32,
    op2: u32,
    operation: Operation,
    old_permutation: &[u32],
    old_path: &[Step],
) -> Option<(Vec<u32>, Vec<Step>)> {
    let result = match operation {
        Operation::Divide => match op1.checked_rem(op2) {
            Some(_) => None,
            None => Some(op1 / op2),
        },
        Operation::Multiply => Some(op1 * op2),
        Operation::Add => Some(op1 + op2),
        Operation::Subtract => op1.checked_sub(op2),
    };

    if let Some(r) = result {
        let mut new_permutation = old_permutation.to_vec();
        new_permutation.push(r);

        let mut path = old_path.to_vec();
        path.push(Step {
            op1,
            op2,
            operation,
        });

        Some((new_permutation, path))
    } else {
        None
    }
}

fn build_permutations_r(slice: &[u32]) -> Vec<Vec<u32>> {
    if slice.is_empty() {
        return vec![vec![]];
    }

    let mut total = vec![];

    for index in 0..slice.len() {
        let mut c = slice.to_vec();

        let at = c.remove(index);

        for mut p in build_permutations_r(&c) {
            p.insert(0, at);

            total.push(p);
        }
    }

    total
}
