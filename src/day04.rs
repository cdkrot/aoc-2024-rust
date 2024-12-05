use crate::utils;

pub(crate) fn main() {
    let lines: Vec<Vec<u8>> = utils::read_all_lines()
        .iter()
        .map(|l| l.trim().as_bytes().to_vec())
        .collect();

    let n = lines.len() as i32;
    let m = lines[0].len() as i32;

    let dirs: Vec<(i32, i32)> = (-1..2).flat_map(|i| (-1..2).map(move |j| (i, j))).collect();

    let get = |i: i32, j: i32| -> Option<u8> {
        if i < 0 || j < 0 || i >= n || j >= m {
            None
        } else {
            Some(lines[i as usize][j as usize])
        }
    };

    let mut cnt = 0;
    for i in 0..n {
        for j in 0..m {
            for &dir in &dirs {
                let shifted = |steps: i32| get(i + steps * dir.0, j + steps * dir.1);

                if shifted(0) == Some('X' as u8)
                    && shifted(1) == Some('M' as u8)
                    && shifted(2) == Some('A' as u8)
                    && shifted(3) == Some('S' as u8)
                {
                    cnt += 1
                }
            }
        }
    }
    println!("Part1 total: {cnt}");

    let mut cnt_cross = 0;
    for i in 1..(n as i32 - 1) {
        for j in 1..(m as i32 - 1) {
            let a = get(i - 1, j - 1).unwrap();
            let b = get(i - 1, j + 1).unwrap();
            let c = get(i + 1, j + 1).unwrap();
            let d = get(i + 1, j - 1).unwrap();

            let all = vec![a, b, c, d];
            let cnt_m = all.iter().filter(|&&val| val == ('M' as u8)).count();
            let cnt_s = all.iter().filter(|&&val| val == ('S' as u8)).count();

            if cnt_m == 2
                && cnt_s == 2
                && get(i, j) == Some('A' as u8)
                && get(i - 1, j - 1) != get(i + 1, j + 1)
            {
                cnt_cross += 1;
            }
        }
    }
    println!("Part2 total: {cnt_cross}");
}
