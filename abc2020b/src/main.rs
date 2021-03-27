use proconio::input;
fn main() {
    input! {
        h: u64,
        w: u64,
    }
    if h == 1 || w == 1 {
        println!("1");
    } else {
        println!("{}", h * w / 2 + (h * w % 2));
    }
}
