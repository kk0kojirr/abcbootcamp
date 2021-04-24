use proconio::{input, marker::Chars};
use std::collections::VecDeque;
fn main() {
    input! {
        s: Chars,
        q: u32,
    }

    let mut ans = VecDeque::from(s);
    let mut rev = false;
    for _i in 0..q {
        input! { p: u32, }
        if p == 1 {
            rev = !rev;
        }
        if p == 2 {
            input! { ht: u32, c: char, }
            if !rev {
                if ht == 1 {
                    ans.push_front(c);
                }
                if ht == 2 {
                    ans.push_back(c);
                }
            } else {
                if ht == 1 {
                    ans.push_back(c);
                }
                if ht == 2 {
                    ans.push_front(c);
                }
            }
        }
    }
    if rev {
        println!("{}", ans.iter().rev().collect::<String>());
    } else {
        println!("{}", ans.iter().collect::<String>());
    }
}
