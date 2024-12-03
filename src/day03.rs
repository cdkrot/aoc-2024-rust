use crate::utils;
use regex::Regex;

fn process_multiplications(text: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut results = vec![];
    for (_, [lhs, rhs]) in re.captures_iter(text).map(|c| c.extract::<2>()) {
        results.push((lhs.parse::<i32>().unwrap(), rhs.parse::<i32>().unwrap()));
    }

    results.iter().map(|(x, y)| x * y).sum::<i32>()
}

pub(crate) fn main() {
    let lines = utils::read_all_lines();
    let mut text = lines.join("\n");

    println!("Part 1: {}", process_multiplications(text.as_str()));

    text = String::from("do()") + text.as_str();
    let re = Regex::new(r"(do|don't)\(\)").unwrap();
    let mut res: Vec<(usize, bool)> = re
        .captures_iter(text.as_str())
        .map(|c| (c.get(0).unwrap().start(), c.get(0).unwrap().as_str() == "do()"))
        .collect();

    res.push((text.len(), false));

    let total = res.iter().zip(res.iter().skip(1)).map(|(x, y)|
        if x.1 {process_multiplications(&text[x.0..y.0])} else {0}
    ).sum::<i32>();

    println!("Part 2: {}", total);
}
