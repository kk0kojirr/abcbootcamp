use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        s: Chars,
    }
    let mut cnt = 0;
    let mut ans = 0;
    for c in s {
        if c == 'A' || c == 'C' || c == 'G' || c == 'T' {
            cnt += 1;
            ans = std::cmp::max(cnt, ans);
        } else {
            cnt = 0;
        }
    }
    println!("{}", ans);
}
