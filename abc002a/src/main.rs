use proconio::input;

fn main() {
    input! {
        x: u32,
        y: u32,
    }
    if (x > y) {
        println!("{}", x);
    } else {
        println!("{}", y);
    }
}
