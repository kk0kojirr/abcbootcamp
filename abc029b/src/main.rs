use proconio::input;
fn main() {
    input! {
        s: [String; 12],
    }
    let mut ans: u32 = 0;
    for s in s {
        for c in s.chars() {
            if c == 'r' {
                ans += 1;
                break;
            }
        }
    }
    println!("{}", ans);
}
