use proconio::input;
use std::cmp::min;
fn main() {
    input! {
        n: u8,
        a: u8,
        b: u8,
    }
    println!("{}", min(n * a, b));
}
