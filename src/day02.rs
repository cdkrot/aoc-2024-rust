use crate::utils;

fn is_safe_level(level: &[i32]) -> bool {
    let diff: Vec<i32> = level.iter().zip(level[1..].iter()).map(|(a, b)| b - a).collect();

    let type_a: bool = diff.iter().all(|num| (1 <= *num && *num <= 3));
    let type_b: bool = diff.iter().all(|num| (-1 >= *num && *num >= -3));
    type_a || type_b
}

fn is_safe_level_vec(level: &&Vec<i32>) -> bool {
    return is_safe_level(&level[..]);
}

fn with_removed(arr: &&Vec<i32>, pos: usize) -> Vec<i32> {
    let mut arr_cloned: Vec<i32> = arr.to_vec();
    arr_cloned.remove(pos);
    arr_cloned
}

fn is_safe_up_to_one_removal(level: &&Vec<i32>) -> bool {
    if is_safe_level_vec(level) {
        return true
    }
    if is_safe_level(&level[1..]) {
        return true
    }
    if is_safe_level(&level[..level.len() - 1]) {
        return true
    }

    let total_diff = level[level.len() - 1] - level[0];
    let target_sign = if total_diff >= 0 { 1 } else {-1 };

    let diff: Vec<i32> = level.iter().zip(level[1..].iter()).map(|(a, b)| b - a).collect();
    println!("{level:?} {diff:?} {total_diff}");
    let optional_pos = diff.iter().position(|&num| if target_sign == 1 { num <= 0 } else { num >= 0 });
    let pos = match optional_pos {
        None => return false,
        Some(x) =>  x
    };

    // diff between pos & pos + 1 is problematic
    if is_safe_level_vec(&&with_removed(level, pos)) {
        return true
    }

    if is_safe_level_vec(&&with_removed(level, pos+1)) {
        return true
    }
    false
}

pub(crate) fn main() {
    let lines = utils::read_all_lines();

    let numbers: Vec<Vec<i32>> = lines.iter().map(
        |line| line.split_whitespace().map(|tok| tok.parse().unwrap()).collect()
    ).collect();

    let total = numbers.iter().filter(is_safe_level_vec).count();
    println!("Part1: {total}");

    let total_part2 = numbers.iter().filter(is_safe_up_to_one_removal).count();
    println!("Part2: {total_part2}");
}
