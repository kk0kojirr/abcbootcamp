use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        item: [usize; n],
    }

    let mut dp = vec![0; n];
    dp[0] = 0;

    for i in 0..n-1 {
        dp[i + 1] = max(dp[i], dp[i] + item[i]);
        println!("{}  {}", i, dp[i]);
    }

    println!("{}", n);
    println!("{:?}", dp);
}
