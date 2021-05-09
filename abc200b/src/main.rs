use proconio::input;
fn main() {
    input! {
        mut n: u128,
        k: u32,
    }
    for i in 0..k {
        if n % 200 == 0 {
            n /= 200;
        } else {
            n = format!("{}{}", n, "200").parse().unwrap();
        }
    }
    println!("{}", n);
}
