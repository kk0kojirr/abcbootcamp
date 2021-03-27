use proconio::input;
use proconio::marker::Chars;
use std::cmp::max;
fn main() {
    input! {
        a: Chars,
        b: Chars,
    }
    let suma: u8 = a.iter().map(|c| *c as u8 -b'0').sum();
    let sumb: u8 = b.iter().map(|c| *c as u8 -b'0').sum();
    println!("{}", max(suma, sumb));
}
