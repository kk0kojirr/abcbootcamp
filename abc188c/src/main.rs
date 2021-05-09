use proconio::input;
use std::collections::VecDeque;
fn main() {
    input! {
        n: usize,
        a: [u32; 1<<n],
    }
    let mut que = VecDeque::<u32>::new();
    for aa in &a {
        que.push_back(*aa);
    }

    let mut ans: u32 = 0;
    while let Some(aa) = que.pop_front() {
        let b = que.pop_front().unwrap();
        que.push_back(std::cmp::max(aa, b));

        let mut secondplace:i64 = -1;
        if que.len() == 1 {
            secondplace = std::cmp::min(aa, b) as i64;
        }

        if que.len() == 2 {
            let c = que.pop_front().unwrap();
            let d = que.pop_front().unwrap();
            secondplace = std::cmp::min(c, d) as i64;
        }

        if secondplace > -1 {
            for (idx, v) in a.into_iter().enumerate() {
                if v == secondplace as u32 {
                    ans = idx as u32+1;
                }
            }
            break;
        }
    }
    println!("{}", ans);
}
