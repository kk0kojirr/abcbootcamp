use std::cmp::max;
fn main() {
    proconio::input!{
        a: i32,
        b: i32,
    }
    println!("{}", max(a+b, max(a-b, a*b)));
}
