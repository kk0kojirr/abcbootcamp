use proconio::input;
use std::cmp::max;
fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
        k: u32,
    }
    let mut big: u32 = 0;
    let mut ota: u32 = 0;
    let mut otb: u32 = 0;
    if a > b && a > c {
        big = a;
        ota = b;
        otb = c;
    } else if b > a && b > c {
        big = b;
        ota = a;
        otb = c;
    } else if c > a && c > b {
        big = c;
        ota = a;
        otb = b;
    } else {
        big = a;
        ota = b;
        otb = c;
    }
    for _i in 1..=k {
        big *= 2;
    }
    println!("{}", big + ota + otb);
}
