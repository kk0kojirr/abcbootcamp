use proconio::input;
use std::collections::HashSet;
fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }
    let mut list = HashSet::new();
    for num in a {
        match list.contains(&num) {
            true => list.remove(&num),
            false => list.insert(num),
        };
    }
    println!("{}", list.len());
}
