use proconio::input;
fn main() {
    input! {
        mut h: u64
    }
    println!("{}", attack(h));
}

fn attack(h: u64) -> u64 {
    if h == 1 { return 1; }
    return 2 * attack(h/2) + 1;
}