use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        k: u32,
        s: Chars,
        t: Chars,
    }

    println!("{} {:?} {:?}", k, s, t);
}
