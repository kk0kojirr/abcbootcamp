use proconio::input;
fn main() {
    input! { n: u64, a: u64, b: u64, }
    let mut ans = n / (a+b) * a;
    ans += std::cmp::min(a, n%(a+b));
    println!("{}", ans);
}
