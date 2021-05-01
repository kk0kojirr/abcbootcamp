use proconio::input;
use std::collections::HashMap;
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut student = HashMap::new();
    for i in 0..n {
        student.insert(a[i], i+1);
    }
    for i in 1..=n {
        print!("{} ", student.get(&i).unwrap());
    }
    println!("");
}
