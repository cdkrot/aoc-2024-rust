use crate::utils;
use std::collections::HashSet;

fn find(field: &Vec<Vec<u8>>, what: u8) -> (i32, i32) {
    for (i, row) in field.iter().enumerate() {
        if let Some(j) = row.iter().position(|&c| c == what) {
            return (i as i32, j as i32);
        }
    }
    panic!();
}

struct VisitResult {
    total_visited: usize,
    is_cycle: bool,
}

fn visit_grid(field: &Vec<Vec<u8>>) -> VisitResult {
    let n = field.len();
    let m = field[0].len();

    let mut visited = vec![vec![false; m]; n];
    let mut cycle_breaker = HashSet::new();

    let dirs = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];

    let mut dir_id: usize = 0;
    let mut pos = find(&field, '^' as u8);
    let mut is_cycle = false;
    while 0 <= pos.0 && pos.0 < (n as i32) && 0 <= pos.1 && pos.1 < (m as i32) {
        if !cycle_breaker.insert((pos, dir_id)) {
            is_cycle = true;
            break;
        }

        if field[pos.0 as usize][pos.1 as usize] == '#' as u8 {
            pos.0 -= dirs[dir_id].0;
            pos.1 -= dirs[dir_id].1;
            dir_id = (dir_id + 1) % dirs.len();
            continue;
        }
        visited[pos.0 as usize][pos.1 as usize] = true;
        pos.0 += dirs[dir_id].0;
        pos.1 += dirs[dir_id].1;
    }

    let total_visited: usize = visited
        .iter()
        .map(|vec| vec.iter().filter(|x| **x).count())
        .sum();

    VisitResult {
        total_visited,
        is_cycle,
    }
}

pub(crate) fn main() {
    let mut field: Vec<Vec<u8>> = utils::read_all_lines()
        .iter()
        .map(|l| l.clone().into_bytes())
        .collect();

    let visit = visit_grid(&field);
    println!("Part1: {}", visit.total_visited);

    let n = field.len();
    let m = field[0].len();

    let mut total_cycled = 0;
    for i in 0..n {
        for j in 0..m {
            if field[i][j] == '.' as u8 {
                field[i][j] = '#' as u8;
                total_cycled += visit_grid(&field).is_cycle as i32;
                field[i][j] = '.' as u8;
            }
        }
    }
    println!("Part2: {}", total_cycled);
}