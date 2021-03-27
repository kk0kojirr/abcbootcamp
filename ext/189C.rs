use proconio::{fastout, input};
use std::cmp;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut max: usize = 0;

    for i in 0..n {
        let mut min = a[i];
        max = cmp::max(max, a[i]);

        for j in i+1..n {
            min = cmp::min(min, a[j]);
            max = cmp::max(max, min * (j -i + 1));
        }
    }

    println!("{}", max);
}
