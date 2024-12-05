use crate::utils;
use std::collections::VecDeque;

#[derive(Debug)]
struct Input {
    rules: Vec<(i32, i32)>,
    updates: Vec<Vec<i32>>,
}

fn parse(input: Vec<String>) -> Input {
    let mut iter = input.iter();

    let head: Vec<(i32, i32)> = iter
        .by_ref()
        .take_while(|&s| !s.is_empty())
        .cloned()
        .map(|s| {
            s.split("|")
                .map(|t| t.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|vec| (vec[0], vec[1]))
        .collect();

    let tail: Vec<Vec<i32>> = iter
        .map(|line| line.split(',').map(|t| t.parse::<i32>().unwrap()).collect())
        .collect();

    Input {
        rules: head,
        updates: tail,
    }
}

fn matches(rule: (i32, i32), update: &Vec<i32>) -> bool {
    let pos1 = update.iter().position(|p| *p == rule.0);
    let pos2 = update.iter().position(|p| *p == rule.1);

    pos1.is_none() || pos2.is_none() || (pos1.unwrap() < pos2.unwrap())
}

fn filter_correct(input: &Input) -> Vec<&Vec<i32>> {
    input
        .updates
        .iter()
        .filter(|update| input.rules.iter().all(|rule| matches(*rule, *update)))
        .collect()
}

fn filter_incorrect(input: &Input) -> Vec<&Vec<i32>> {
    input
        .updates
        .iter()
        .filter(|update| !input.rules.iter().all(|rule| matches(*rule, *update)))
        .collect()
}

fn reorder(update: &Vec<i32>, input: &Input) -> Vec<i32> {
    let mut balance = vec![0; update.len()];
    let mut graph = vec![vec![]; update.len()];

    input.rules.iter().for_each(|rule| {
        let pos0 = update.iter().position(|p| *p == rule.0);
        let pos1 = update.iter().position(|p| *p == rule.1);

        let _ = pos0.is_none() || pos1.is_none() || {
            balance[pos1.unwrap()] += 1;
            graph[pos0.unwrap()].push(pos1.unwrap());
            true
        };
    });

    let mut queue = VecDeque::new();
    let mut result = vec![];

    println!("{update:?}, {balance:?} {graph:?}");

    balance
        .iter()
        .enumerate()
        .filter(|(_i, val)| **val == 0)
        .for_each(|(i, _val)| {
            queue.push_back(i);
        });

    while let Some(i) = queue.pop_front() {
        result.push(update[i]);

        for j in graph[i].iter() {
            balance[*j] -= 1;
            if balance[*j] == 0 {
                queue.push_back(*j);
            }
        }
    }

    result
}

fn reorder_incorrect(input: &Input) -> Vec<Vec<i32>> {
    let incorrect = filter_incorrect(&input);
    incorrect
        .iter()
        .map(|update| reorder(update, &input))
        .collect()
}

pub(crate) fn main() {
    let input = parse(utils::read_all_lines());
    let correct = filter_correct(&input);

    let total: i32 = correct.iter().map(|vec| vec[vec.len() / 2]).sum();
    println!("Part 1: {total}");

    let incorrect = reorder_incorrect(&input);
    let total_part2: i32 = incorrect.iter().map(|vec| vec[vec.len() / 2]).sum();
    println!("Incorrect: {incorrect:?}");
    println!("Part2: {total_part2}");
}
