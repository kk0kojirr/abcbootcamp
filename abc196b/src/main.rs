use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        x: Chars,
    }
    for c in x {
        if c == '.' {
            break;
        }
        print!("{}", c);
    }
    println!("");
}
