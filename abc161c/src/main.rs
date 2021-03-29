use proconio::input;
fn main() {
    input! {
        n: i64,
        k: i64,
    }
    let n = n % k;
    let ans = std::cmp::min(k-n, n);
    println!("{}", ans);
}
