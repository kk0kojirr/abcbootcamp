use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        n: u32,
        a: u32,
        b: u32,
        s: Chars,
    }
    let mut okcnt: u32 = 0;
    let mut bcnt: u32 = 0;
    for i in 0..s.len() {
        if s[i] == 'a' {
            if okcnt < a + b {
                println!("Yes");
                okcnt += 1;
            } else {
                println!("No");
            }
            continue;
        } else if s[i] == 'b' {
            bcnt += 1;
            if okcnt < a + b && bcnt <= b {
                println!("Yes");
                okcnt += 1;
            } else {
                println!("No");
            }
            continue;
        } else {
            println!("No");
            continue;
        }
    }
}
