use proconio::input;
fn main() {
    input! {
        n: u32,
    }
    if n % 3 == 0 {
        println!("YES");
    } else {
        println!("NO");
    }
}
