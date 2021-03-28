fn main() {
    proconio::input! {
        n: usize,
        k: u32,
        x: [u32; n],
    }
    let mut ans = 0;
    for xx in x {
        ans += 2 * (k - xx).min(xx);
    }
    println!("{}", ans);
}
