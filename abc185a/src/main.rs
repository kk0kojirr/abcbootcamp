use proconio::input;
use std::cmp::min;
fn main() {
    input! {
        a: [u32; 4],
    }
    println!("{}", min(min(min(a[0], a[1]),a[2]),a[3]));
}
