use proconio::input;
fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }
    a.sort();
    let mut sums: Vec<i64> = Vec::new();
    sums.push(0);
    for _i in 1..=n {
        sums.push(sums[_i-1] + a[_i-1]);
    }
    let mut ans: i64 = 0;
    for _i in 0..n-1 {
        ans = ans + ((sums[n] - sums[_i+1]) - (n-1-_i)as i64 * a[_i]);
    }
    println!("{}", ans);
}
