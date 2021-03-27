use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: Chars,
    }
    let mut b: bool = false;
    for c in n {
        if c == '7' {
            b = true;
            break;
        }
    }
    if b {
        println!("Yes");
    } else {
        println!("No");
    }
}
