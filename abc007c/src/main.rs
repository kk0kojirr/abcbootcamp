use proconio::input;
use std::collections::VecDeque;

fn main() {
    input!(
        r: usize,
        c: usize,
        s_r: usize,
        s_c: usize,
        e_r: usize,
        e_c: usize,
    );

    let mut map = Vec::new();
    for _ in 0..r {
        input!(row: String);
        map.push(row.chars().collect::<Vec<_>>());
    }

    let mut distance = vec![vec![std::usize::MAX; c]; r];
    distance[s_r - 1][s_c - 1] = 0;
    let mut queue = VecDeque::new();
    queue.push_back((s_r - 1, s_c - 1));

    while let Some((r, c)) = queue.pop_front() {
        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            let new_r = (r as i32 + dr) as usize;
            let new_c = (c as i32 + dc) as usize;

            if map[new_r][new_c] != '#' && distance[new_r][new_c] == std::usize::MAX {
                distance[new_r][new_c] = distance[r][c] + 1;
                queue.push_back((new_r, new_c));
            }
        }
    }

    println!("{}", distance[e_r - 1][e_c - 1]);
}
