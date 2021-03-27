use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let mut cnt = 0;
    for i in 0..3 {
        if s[i] == t[i] { cnt += 1; }
    }
    println!("{}", cnt);
}
