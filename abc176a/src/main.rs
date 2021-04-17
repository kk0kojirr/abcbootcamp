use proconio::input;
fn main() {
    input! {
        n: u32,
        x: u32,
        t: u32,
    }
    println!("{}", if n % x == 0 {n/x*t} else {n/x*t+t});
}
