use proconio::input;
fn main() {
    input! {
        n: u32,
        k: u32,
    }
    let mut ans = 0.0;
    for i in 1..=n {
        let mut m = 1.0 / n as f64;
        let mut x = i;
        while x < k {
            x *= 2;
            m *= 0.5;
        }
        ans += m;
    }
    println!("{}", ans);
}
