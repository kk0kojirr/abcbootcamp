use proconio::input;
fn main() {
    input! {
        width: u128,
        height: u128,
        depth: u128,
    }
    let cube: u128 = width * height * depth;
    println!("{}", cube % (1_000_000_000+7));
}
