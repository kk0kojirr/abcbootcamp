use proconio::input;
fn main() {
    input! {
        a: u32,
        b: u32,
    }
    let mut ans: u32 = 0;
    if a > b {
        ans = a -1;
    } else {
        ans = a;
    }
    println!("{}", ans);
}
