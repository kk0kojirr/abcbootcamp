use proconio::input;
fn main() {
    input! {
        n: u32,
        s: String,
    }
    let mut x = 0;
    let mut ans = 0;
    for c in s.chars() {
        if c == 'I' {
            x += 1;
        }
        if c == 'D' {
            x -= 1;
        }
        ans = std::cmp::max(ans, x);
    }
    println!("{}", ans);
}
