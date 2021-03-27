use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        s: Chars,
    }
    let mut pre = ' ';
    for c in s {
        if pre == c {
            println!("Bad");
            return;
        }
        pre = c;
    }
    println!("Good");
}
