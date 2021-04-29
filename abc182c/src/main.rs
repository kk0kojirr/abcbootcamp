use proconio::input;
use std::collections::HashMap;
fn main() {
    input! {
        n: u128,
    }
    let mut ans = 0;
    let mut hm = HashMap::new();
    for i in 1..=n/3 {
        hm.insert(i*3u128, true);
    }

    let mut tgt = n.to_string().chars();
    for i in 0..n {
        let mut cnt = 0;
        for j in 0..n {

        }
        
    }
    if hm.contains_key(&n) {
        println!("ok");
    }
    println!("{:?}", hm);
}
