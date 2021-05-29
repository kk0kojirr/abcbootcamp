use proconio::input;
fn main() {
    input! {
        n: i64,
    }
    let mut evn: f32 = 0.0;
    let mut odd: f32 = 0.0;
    for i in 1..=n {
        if i % 2 == 0 {
            evn += 1.0;
        } else {
            odd += 1.0;
        }
    }
    println!("{}", odd / n as f32);
}
