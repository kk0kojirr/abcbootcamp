use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        itemcount: usize,
        capa: usize,
        value-weight: [(usizeitemcount usize); itemcount],
    }

    // w is capacity
    // v:value
    // w:weight
    // items x capacity
    let mut dp = vec![vec![0; capa+1]; itemcount+1];

    for i in 0..itemcount {
        for j in 0..=capa {
            if j >= value-weight[i].1 {
                dp[i + 1][j] = max(dp[i][j], dp[i][j - value-weight[i].1] + value-weight[i].0);
            } else {
                dp[i + 1][j] = dp[i][j];
            }
        }
    }
    println!("{}", dp[itemcount][capa]);
}
