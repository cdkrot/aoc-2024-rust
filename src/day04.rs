use crate::utils;
use crate::utils::is_inside_grid;
use crate::vec::Vec2d;

pub(crate) fn main() {
    let lines: Vec<Vec<u8>> = utils::read_all_lines()
        .iter()
        .map(|l| l.trim().as_bytes().to_vec())
        .collect();

    let n = lines.len() as i32;
    let m = lines[0].len() as i32;

    let dirs: Vec<Vec2d> = (-1..2)
        .flat_map(|i| (-1..2).map(move |j| Vec2d::new(i, j)))
        .collect();

    let get = |pt: Vec2d| -> Option<u8> {
        if !is_inside_grid(n, m, pt) {
            None
        } else {
            Some(lines[pt.x as usize][pt.y as usize])
        }
    };

    let mut cnt = 0;
    for i in 0..n {
        for j in 0..m {
            for &dir in &dirs {
                let pt = Vec2d::new(i, j);
                let shifted = |steps: i32| get(pt + dir.scale(steps));

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

    let dirs = vec![
        Vec2d::new(-1, -1),
        Vec2d::new(-1, 1),
        Vec2d::new(1, 1),
        Vec2d::new(1, -1),
    ];

    let mut cnt_cross = 0;
    for i in 1..(n as i32 - 1) {
        for j in 1..(m as i32 - 1) {
            let pt = Vec2d::new(i, j);
            let all: Vec<u8> = dirs.iter().map(|&d| get(pt + d).unwrap()).collect();
            let cnt_m = all.iter().filter(|&&val| val == ('M' as u8)).count();
            let cnt_s = all.iter().filter(|&&val| val == ('S' as u8)).count();

            if cnt_m == 2 && cnt_s == 2 && get(pt) == Some('A' as u8) && all[0] != all[2] {
                cnt_cross += 1;
            }
        }
    }
    println!("Part2 total: {cnt_cross}");
}
