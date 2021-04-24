use proconio::input;
fn main() {
    input! {
        n: u32,
    }
    let mut ans: u32 = 0;
    for i in 1..=n{
        ans += 10000 * i;
    }
    println!("{}", ans / n);
}
