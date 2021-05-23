use proconio::input;
fn main() {
    input! {
        l: usize,
    }
    println!("{:?}", (1..=11).fold(1, |acc, i| acc*(l-i)/i));
}
