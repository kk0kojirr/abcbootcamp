use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
    }

    let yen = vec![1, 5, 10, 50, 100, 500, 1000, 2000, 5000, 10000];

    let mut dp = vec![std::usize::MAX; n+1];
    dp[0] = 0;
    for i in 1..n+1 {
        for j in 0..yen.len() {
            if i >= yen[j] {
                dp[i] = min(dp[i], dp[i - yen[j]] + 1)
            }
        }
    }

    println!("{:?}", dp);
}
