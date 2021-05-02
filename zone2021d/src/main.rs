use proconio::input;
use std::collections::VecDeque;
fn main() {
    input! {
        s: String,
    }
    let mut t = VecDeque::new();
    let mut rev = false;
    for c in s.chars() {
        if c == 'R' {
            rev = !rev;
        } else {
            if !rev {
                t.push_back(c);
            } else {
                t.push_front(c);
            }
        }
    }

    let mut tt = VecDeque::new();
    while let Some(c) = t.pop_front() {
        if !rev {
            tt.push_back(c);
        } else {
            tt.push_front(c);
        }
    }

    let mut ans = VecDeque::new();

    while let Some(c) = tt.pop_front() {
        let prev = ans.pop_back();
        if prev == None {
            ans.push_back(c);
        } else if prev.unwrap() == c {
            continue;
        } else {
            ans.push_back(prev.unwrap());
            ans.push_back(c);
        }
    }

    println!("{}", ans.iter().collect::<String>());
}
