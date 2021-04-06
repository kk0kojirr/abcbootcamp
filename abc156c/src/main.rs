use proconio::input;
fn main() {
    input! {
        n: usize,
        x: [i32; n],
    }
    let mut sum: u128 = 0;
    let mut ans: u128 = std::u128::MAX;
    for p in 1..=100 {
        for v in &x {
            sum += (v - p as i32).pow(2) as u128;
        }
        ans = std::cmp::min(ans, sum);
        sum = 0;
    }
    println!("{}", ans);
}
