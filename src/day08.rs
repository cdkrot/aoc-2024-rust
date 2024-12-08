use crate::utils;
use crate::vec::Vec2d;
use std::collections::HashMap;

struct Input {
    n: i32,
    m: i32,
    antennas: Vec<Vec<Vec2d>>,
}

fn parse(lines: &Vec<String>) -> Input {
    let mut hashmap = HashMap::new();

    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c != '.' {
                hashmap
                    .entry(c)
                    .or_insert_with(Vec::new)
                    .push(Vec2d::new(i as i32, j as i32));
            }
        }
    }

    Input {
        n: lines.len() as i32,
        m: lines[0].len() as i32,
        antennas: hashmap.iter().map(|(_, v)| v.clone()).collect(),
    }
}

fn is_antinode_with_set_part1(antennas: &Vec<Vec2d>, coord: Vec2d) -> bool {
    let res = antennas.iter().any(|&a| {
        antennas
            .iter()
            .filter(|&&b| a != b)
            .any(|&b| (a - coord).scale(2).to_upperplane() == (coord - b).to_upperplane())
    });

    res
}

fn is_antinode_with_set_part2(antennas: &Vec<Vec2d>, coord: Vec2d) -> bool {
    let res = antennas.iter().any(|&a| {
        antennas
            .iter()
            .filter(|&&b| a != b)
            .any(|&b| simplify(a - b) == simplify(a - coord))
    });

    res
}

fn is_antinode(
    input: &Input,
    coord: Vec2d,
    is_antinode_with_set: &dyn Fn(&Vec<Vec2d>, Vec2d) -> bool,
) -> bool {
    input
        .antennas
        .iter()
        .any(|set| is_antinode_with_set(set, coord))
}

fn simplify(v: Vec2d) -> Vec2d {
    let mut g = utils::gcd(v.x, v.y);
    if g == 0 {
        g = 1;
    }
    return Vec2d::new(v.x / g, v.y / g).to_upperplane();
}

fn count_antinodes(
    input: &Input,
    is_antinode_with_set: &dyn Fn(&Vec<Vec2d>, Vec2d) -> bool,
) -> usize {
    (0..input.n)
        .map(|i| {
            (0..input.m)
                .filter(|j| is_antinode(&input, Vec2d::new(i, *j), is_antinode_with_set))
                .count()
        })
        .sum()
}

pub(crate) fn main() {
    let input = parse(&utils::read_all_lines());

    let total_antinodes = count_antinodes(&input, &is_antinode_with_set_part1);

    println!("Total part1: {total_antinodes}");

    let total_antinodes_p2 = count_antinodes(&input, &is_antinode_with_set_part2);

    println!("Total part2: {total_antinodes_p2}");
}
