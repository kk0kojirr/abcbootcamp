use proconio::input;
fn main() {
    input! {
        a: String,
        b: String,
    }

    let ab = format!("{}{}", a, b);
    let f: f64 = ab.parse().unwrap();
    let n = f.sqrt() as i64;
    if n * n == f as i64 {
        println!("{}", "Yes");
    } else {
        println!("{}", "No");
    }
}
