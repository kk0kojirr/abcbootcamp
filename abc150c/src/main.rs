use proconio::input;
use itertools::Itertools;
fn main() {
    input! {
        n: usize,
        p: [usize; n],
        q: [usize; n],
    }
    let perms = (1..=n).permutations(n);
    let mut i: i32 = 1;
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    for pp in perms {
        if pp == p {
            a = i;
        }
        if pp == q {
            b = i;
        }
        i += 1;
    }
    println!("{}", (a - b).abs());
}
