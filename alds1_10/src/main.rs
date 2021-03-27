use proconio::input;

fn main() {
    input! {
        n: usize,
    }


    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    dp[1] = 1;
    for i in 2..n+1 {
        dp[i] = dp[i - 1] + dp[i - 2];
    }

    println!("{}", dp[n]);
}
