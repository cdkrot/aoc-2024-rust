use crate::utils;

pub(crate) fn main() {
    let lines = utils::read_all_lines();

    let line_pairs: Vec<Vec<i32>> = lines.iter().map(
        |line| line.split_whitespace().map(|tok| tok.parse().unwrap()).collect()
    ).collect();

    let mut column_a: Vec<i32> = line_pairs.iter().map(|row| row[0]).collect();
    let mut column_b: Vec<i32> = line_pairs.iter().map(|row| row[1]).collect();

    column_a.sort();
    column_b.sort();

    let total: i32 = column_a.iter().zip(column_b.iter()).map(|(a, b)| (a - b).abs()).sum();
    println!("Part1: {total}");

    let products: i32 = column_a.iter().map(
        |&num| column_b.iter().filter(|&&num2| num2 == num).count() as i32 * num
    ).sum();
    println!("Part2: {products}");
}
