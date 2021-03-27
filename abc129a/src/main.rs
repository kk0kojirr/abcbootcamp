use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        p: u32,
        q: u32,
        r: u32,
    }
    println!("{}", min(p+q, min(q+r, r+p)));
}
