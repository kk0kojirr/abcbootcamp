fn main() {
    proconio::input! {
        a: u32,
        b: u32,
        c: u32,
    }
    println!("{}", c - std::cmp::min(a - b, c));
}
