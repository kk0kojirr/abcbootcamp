use proconio::input;
fn main() {
    input! {
        a: i64,
        b: i64,
    }
    for i in 0..=std::cmp::max(a, b) {
        if (a-i).abs() == (b-i).abs() {
            println!("{}", i);
            return;
        }
    }
    println!("{}", "IMPOSSIBLE");
}
