use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        n: u32,
        x: u32,
        s: Chars,
    }
    let mut ans = x;
    for c in s {
        if c == 'o' {
            ans += 1;
        } else if c == 'x' && ans != 0 {
            ans -= 1;
        }
    }
    println!("{}", ans);
}
