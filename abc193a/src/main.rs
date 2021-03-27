use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
    }
    let ans :f64 = (a * 100.0 - b * 100.0) / a;

    println!("{}", ans);
}
