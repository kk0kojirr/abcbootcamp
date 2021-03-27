use proconio::input;
fn main() {
    input! {
        x: u32,
        a: u32,
    }

    if x < a {
        println!("{}", 0);
    } else {
        println!("{}", 10);
    }
}
