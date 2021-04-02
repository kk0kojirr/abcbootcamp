use proconio::input;
fn main() {
    input! {
        n: usize,
        h: [u32; n],
    }
    let mut prev: u32 = 0;
    let mut ans: u32 = 0;
    for i in h {
        if prev <= i {
            ans += 1;
        }
        prev = std::cmp::max(i, prev);
    }
    println!("{}", ans);
}
