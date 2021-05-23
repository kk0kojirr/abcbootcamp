use proconio::input;
use std::collections::HashSet;
fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    let mut hs = HashSet::new();
    for ss in s.iter() {
        hs.insert(ss);
    }
    for ss in s.iter() {
        let key = format!("!{}", ss);
        if hs.contains(&key) {
            println!("{}", ss);
            return;
        }
    }
    println!("satisfiable");
}
