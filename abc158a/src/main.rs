use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        s: Chars,
    }
    if s[0] == s[1] && s [1] == s[2] {
        println!("No");
    } else {
        println!("Yes");
    }
}
