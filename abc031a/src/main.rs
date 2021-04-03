use proconio::input;
use std::cmp::max;
fn main() {
    input! {
        a: u32,
        d: u32,
    }
    println!("{}", max((a + 1) * d, a * (d + 1)));
}
