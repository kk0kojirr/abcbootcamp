use proconio::{input, marker::Chars};
use std::collections::VecDeque;
fn main() {
    input! {
        n: usize,
        s: Chars,
        q: u32,
        tab: [(u32, usize, usize); q],
    }
    let mut ans: VecDeque<char> = VecDeque::new();
    for c in s {
        ans.push_back(c);
    }

    let mut rev = false;
    for q in tab {
        if q.0 == 1
        {
            if rev {
                let a = if q.1>n{q.1-n} else {q.1+n};
                let b = if q.2>n{q.2-n} else {q.2+n};
                ans.swap(a-1, b-1);
            } else {
                ans.swap(q.1-1, q.2-1)
            }
        }

        if q.0 == 2 {
            rev = !rev
        }
    }

    if rev {
        for i in 0..n {
            ans.swap(i, 2*n-n+i);
        }
    }

    println!("{}", ans.iter().collect::<String>());
}
